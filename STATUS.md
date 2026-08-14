# Waar het staat — 15 augustus 2026

**0.5.3 is uit.** `main` staat op `66e06ca`; PR #127 (vijf issues) en PR #131 (de
release) zijn gemerged. Er staat geen werk meer open op een branch.

| onderdeel | stand |
|---|---|
| #121 SSH-host | volledig bewezen, spiegel-tab in beide richtingen |
| #124 machinescherm | uitgeleverd in 0.5.3 |
| #125 vraagmodus | uitgeleverd in 0.5.3 |
| #126 macht volgt toezicht | uitgeleverd in 0.5.3, onbeheerd = `dontAsk` |
| #128 agents i.p.v. shells | uitgeleverd in 0.5.3 |
| #129 sessiegeschiedenis | uitgeleverd in 0.5.3 |
| #130 permissiemodi | uitgeleverd in 0.5.3, alle zes |
| tests | 95 groen, 10 ignored |
| release-build | 15 aug 01:08, ook op ursu klaargezet (`188D6029…`) |
| versie | **0.5.3** |
| nog te doen | ursu bijwerken (`update-taurus.ps1`), en een ronde over de echte LAN |

---

## De regel waar alles uit volgt

**Taurus toont agents.** ssh, tmux en herdr maken de weg vrij zodat er een agent kan
starten — ze zijn leidingwerk en nooit iets wat je kiest. De route mag je zien als detail
op de machineregel. **Geen agent = niets om mee te verbinden**: geen keuze met een
waarschuwingslabel, maar geen keuze.

**De as is eigenaarschap.** Onbeheerd starten op je eigen machines en alleen meekijken bij
een collega botsen niet; het is één regel met twee kanten.

| | jouw machines | machine van een collega |
|---|---|---|
| gevonden via | `hosts.json`, jij richt in | mDNS, alleen zolang hij vraagt |
| wat mag | agent starten, onbeheerd; terug naar wat draait | alleen meedoen aan de agent die hij aanbiedt |
| toestemming | eigen computer, één keer koppelen | zijn uitnodiging ís de toestemming |
| overnemen | ja | **nee** — blijft zijn sessie |

Volledige notitie met beelden: `DESIGN-vloot-en-vraagmodus.md` /
https://claude.ai/code/artifact/8e9deab1-8aba-4421-b4a9-4509d554d4ff

---

## Wat er gebouwd is

### #124 — één scherm voor je eigen machines

Eén regel per computer, routes klein eronder met eigen bolletje, Taurus-route heeft
voorkeur. Routes naar hetzelfde adres vallen vanzelf samen; bestaande `hosts.json` hoeft
niet herschreven. De routenaam is weg uit tab-badges, tooltips, hover-recap, agentkaarten
en de dropdowns. **Connect werd "Nieuwe agent…"** — die startte iets naamloos in een map
die je niet koos, en dat is precies hoe je in `C:\Users\arjen` belandde.

### #128 — agents, geen shells of mux-sessies

`remote_agents()` voegt twee bronnen samen: herdr voor wat Taurus daar over ssh startte,
en de `sessions.json` van de Taurus dáár voor wat er in zijn eigen tabs draait. Voor wie
kijkt is dat hetzelfde ding. `collect_agents()` is puur, met de echte data van ursu als
fixture. Lege sessies staan onder de keuzes als opruimwerk. Een agent die in die andere
Taurus leeft wordt getoond maar niet aangeboden — er is geen kanaal, en de regel zegt dat.

### #129 — een geschiedenis die een mislukte restore overleeft

`history.json` is een tweede bestand met een andere regel: er wordt aan toegevoegd en
bijgewerkt, en er verdwijnt nooit iets omdat een herstart niet lukte. Opstarten **vraagt**:
wat openstond voorgevinkt, de rest eronder, en wat niet kan hervatten houdt zijn regel mét
reden. De instelling is nu ask / silent / clean; clean wist alleen de open-lijst.

### De ⇱-knop

Blijft, en zegt over wiens machines het gaat: **op deze computer** eerst (de geschiedenis
uit #129, op elk moment bereikbaar — wat dat issue letterlijk vraagt), daarna **op jouw
machines** met de agents uit #128.

### #125 — vraagmodus

Rechtsklik op een tab → *Vraag om hulp*. De aankondiging noemt één agent en draagt een
eenmalig token; ze bestaat alleen zolang de hand omhoog is. De helper verbindt met dat
token als ssh-gebruikersnaam, en de vragende kant laat hem in de auth-fase binnen **zonder
pairing-popup** — degene die die popup zou moeten beantwoorden is juist degene die het
vroeg. Die acceptatie is smal: met een hulptoken kan de verbinding alleen meelezen met de
aangeboden sessie; al het andere wordt geweigerd en in het audit-spoor gezet.

**Meedoen start niets.** `start_pty` heeft een fan-out gekregen, zodat de bytes van een
lokale sessie naar een tweede lezer kunnen, en wat de helper typt gaat diezelfde lokale pty
in. Twee toetsenborden op één agent; het werk blijft waar het probleem zit.

### #126 — macht volgt toezicht

Ongewijzigd van bedoeling, gecorrigeerd in aanname: Taurus start agents, geen shells.
Onbeheerd krijgt `--permission-mode dontAsk` (vraagt niets, weigert wat niet vooraf mag),
meegekeken de eigen modus van de agent.

---

## Wat er tegen echte machines is gemeten

Drie dingen zijn niet alleen unit-getoetst maar echt gedraaid, en alle drie zijn
herhaalbaar:

| commando | uitkomst |
|---|---|
| `cargo test --lib -- --ignored --nocapture announce_and_find` | aankondigen + vinden over een echte UDP 5353-socket: één rij, met agentnaam, map en token |
| `TAURUS_TEST_HOST=ursu cargo test --lib -- --ignored --nocapture toon_agents_op` | op ursu: **1 agent** (Ontwikkel, uit de Taurus dáár, terecht niet-aanhaakbaar) en **2 lege sessies** als opruimwerk |
| `TAURUS_TEST_EXE=<pad> cargo test --lib -- --ignored --nocapture toon_firewall` | voor de release-exe: TCP en UDP toegestaan, nul blokkades |

Die tweede is precies de omkering waar deze tak over gaat: die twee lege sessies waren
eerst het enige wat het scherm aanbood, en die ene echte agent was onzichtbaar.

De mDNS-test faalde de eerste keer, en terecht: hij kondigde nog zonder token aan, en dat
filtert `absorb` weg omdat een aankondiging zonder token aanwezigheid is en geen vraag.
Het filter werkte dus; de test was achterhaald.

---

## De lus is gelopen — tussen twee echte instanties

Twee Taurus-processen naast elkaar op dit werkstation, elk met een eigen configmap en
webview-profiel, aangestuurd via CDP (`--remote-debugging-port`). A vraagt, B helpt.

| stap | uitkomst |
|---|---|
| A steekt de hand op | verzoek met token, `help_asking` bevestigt |
| B zoekt over mDNS | **binnen 0,5 s** zichtbaar, met agentnaam, map, fingerprint en token, op 192.168.225.44 |
| B antwoordt | `help-auth` en `help-answered` in A's audit |
| A typt `echo GROET-VAN-A` | B ontvangt exact dezelfde bytes, inclusief de uitvoer |
| B typt `echo TYPT-DE-HELPER` | verschijnt in A's terminal, mét uitvoer |
| na het antwoorden | vraag dicht: `help_asking` is null, B's lijst leeg |
| token nog eens gebruiken | geweigerd |
| verzonnen token | geweigerd |
| `whoami` op een geldig token | geweigerd, geaudit als `help-refused`, token blijft ongebruikt |

En het laatste openstaande punt van **#121** meteen erbij: B start een gewone sessie op A,
A krijgt de pairing- en daarna de sessie-popup, antwoord *join* → A ontvangt de echte
`cmd.exe`-uitvoer in de spiegel, en typen ín de spiegel voert uit op de sessie. Twee
toetsenborden, aantoonbaar.

---

## Wat er onderweg fout bleek

- **De firewallcheck zei altijd nee.** `Get-NetFirewallPortFilter` als losse lijst koppelt
  niet op `InstanceID` aan de regels — de 599 objecten bevatten de InstanceID van onze
  eigen regel niet, terwijl de associatie hem wél geeft. En er stonden twee
  Defender-**block**-regels op `taurus.exe` die van elke allow winnen. Allebei opgelost, in
  één UAC-stap.
- **`herdr --session X server stop` deed niets** — exitcode 0, sessie bleef staan, en het
  startte de default-sessie. De juiste vorm is `herdr session stop <naam>` gevolgd door
  `herdr session delete <naam>`.
- **Connect liet herdr-sessies achter** bij elke wegwerpsessie; `ephemeral` betekent nu aan
  beide kanten wegwerp.
- **De vertrouwde adapter matchte nooit.** De Network List Manager schrijft een GUID
  zonder accolades, `GetAdaptersAddresses` mét. Een kale vergelijking matchte dus nooit,
  `trusted_ipv4()` gaf altijd `None`, en de aankondiging bleef **op elke machine**
  stilzwijgend achterwege — zonder fout, want "geen vertrouwd netwerk" is een geldig
  antwoord. Kwam alleen boven door de echte opstelling te bouwen.
- **Hulp bieden vroeg een eigen SSH-sleutel.** De helper bood niets aan omdat deze machine
  geen `~/.ssh/id_*` heeft, dus `auth_publickey` werd nooit aangeroepen en het token kreeg
  geen kans. De host beantwoordt nu `auth_none` bij een geldig token — niet zwakker, want
  op dat pad werd elke sleutel geaccepteerd zodra het token klopte.
- **#126 stond op een verkeerde aanname**: join kwam nog op `cmd.exe` uit. Taurus vraagt
  uitgaand altijd om `ssh -t host "<agent>"` en nooit om een login; inkomend hoort dat net
  zo te zijn.

---

## Wat er nog moet

1. **Versienummer.** Staat op 0.5.2; dit is een flinke functieronde, dus het nummer
   bevestig jij.
2. **Een ronde met jouw eigen ogen op ursu.** De lus is bewezen tussen twee instanties op
   dit werkstation; over een echte LAN heen is het dezelfde code maar niet dezelfde dag.

---

## Draaiende opstelling

| wat | waar |
|---|---|
| startscript testclient | `C:\Users\AST\claude\Taurus\start-taurus-test.ps1` (met `-Exe <pad>` voor een build elders) |
| binary | `C:\Users\AST\claude\Taurus\src-tauri\target\release\taurus.exe` (14 aug 23:27) |
| config testclient | `C:\Users\AST\AppData\Roaming\Taurus-TEST` |
| ursu: klaargezet | `C:\Tools\Taurus\taurus-nieuw.exe` + `update-taurus.ps1` |
| ursu: audit | `C:\Users\arjen\AppData\Roaming\Taurus\audit\events.log` |
| testinstructie | `C:\Users\AST\claude\Taurus\TEST-127-machinescherm.md` |
| ontwerpnotitie | `C:\Users\AST\claude\Taurus\DESIGN-vloot-en-vraagmodus.md` |

---

## Achterstand

| # | wat |
|---|---|
| #98 | remote tab support via SSH (epic; grotendeels geland) |
| #91 | zijbalk: scrollen zonder scrollbar |
| #82 | refactor: `lib.rs` en `main.js` opsplitsen — inmiddels 6.401 en 4.593 regels |
| #62 | settings: white-label-sectie |
| #51 | explorer-ergonomie |
| #49 | bestandsboom naast de zijbalk |
| #45 | projecteditor: map scannen |
| #43 | inklapbare projectgroepen |
| #42 | filterveld voor de projectlijst |

---

## Uitgangspunt

Taurus probeert het en werkt waar het kan. Is een omgeving strakker, dan is dat het
probleem van die omgeving — niet iets waar het ontwerp omheen gebouwd wordt.
