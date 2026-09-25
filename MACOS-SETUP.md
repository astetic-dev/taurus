# Setting up a Mac to build Taurus

For a fresh Mac (Apple Silicon), straight out of the box. Follow the steps in order; each one needs the previous. Run the commands in **Terminal** (Applications > Utilities). The port itself is tracked in #199.

## 1. macOS

- [ ] Finish the first-run setup, sign in with your Apple ID and install all updates (System Settings > General > Software Update).

## 2. Xcode Command Line Tools

Provides git, the C compiler and the linker that Rust and Tauri need. Full Xcode is not required.

```
xcode-select --install
```

## 3. Homebrew

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

- [ ] Run the two lines the installer prints at the end (`echo ... >> ~/.zprofile` and `eval ...`). They put `brew` on your PATH.

## 4. Tools

```
brew install node gh git
```

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- [ ] Rustup: choose the default installation, then open a **new** Terminal window so `cargo` is on the PATH.
- [ ] Check: `cargo --version`, `node --version` and `gh --version` each print a version.

## 5. Claude Code

```
curl -fsSL https://claude.ai/install.sh | bash
```

- [ ] Open a new window, run `claude` and sign in.

## 6. GitHub and the repo

```
gh auth login
```

- [ ] Choose GitHub.com, HTTPS, log in with the browser, as **astetic-dev**.

```
git config --global user.name "astetic-dev"
```

```
git config --global user.email "arjenstet@gmail.com"
```

```
mkdir -p ~/dev && cd ~/dev && gh repo clone astetic-dev/taurus && cd taurus && npm install
```

## 7. First build

```
npx tauri dev
```

- [ ] The first run takes a while, because every crate is compiled. Expect compile or runtime errors: that is where the port starts.

## 8. Start the agent

Start `claude` in `~/dev/taurus` and give it #199 as its first task, starting with step 1: make `npx tauri dev` compile and start. `CLAUDE.md` and `PITFALLS.md` in the repo carry the working agreements and the measured pitfalls, so the session on the Mac starts with them.
