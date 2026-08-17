# Special agents — installeren en doorlopen

Versie 0.6.0, branch `special-agents-roles`. Alles rondom dit thema zit erin
behalve twee dingen; die staan onderaan.

## 1. Installeren

Taurus vergrendelt zijn eigen exe zolang hij draait, dus dit gaat niet terwijl
het venster open staat. En omdat deze sessie ín Taurus draait, moet jij het doen.

Sluit Taurus, en draai dan in een gewone PowerShell:

```powershell
Copy-Item "C:\Users\AST\claude\Taurus\src-tauri\target\release\taurus.exe" "C:\Tools\Taurus\taurus.exe" -Force
```

Wil je eerst terug kunnen:

```powershell
Copy-Item "C:\Tools\Taurus\taurus.exe" "C:\Tools\Taurus\taurus-0.5.11.exe"
```

Start daarna `C:\Tools\Taurus\taurus.exe`.

## 2. De zeven rollen zien

**Instellingen (⚙) → Agents.** Zeven regels, elk met de bron erin die je hebt
aangeleverd. Per rol twee vinkjes: *gebruik* (staat aan) en *ook als skill* (staat
uit). Haal een bron weg en het gebruik-vinkje gaat uit en wordt onklikbaar — geen
bron, geen vinkje, geen regel in de balk.

De bron is aanpasbaar. Wissel hem en je houdt dezelfde veldmap, dus de
geschiedenis van die rol blijft op één plek staan.

## 3. Een rol uitrollen

**＋** in de balk. Bovenin staat nu een rij tegels: *Map die je al hebt*, de zeven
rollen op naam, en *Ander ICM-adres*.

Klik **Mimir**. Dan:

- de kop zegt welke rol het is en met welke vraag je binnenkomt;
- *Waar werkt hij?* staat op **Staand** (je hebt er nog geen);
- *Waar gaat deze over?* mag leeg — dan heet de map `general`;
- de bron staat al ingevuld.

**Lezen.** Taurus kloont naar een wegwerpmap, leest wat er ligt, en meldt:

```
Mimir — names the one structural reason an agent folder stopped doing what you asked…

vak        Diagnosticus
vorm       werkmap
versie     main @ <sha>, 17 augustus 2026
bevat      eigen CLAUDE.md — die blijft ongemoeid
grootte    <n> KB
```

Dat "eigen CLAUDE.md" is echt: Mimir en Forseti leveren er een mee, de andere vier
niet. Bij Cassini staat er *geen CLAUDE.md — Taurus schrijft er een*.

Daaronder staat **Komt in** met het volledige pad:
`C:\Users\AST\claude\Mimir\diagnosis\general`. De ouder is de map die de meeste
van je lokale agents al delen; 📁 als je hem elders wilt.

**Uitrollen.** De kaart verschijnt bovenin, en de toast zegt hoeveel bestanden waar
geland zijn. Hij wordt niet gestart — dat rapport is het ding dat je wilt lezen.

Doe hetzelfde met **Cassini** om de andere tak te zien (Taurus schrijft de
`CLAUDE.md`; open hem, hij wijst naar `identity.md`, `rules.md` en `reference/` en
zegt dat er geen skill nodig is).

## 4. Ingebed uitrollen

Klik **＋ → Vör**. *Waar werkt hij?* staat nu op **In: …** zodra je één werkproces
hebt — kies bijvoorbeeld `Taurus dev`. Vul bij het onderwerp `meetings` in.

Het pad wordt `C:\Users\AST\claude\Taurus\_coaching\meetings`. Na het uitrollen
staat de kaart **ingesprongen onder Taurus dev**, met een streepje ernaar toe.

Doe het nog eens met onderwerp `projectmanagement`. Twee coaches in hetzelfde
werkproces, naast elkaar, elk met een eigen map onder `_coaching/`.

## 5. Een niet-ICM-repo wordt geweigerd

**＋ → Ander ICM-adres**, en vul `astetic-dev/taurus` in. Lezen geeft:

> Dit lijkt geen ICM-werkproces: geen identity.md en geen SKILL.md in de wortel.
> Wat er wel staat: .gitignore, LICENSE, README.md, package.json, src, src-tauri, …

Er is niets geschreven en er valt niets op te ruimen — de probe gebeurt in een
wegwerpmap.

## 6. Een lokale map als bron

Zet in **Instellingen → Agents** bij de Architect een pad in plaats van een adres,
bijvoorbeeld je eigen `nieuw-werkproces`-map. Rol hem dan uit: Taurus kopieert in
plaats van te klonen, met dezelfde nooit-overschrijven-regels, en het rapport zegt
*lokale bron: geen versiebewaking*. Die kaart krijgt dus nooit een pijltje.

## 7. Versiebewaking

Push iets naar een van je rol-repo's en herstart Taurus. Op de kaart staat een
groen **↑**. Klik erop:

```
Mimir — nieuwe versie
jij hebt     <sha>
er is        <sha>
dit blijft   alles wat hier al gemaakt is
             C:\Users\AST\claude\Mimir\diagnosis\general
```

**Bijwerken** doet `git pull --ff-only` in díe map, en niets anders. Geen
`diagnosis/` wordt heropend. **Laat maar** onthoudt precies die versie; een
volgende commit vraagt opnieuw.

Wil je de weigeringen zien: zet met de hand iets in een bestand in die map en klik
Bijwerken. Dan komt *"Je hebt in deze map zelf iets gewijzigd"* — en de map blijft
ongemoeid. Er draait nooit `reset`, `checkout --force` of `clean`.

## 8. Afsluiten

Sluit Taurus met een agent open. Nu komt er een vraag, met per sessie wat er
gebeurt: wat op een mux draait blijft doorlopen en is met ⇱ weer op te pikken, wat
lokaal draait stopt en is bij de volgende start als conversatie te hervatten. Zet
je de opstartkeuze op *schoon beginnen*, dan belooft de vraag dat laatste niet meer.

**Annuleer** laat alles draaien. **Niet meer vragen** komt terug als een vinkje in
Instellingen → Sessies.

## 9. Wat er ook nog is rechtgezet

- De **✎** op een kaart bewerkt nu díe agent. Hij opende hiervoor de hele lijst,
  ingeklapt, en negeerde waar je op klikte.
- **Verwijderen** doet overal hetzelfde. Een werkproces verwijderen neemt de
  kaarten van zijn ingebedde rollen mee, met het aantal in de bevestiging. Er gaat
  niets van schijf, en de toast noemt het pad dat blijft staan.
- **Beheren** zit onderaan de balk (`✎ Agents`). Die lijst had geen eigen ingang —
  de `＋` was de enige, en daarom vielen aanmaken en beheren samen.
- Bij het aanmaken van een gewone agent zegt Taurus **vóór** het opslaan of er een
  `CLAUDE.md` in de map staat. Ook als hij er wél is; dat zei de app nooit.

---

## 10. Een leesrol op iets anders richten

Rechtermuisknop op een kaart. Dat opende hiervoor meteen het verplaats/sync-scherm;
nu is dat één regel in een menu, met daaronder de leesrollen die je hebt staan —
Cassini, Mimir, Forseti, als je ze uitgerold hebt.

Klik **"Laat Cassini hiernaar kijken"** op bijvoorbeeld `NEXUS AI dev`. Taurus
selecteert Cassini en zet het pad van die map in zijn taak. Starten doe je zelf,
zodat je nog ziet wat er gaat gebeuren.

Richten is niet hetzelfde als inbedden. Inbedden zet een submap in een map die van
jou is; richten laat een staande rol ergens naar kíjken zonder er iets neer te
zetten. Daarom gaat het pad in de taak en niet in een uitrol — en daarom kun je een
leesrol ook op een map richten die geen ICM-werkproces is.

---

## Nog niet gebouwd

**Een onbekende repo verkennen** — de andere helft van #164: een adres plakken,
naar een wegwerpmap klonen en een leesrol daarop richten. Richten op een map die
je al hebt werkt wel (§10).

**Een agent registreren die door een agent gemaakt is** (#165) — bewust
uitgesteld. Als Jake of Heimdall een map achterlaat, druk je zelf op ＋ → map die
je al hebt. Dat werkt vandaag; de automatische variant wacht tot er een rol is die
het afgesproken bestandje schrijft.

## Niet getest, en dat hoor je te weten

- **Uitrollen naar een remote host.** Het pad is er en is in de code gemarkeerd,
  maar heeft nooit tegen een echte machine gedraaid.
- **`ook als skill`** installeert naar `~/.claude/skills/icm-<rol>/`. Gebouwd, niet
  gedraaid.
- **De schermen zelf.** De Rust-kant heeft 123 tests plus vier die echt met GitHub
  praten; de frontend is statisch gecontroleerd (syntax, elk element bestaat, alle
  i18n-sleutels in nl én en). Maar ik kon de launcher niet starten — deze sessie
  draait erin.
