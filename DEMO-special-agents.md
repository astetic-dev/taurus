# Special agents — installeren en doorlopen

Versie 0.6.2, op `main`. Alles rondom dit thema zit erin; wat er bewust niet in
zit staat onderaan.

## 1. Draaien zonder je productie aan te raken

**Je hoeft niets af te sluiten.** Dit start de nieuwe build naast je draaiende
Taurus, met een eigen configmap (`%APPDATA%\Taurus-TEST`): eigen `projects.json`,
eigen `sessions.json`, eigen `roles.json`. Je lopende agents blijven waar ze zijn
en worden niet overgenomen.

In een gewone PowerShell, in de Taurus-map:

```powershell
.\start-taurus-test.ps1
```

De titelbalk zegt **TEST**. Er staat nog geen enkele agent in, en dat is precies
wat je wilt om dit uit te proberen.

Twee dingen om te weten:

- Zodra dit testexemplaar draait houdt het `target\release\taurus.exe`
  vergrendeld. Een nieuwe build moet dan naar een andere map
  (`cargo build --release --target-dir target\fixbuild`) en start je met
  `.\start-taurus-test.ps1 -Exe src-tauri\target\fixbuild\release\taurus.exe`.
- **De instellingen zijn wel gedeeld.** Die staan in localStorage van de webview
  en niet in de configmap, dus taal, thema, "vragen voor afsluiten" en de
  onthouden uitrolmap gelden voor beide exemplaren. Agents, sessies en rollen zijn
  gescheiden; de voorkeuren niet.

Wil je hem later echt in gebruik nemen, dan moet Taurus wel dicht:

```powershell
Copy-Item "C:\Users\AST\claude\Taurus\src-tauri\target\release\taurus.exe" "C:\Tools\Taurus\taurus.exe" -Force
```

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

Daaronder staat **Komt in** met het volledige pad, en eronder staat dat het een
**voorstel** is. Pas het aan, of kies met 📁 een andere ouder -- dan rekent Taurus
de rest van het pad opnieuw uit onder die plek.

In het testexemplaar stelt hij nog niets voor: de ouder wordt afgeleid uit de map
die je bestaande lokale agents delen, en dat lijstje is daar leeg. Kies dus zelf,
bijvoorbeeld `C:\Users\AST\claude\_taurus-test` -- dan blijft je echte `claude`-map
schoon terwijl je dit uitprobeert.

**Uitrollen.** De kaart verschijnt bovenin, en de toast zegt hoeveel bestanden waar
geland zijn. Hij wordt niet gestart — dat rapport is het ding dat je wilt lezen.

Doe hetzelfde met **Cassini** om de andere tak te zien (Taurus schrijft de
`CLAUDE.md`; open hem, hij wijst naar `identity.md`, `rules.md` en `reference/` en
zegt dat er geen skill nodig is).

## 4. Ingebed uitrollen

Klik **＋ → Vör**. *Waar werkt hij?* staat nu op **In: …** zodra je één werkproces
hebt — kies bijvoorbeeld `Taurus dev`. Vul bij het onderwerp `meetings` in.

Het voorstel wordt dan `<map van dat werkproces>\_coaching\meetings`: een submap in
de map van het proces waar hij over gaat. **Ook dat is een voorstel.** Wil je die
map niet in een productie-repo hebben, kies dan met 📁 een andere ouder -- Taurus
houdt de underscore-vorm aan, want ingebed blijft ingebed.

Na het uitrollen staat de kaart **ingesprongen onder zijn werkproces**, met een
streepje ernaartoe.

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

Klik **"Laat Cassini hiernaar kijken"** op een van je bestaande kaarten. Taurus
selecteert Cassini en zet het pad van die kaart in zijn taak. Starten doe je zelf,
zodat je nog ziet wat er gaat gebeuren.

Richten is niet hetzelfde als inbedden. Inbedden zet een submap in een map die van
jou is; richten laat een staande rol ergens naar kíjken zonder er iets neer te
zetten. Daarom gaat het pad in de taak en niet in een uitrol — en daarom kun je een
leesrol ook op een map richten die geen ICM-werkproces is.

## 11. Opdrachten worden bewaard

Start een rol met een taak erin. Taurus legt die taak vast in de werkplek van die
rol zelf, als `_assignments.md`:

```markdown
# Assignments

What was asked of this role, newest last. Written by Taurus.

## 2026-08-17 14:32

Kijk naar deze map en doe wat je hoort te doen: X:\AI
```

Onderaan de balk staat **≡ Opdrachten**. Dat leest de bestanden van al je rollen en
zet ze per rol op een rij, nieuwste bovenaan, met erbij of die rol staand is of in
welk werkproces hij zit.

Het bestand staat in de map van de rol zelf, dus onder zijn eigen map in de
standaard uitrolmap, naast de uitkomst waar de vraag bij hoort. Het is ongetrackt
en staat in `.git/info/exclude`, dus het geldt niet als wijziging van de repo van
de eigenaar en het blokkeert **Bijwerken** niet.

Een sessie zonder taak legt niets vast: dat is geen opdracht.

---

## Bewust niet gebouwd

**Repositories downloaden voor onderzoek.** Kvasir regelt dat zelf, in zijn eigen
werkplek, en ruimt het daarna op. Taurus hoeft daar niets voor te doen -- en dat is
ook waarom de andere helft van het richten (een adres naar een wegwerpmap klonen)
er niet in zit.

**Een agent registreren die door een agent gemaakt is.** Als Jake of Heimdall een
map achterlaat, druk je zelf op + en kies je hem als map die je al hebt. Dat werkt
vandaag; de automatische variant wacht tot er een rol is die het afgesproken
bestandje schrijft, anders bouwen we een lezer voor iets wat niemand schrijft.

## Niet getest, en dat hoor je te weten

- **Uitrollen naar een remote host.** Het pad is er en is in de code gemarkeerd,
  maar heeft nooit tegen een echte machine gedraaid.
- **`ook als skill`** installeert naar `~/.claude/skills/icm-<rol>/`. Gebouwd, niet
  gedraaid.
- **De schermen zelf.** De Rust-kant heeft 124 tests plus vier die echt met GitHub
  praten; de frontend is statisch gecontroleerd (syntax, elk element bestaat, alle
  i18n-sleutels in nl én en). Maar ik kon de launcher niet starten — deze sessie
  draait erin.
