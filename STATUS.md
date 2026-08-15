# Waar het staat — 15 augustus 2026

**0.5.7 is uit.** `main` staat op `2c39ede`; alles is gemerged en er staat geen werk
meer open op een branch. Lokaal geïnstalleerd (`C:\Tools\Taurus\taurus.exe`), op ursu
klaargezet als `taurus-nieuw.exe`.

| onderdeel | stand |
|---|---|
| #121 SSH-host | volledig bewezen, spiegel-tab in beide richtingen |
| #124 machinescherm | uitgeleverd in 0.5.3 |
| #125 vraagmodus | 0.5.3, aanklikbaar in 0.5.5, **tweede vraag komt aan** in 0.5.6 |
| #126 macht volgt toezicht | uitgeleverd in 0.5.3, onbeheerd = `dontAsk` |
| #128 agents i.p.v. shells | 0.5.3; de agent van hiernaast **opent** sinds 0.5.6 (#136) |
| #129 sessiegeschiedenis | 0.5.3, startlijst compleet sinds 0.5.4 |
| #130 permissiemodi | uitgeleverd in 0.5.3, alle zes |
| #135 hulpvraag opnieuw stellen | uitgeleverd in 0.5.6 |
| #136 agent van hiernaast openen | 0.5.6; route wordt afgeleid sinds 0.5.7 |
| tests | 101 groen, 11 ignored |
| release-build | 15 aug 20:55, lokaal geïnstalleerd en op ursu klaargezet |
| versie | **0.5.7** |
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
fixture. Lege sessies staan onder de keuzes als opruimwerk.

### #136 — en die agent van hiernaast open je ook

Eerst werd een agent die in de Taurus dáár leeft wél getoond maar niet aangeboden: geen
kanaal. Dat klopte niet meer zodra de vraagmodus er was — díé fan-out is precies het
kanaal. Klik je zo'n agent aan, dan deelt die Taurus de terminal die hij al open heeft;
er start niets en er verhuist niets.

Twee dingen blijven anders dan bij een hulpvraag. Dit is **niet uitgenodigd** — er kan
iemand in die terminal zitten te werken — dus het loopt langs de gewone
toestemmingsvraag, en geen antwoord binnen de tijd is nee. En een **hulptoken koopt hier
niets**: wie daarop binnen is krijgt de aangeboden sessie en verder niets, anders zou dat
eenmalige token elke andere terminal op die machine openen.

De verbinding gaat over de Taurus-route van die machine (8287), nooit over sshd — daar
zou de shell een commando met die naam proberen te draaien. Staat die route niet in
`hosts.json`, dan wordt hij afgeleid uit de route die je aanklikte: zelfde adres, zelfde
sleutel, vaste poort. Dat bleek nodig op de echte opstelling — ursu stond er alleen als
sshd en WSL in terwijl zijn Taurus al die tijd op 8287 luisterde.

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

**Twee keer vragen, en de agent van hiernaast** (0.5.6/0.5.7, dezelfde opstelling):

| stap | uitkomst |
|---|---|
| A trekt in en vraagt opnieuw | B ziet de **tweede** vraag, met eigen token en eigen agentnaam |
| B beantwoordt die tweede vraag | A krijgt `help-answered`, de balk gaat weg, `help_asking` is null |
| B opent een agent uit A's Taurus | A logt `auth-known`, `session-allow` (*meekijken met sTest*) en `join-local` |
| B typt `echo joinwerkt-123` | komt in A's pty en echoot terug in B |
| A typt `echo vanafA-456` | staat in beide vensters |
| host-regel op poort 22 i.p.v. 8287 | route wordt afgeleid, join komt gewoon aan |

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
- **Een verzoek was zichtbaar maar niet aanklikbaar** (0.5.4). Vier oorzaken tegelijk:
  verzoeken van bekende machines werden weggefilterd, de zoekronde verving de knop onder
  je cursor, alleen het kleine knopje was klikbaar, en foutmeldingen landden in een
  ingeklapt formulier. Alle vier apart genoeg voor "klikken doet niets".
- **Eén keer vragen werkte, twee keer niet** (0.5.5). De mDNS-instantienaam was de
  machine, dus opnieuw vragen was voor de andere kant dezelfde dienst onder dezelfde naam:
  geen nieuwe resolve, oud token, dode knop. De naam draagt nu de vraag.
- **De opgestoken hand ging nooit omlaag.** De backend nam de vraag netjes in zodra iemand
  meekwam, maar het venster luisterde niet naar `help-answered`. Je bleef "vragen" terwijl
  er niets meer werd aangekondigd — en dan voelt een volgende vraag als kapot.
- **De join zou huiswerk hebben teruggegeven** (gevonden ná de 0.5.6-release, door hem
  tegen de echte opstelling te houden). `ursu` heeft geen route-regel op 8287, terwijl zijn
  Taurus daar wél luistert. De route wordt nu afgeleid; hij valt niets aan te kiezen.

---

## Wat er nog moet

1. **ursu bijwerken.** 0.5.7 staat er als `taurus-nieuw.exe`; `update-taurus.ps1` doet de
   wissel. Daar draait nu nog 0.5.4.
2. **Een ronde met jouw eigen ogen op ursu.** De lus is bewezen tussen twee instanties op
   dit werkstation; over een echte LAN heen is het dezelfde code maar niet dezelfde dag.
3. **Eén open ontwerpvraag: mag een hulpvraag jou onderbreken?** Zoeken loopt alleen
   zolang het machinescherm openstaat — dat is de "geen omgevingsgeluid"-regel uit #125.
   Gevolg: vraagt een collega hulp terwijl jij dat scherm dicht hebt, dan zie je het niet.
   Wil je een stil teken (een bolletje op de knop), dan moet er op de achtergrond gezocht
   worden, en dat is precies wat die regel uitsloot. Jouw keuze.

---

## Draaiende opstelling

| wat | waar |
|---|---|
| startscript testclient | `C:\Users\AST\claude\Taurus\start-taurus-test.ps1` (met `-Exe <pad>` voor een build elders) |
| binary | `C:\Users\AST\claude\Taurus\src-tauri\target\release\taurus.exe` (0.5.7, 15 aug 20:55) |
| geïnstalleerd | `C:\Tools\Taurus\taurus.exe` (0.5.7); vorige versies staan ernaast |
| config testclient | `C:\Users\AST\AppData\Roaming\Taurus-TEST` |
| ursu: klaargezet | `C:\Tools\Taurus\taurus-nieuw.exe` (0.5.7) + `update-taurus.ps1` |
| ursu: audit | `C:\Users\arjen\AppData\Roaming\Taurus\audit\events.log` |
| testinstructie | `C:\Users\AST\claude\Taurus\TEST-127-machinescherm.md` |
| ontwerpnotitie | `C:\Users\AST\claude\Taurus\DESIGN-vloot-en-vraagmodus.md` |

---

## Achterstand

| # | wat |
|---|---|
| #98 | remote tab support via SSH (epic; grotendeels geland) |
| #91 | zijbalk: scrollen zonder scrollbar |
| #82 | refactor: `lib.rs` en `main.js` opsplitsen — inmiddels 6.986 en 4.754 regels |
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
