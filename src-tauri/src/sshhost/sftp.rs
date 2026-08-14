// SFTP-subsysteem voor de spike. Nodig omdat de DROPZONE met scp.exe werkt, en
// scp sinds OpenSSH 9 standaard over SFTP loopt -- zonder dit kan Taurus wel
// verbinden maar geen bestand versturen.
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use russh_sftp::protocol::{
    Attrs, Data, File, FileAttributes, Handle, Name, OpenFlags, Status, StatusCode, Version,
};

#[derive(Default)]
pub struct SftpSession {
    files: HashMap<String, fs::File>,
    // Per open map: de nog niet uitgeleverde regels. readdir wordt herhaald
    // aangeroepen tot je Eof teruggeeft.
    dirs: HashMap<String, Vec<File>>,
    next: u64,
}

fn io_err(e: &std::io::Error) -> StatusCode {
    match e.kind() {
        std::io::ErrorKind::NotFound => StatusCode::NoSuchFile,
        std::io::ErrorKind::PermissionDenied => StatusCode::PermissionDenied,
        _ => StatusCode::Failure,
    }
}

fn ok(id: u32) -> Status {
    Status { id, status_code: StatusCode::Ok, error_message: "Ok".into(), language_tag: "en-US".into() }
}

// De client stuurt Windows-paden ("C:\werk\input"), maar ook POSIX-vormen die
// sftp zelf maakt ("/C:/werk"). Allebei moeten hier op hetzelfde pad uitkomen.
fn to_path(p: &str) -> PathBuf {
    let s = p.replace('\\', "/");
    let s = s.strip_prefix('/').filter(|r| r.chars().nth(1) == Some(':')).unwrap_or(&s);
    if s.is_empty() || s == "." {
        return PathBuf::from(std::env::var("USERPROFILE").unwrap_or_else(|_| ".".into()));
    }
    PathBuf::from(s)
}

fn attrs_of(md: &fs::Metadata) -> FileAttributes {
    let mut a = FileAttributes::from(md);
    // Zonder dit ziet de client een map als een bestand en gaat scp -r de mist in.
    a.set_dir(md.is_dir());
    a.set_regular(md.is_file());
    a
}

impl SftpSession {
    fn handle(&mut self, prefix: &str) -> String {
        self.next += 1;
        format!("{prefix}{}", self.next)
    }
}

#[cfg(test)]
mod tests {
    use super::to_path;

    // GEMETEN: scp stuurt Windows-paden door als "C:/werk/input", en de
    // sftp-laag maakt daar soms "/C:/werk/input" van. Allebei moeten op
    // hetzelfde pad uitkomen, anders belandt een DROPZONE-bestand nergens.
    #[test]
    fn windows_paths_arrive_in_three_shapes() {
        let want = std::path::PathBuf::from("C:/werk/input");
        assert_eq!(to_path("C:/werk/input"), want);
        assert_eq!(to_path("/C:/werk/input"), want);
        assert_eq!(to_path(r"C:\werk\input"), want);
    }

    // Een leeg pad of "." is de thuismap, niet de werkmap van het proces: een
    // client die realpath(".") vraagt hoort daar te landen.
    #[test]
    fn dot_is_the_home_directory() {
        let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".into());
        assert_eq!(to_path("."), std::path::PathBuf::from(&home));
        assert_eq!(to_path(""), std::path::PathBuf::from(&home));
    }

    // Een POSIX-pad zonder stationsletter mag NIET zijn leidende slash kwijt --
    // dan zou /tmp/x stiekem relatief worden.
    #[test]
    fn posix_paths_keep_their_root() {
        assert_eq!(to_path("/tmp/x"), std::path::PathBuf::from("/tmp/x"));
    }
}

impl russh_sftp::server::Handler for SftpSession {
    type Error = StatusCode;

    fn unimplemented(&self) -> Self::Error {
        StatusCode::OpUnsupported
    }

    async fn init(
        &mut self,
        _version: u32,
        _ext: HashMap<String, String>,
    ) -> Result<Version, Self::Error> {
        Ok(Version::new())
    }

    async fn realpath(&mut self, id: u32, path: String) -> Result<Name, Self::Error> {
        let p = to_path(&path);
        // canonicalize geeft op Windows een \\?\-pad terug; dat verwart de client.
        let resolved = fs::canonicalize(&p)
            .map(|c| c.to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/"))
            .unwrap_or_else(|_| p.to_string_lossy().replace('\\', "/"));
        println!("[sftp] realpath {path} -> {resolved}");
        Ok(Name { id, files: vec![File::new(resolved, FileAttributes::default())] })
    }

    async fn stat(&mut self, id: u32, path: String) -> Result<Attrs, Self::Error> {
        let md = fs::metadata(to_path(&path)).map_err(|e| io_err(&e))?;
        Ok(Attrs { id, attrs: attrs_of(&md) })
    }

    async fn lstat(&mut self, id: u32, path: String) -> Result<Attrs, Self::Error> {
        let md = fs::symlink_metadata(to_path(&path)).map_err(|e| io_err(&e))?;
        Ok(Attrs { id, attrs: attrs_of(&md) })
    }

    async fn fstat(&mut self, id: u32, handle: String) -> Result<Attrs, Self::Error> {
        let f = self.files.get(&handle).ok_or(StatusCode::Failure)?;
        let md = f.metadata().map_err(|e| io_err(&e))?;
        Ok(Attrs { id, attrs: attrs_of(&md) })
    }

    async fn open(
        &mut self,
        id: u32,
        filename: String,
        pflags: OpenFlags,
        _attrs: FileAttributes,
    ) -> Result<Handle, Self::Error> {
        let mut o = fs::OpenOptions::new();
        o.read(pflags.contains(OpenFlags::READ))
            .write(pflags.contains(OpenFlags::WRITE))
            .append(pflags.contains(OpenFlags::APPEND))
            .create(pflags.contains(OpenFlags::CREATE))
            .truncate(pflags.contains(OpenFlags::TRUNCATE));
        let f = o.open(to_path(&filename)).map_err(|e| {
            println!("[sftp] open {filename} mislukt: {e}");
            io_err(&e)
        })?;
        let h = self.handle("f");
        println!("[sftp] open {filename} flags={pflags:?} -> {h}");
        self.files.insert(h.clone(), f);
        Ok(Handle { id, handle: h })
    }

    async fn read(&mut self, id: u32, handle: String, offset: u64, len: u32) -> Result<Data, Self::Error> {
        let f = self.files.get_mut(&handle).ok_or(StatusCode::Failure)?;
        f.seek(SeekFrom::Start(offset)).map_err(|e| io_err(&e))?;
        let mut buf = vec![0u8; len as usize];
        let n = f.read(&mut buf).map_err(|e| io_err(&e))?;
        if n == 0 {
            return Err(StatusCode::Eof);
        }
        buf.truncate(n);
        Ok(Data { id, data: buf })
    }

    async fn write(&mut self, id: u32, handle: String, offset: u64, data: Vec<u8>) -> Result<Status, Self::Error> {
        let f = self.files.get_mut(&handle).ok_or(StatusCode::Failure)?;
        f.seek(SeekFrom::Start(offset)).map_err(|e| io_err(&e))?;
        f.write_all(&data).map_err(|e| io_err(&e))?;
        Ok(ok(id))
    }

    async fn close(&mut self, id: u32, handle: String) -> Result<Status, Self::Error> {
        self.files.remove(&handle);
        self.dirs.remove(&handle);
        Ok(ok(id))
    }

    async fn opendir(&mut self, id: u32, path: String) -> Result<Handle, Self::Error> {
        let dir = to_path(&path);
        let mut files = Vec::new();
        for e in fs::read_dir(&dir).map_err(|e| io_err(&e))? {
            let e = e.map_err(|err| io_err(&err))?;
            let md = e.metadata().map_err(|err| io_err(&err))?;
            files.push(File::new(e.file_name().to_string_lossy().to_string(), attrs_of(&md)));
        }
        let h = self.handle("d");
        self.dirs.insert(h.clone(), files);
        Ok(Handle { id, handle: h })
    }

    async fn readdir(&mut self, id: u32, handle: String) -> Result<Name, Self::Error> {
        // Alles in een keer, daarna Eof: de client blijft anders eeuwig vragen.
        match self.dirs.get_mut(&handle) {
            Some(v) if !v.is_empty() => Ok(Name { id, files: std::mem::take(v) }),
            _ => Err(StatusCode::Eof),
        }
    }

    async fn mkdir(&mut self, id: u32, path: String, _attrs: FileAttributes) -> Result<Status, Self::Error> {
        match fs::create_dir_all(to_path(&path)) {
            Ok(_) => Ok(ok(id)),
            Err(e) => Err(io_err(&e)),
        }
    }

    async fn remove(&mut self, id: u32, filename: String) -> Result<Status, Self::Error> {
        fs::remove_file(to_path(&filename)).map_err(|e| io_err(&e))?;
        Ok(ok(id))
    }

    async fn rmdir(&mut self, id: u32, path: String) -> Result<Status, Self::Error> {
        fs::remove_dir_all(to_path(&path)).map_err(|e| io_err(&e))?;
        Ok(ok(id))
    }

    async fn rename(&mut self, id: u32, oldpath: String, newpath: String) -> Result<Status, Self::Error> {
        fs::rename(to_path(&oldpath), to_path(&newpath)).map_err(|e| io_err(&e))?;
        Ok(ok(id))
    }

    // Rechten/tijden zijn op Windows niet zinvol over te nemen. Ok teruggeven en
    // niets doen: een fout hier laat scp de hele overdracht als mislukt melden.
    async fn setstat(&mut self, id: u32, _path: String, _attrs: FileAttributes) -> Result<Status, Self::Error> {
        Ok(ok(id))
    }

    async fn fsetstat(&mut self, id: u32, _handle: String, _attrs: FileAttributes) -> Result<Status, Self::Error> {
        Ok(ok(id))
    }
}
