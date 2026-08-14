# Testclient starten — PR #127 (#124, #125, #126, #128, #129)

Build van **14 augustus 2026 23:27**, SHA-256 `7533ABE8C4914292F2793CC7F0DFA5CC94E913C593A238A35EB7C18AB225C037`.

## Waar het staat

| wat | pad |
|---|---|
| startscript | `C:\Users\AST\claude\Taurus\start-taurus-test.ps1` |
| binary | `C:\Users\AST\claude\Taurus\src-tauri\target\release\taurus.exe` |
| config testclient | `C:\Users\AST\AppData\Roaming\Taurus-TEST` |
| jouw draaiende Taurus | `C:\Tools\Taurus\taurus.exe`, config `%APPDATA%\Taurus` |

## Starten

Sluit eerst een eventueel lopend TEST-venster (die houdt zijn eigen exe vergrendeld), dan:

```powershell
pwsh -NoProfile -File C:\Users\AST\claude\Taurus\start-taurus-test.ps1
```

Het script verlegt `TAURUS_CONFIG_DIR` naar `Taurus-TEST`, dus eigen projects/hosts/
sessions/peers/history. Jouw echte exemplaar merkt er niets van. Titelbalk zegt **TEST**.

Draait er al een testexemplaar, dan weigert het script — twee daarvan zouden dezelfde
configmap delen. Wil je een build van een ander pad starten: `-Exe <volledig pad>`.

**Botst met je echte Taurus** als je het in het testexemplaar aanzet: poort 8287 en
UDP 5353. Eén van de twee tegelijk "bereikbaar".

---

## Wat er te zien is

### Opstarten vraagt nu (#129)

De testconfig is leeg, dus de eerste keer zie je niets. Start een agent, sluit Taurus,
start opnieuw: nu komt **"Vorige sessies openen?"** met wat openstond voorgevinkt.
Escape of "Niets openen" verliest niets — de geschiedenis blijft.

Onder Instellingen → Sessies staat nu een keuzelijst in plaats van een vinkje: *vragen* /
*stil hervatten* / *schoon beginnen*.

### ⇱ in de zijbalk — verder werken

Twee koppen: **op deze computer** (je geschiedenis, minus wat al open is) en **op jouw
machines** (de agents per ingerichte machine). De ondertitel zegt expliciet dat het over
jouw eigen spullen gaat.

### 🖥 machinescherm

- Eén blok per machine. Je `hosts.json` heeft `ursu` (:22) en `ursu-wsl` (:2223) op
  hetzelfde adres — dat hoort één regel **ursu** te zijn met twee routes eronder, elk met
  eigen bolletje en het OS erbij.
- **Nieuwe agent…** opent het gewone startformulier met de map van die machine ingevuld.
  (Heette Connect; die startte iets naamloos in `C:\Users\arjen`.)
- **⇱ agents** toont welke agents daar draaien — niet de mux-sessies. Draait er niets, dan
  staat er dat er niets is om mee te verbinden. Lege sessies staan eronder als opruimwerk
  met een knop, niet als keuze.
  *Op ursu staat nog `taurus-ursu-c-users-arjen-28f85e64` (running, geen agent) — die zou
  daar nu als opruimwerk moeten staan, en de knop is meteen een test van `session delete`.*

### ✋ vraagmodus (#125)

1. Zet aan beide kanten "bereikbaar" aan op een vertrouwd netwerk (Instellingen → Netwerk)
   en laat de firewall-regels aanmaken als het scherm daarom vraagt.
2. Op de ene machine: rechtsklik op een tab van een **lokale** sessie → *✋ Vraag om hulp
   bij deze agent*. Er verschijnt een balk onder de tabbalk met *Intrekken*.
3. Op de andere: machinescherm open → bovenaan **Iemand vraagt hulp** met de naam van die
   agent → **Meedoen**.
4. Je landt in dezelfde terminal. Typ aan beide kanten; het hoort door elkaar te lopen.
5. Er komt géén pairing- of sessiepopup bij de vrager: hij heeft er zelf om gevraagd.
6. Trek de vraag in (of laat iemand meedoen) en de regel verdwijnt bij de ander.

In het audit-log van de vragende machine horen `help-auth` en `help-answered` te staan.

### Toestemmingspopup (#126)

Bij een gewone inkomende sessie staat er een vinkje *"Vol beheer: de agent zonder rem"*.
Zonder vinkje start de agent in ask-modus, met vinkje in zijn eigen modus. Er start in
geen van beide gevallen een kale shell.

---

## Ursu

Nieuwe build klaargezet als `C:\Tools\Taurus\taurus-nieuw.exe` (zelfde hash als hierboven).
**Jij draait daar** in je eigen sessie (vanuit SSH start hij niet op je scherm):

```powershell
C:\Tools\Taurus\update-taurus.ps1
```

Stand daar bij het laatste kijkje: `enabled: true` op poort 8287, één netwerk vertrouwd,
`peers.json` weg (dus eerste keer nog een pairing-popup bij een *gewone* sessie), en zowel
`Taurus` als `Taurus mDNS` in de firewall aanwezig zonder block-regels.

## Wat ik níét gedraaid heb

```powershell
cd C:\Users\AST\claude\Taurus\src-tauri
cargo test --lib -- --ignored --nocapture announce_and_find
```

Kondigt echt aan en zoekt echt in één proces, en controleert dat één machine één regel
oplevert. Hij **bindt UDP 5353** — tijdens de spike blokkeerde Defender precies dat en
moest de regel met de hand weg. Daarom `#[ignore]`.

Deze bindt niets en laat zien welk adres de aankondiging zou dragen:

```powershell
cargo test --lib -- --ignored --nocapture toon_trusted_ip
```
