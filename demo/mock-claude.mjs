// Mock Claude — fake Claude Code session for the Taurus demo video.
// Shows a fresh "just started" welcome screen and replies with FICTIONAL data only.
// No real claude, no MCP, no real files. cwd is set by Taurus to the project folder.
import readline from "node:readline";
import path from "node:path";
import process from "node:process";

const O = "\x1b[38;2;217;119;87m"; // Claude orange
const R = "\x1b[0m", DIM = "\x1b[2m", B = "\x1b[1m", CY = "\x1b[36m";
const cwd = process.cwd();
const project = path.basename(cwd);
const dash = path.join(cwd, "dashboard.html");
const W = 60;
const out = (s) => process.stdout.write(s);
const pad = (s) => {
  // visible length ignoring ANSI
  const vis = s.replace(/\x1b\[[0-9;]*m/g, "").length;
  return s + " ".repeat(Math.max(0, W - vis));
};
function box(lines) {
  out(`${O}╭${"─".repeat(W)}╮${R}\n`);
  for (const l of lines) out(`${O}│${R}${pad(" " + l)}${O}│${R}\n`);
  out(`${O}╰${"─".repeat(W)}╯${R}\n`);
}

out("\x1b[2J\x1b[H"); // clear
box([
  `${O}✻${R} ${B}Welcome to Claude Code${R}`,
  "",
  `${DIM}/help for help, /status for your current setup${R}`,
  "",
  `${DIM}cwd: ${cwd}${R}`,
]);
out(`\n ${DIM}Tip: type a question and press Enter — this is a ${B}demo${R}${DIM} (fictional data).${R}\n`);

const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
const promptStr = `\n${O}>${R} `;
const prompt = () => out(promptStr);

const verbs = ["Pondering", "Orbiting", "Spelunking", "Marinating", "Scheming"];
let vi = 0;
const glyphs = ["✶", "✻", "✽", "✳"];

function think(ms) {
  return new Promise((resolve) => {
    rl.pause();
    const verb = verbs[vi++ % verbs.length];
    let g = 0;
    const t0 = Date.now();
    const iv = setInterval(() => {
      out(`\r${O}${glyphs[g++ % glyphs.length]}${R} ${verb}… ${DIM}(${((Date.now() - t0) / 1000).toFixed(0)}s)${R}   `);
    }, 180);
    setTimeout(() => {
      clearInterval(iv);
      out("\r\x1b[K"); // clear spinner line
      rl.resume();
      resolve();
    }, ms);
  });
}

function reply(q) {
  if (/porter/i.test(project)) {
    return (
      `${B}Mail-triage (voorbeeld)${R}\n` +
      `Ik heb je voorbeeld-inbox bekeken en gecategoriseerd:\n\n` +
      `  • ${CY}4${R} actie   • ${CY}7${R} info   • ${CY}12${R} laag   • ${CY}23${R} afgehandeld\n\n` +
      `Belangrijkste actie: ${DIM}"Ticket DEMO-123 bijgewerkt"${R} — opvolgen.\n` +
      `Overzicht weggeschreven naar:\n  ${dash}\n`
    );
  }
  return (
    `${B}Projectoverzicht (voorbeeld)${R}\n` +
    `Plan voor een voorbeeldproject in 3 stappen:\n\n` +
    `  1. Voorbeeldproject Alfa — opzet (DOING)\n` +
    `  2. Demo-migratie Beta — voorbereiden (WACHT)\n` +
    `  3. Test-uitrol Delta — uitvoeren\n\n` +
    `Dashboard met de kaarten staat klaar op:\n  ${dash}\n`
  );
}

rl.on("line", async (line) => {
  const q = line.trim();
  if (q === "/exit" || q === "exit" || q === "/quit") { rl.close(); return; }
  if (!q) { prompt(); return; }
  await think(1900);
  out("\n" + reply(q) + "\n");
  prompt();
});
rl.on("close", () => { out(`\n${DIM}Tot ziens (demo).${R}\n`); process.exit(0); });
prompt();
