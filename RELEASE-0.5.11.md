## Een sessie die niet hervat, zegt nu waarom

Een lang gesprek dat je had onderbroken stond in de lijst als **"[Request interrupted by
user]"** en wilde niet meer opstarten — zonder dat er iets bij stond.

Er was niets mis met die sessie. Zijn werkmap staat op `X:\AI\...`, een DFS-koppeling naar
`\\nlad.intra\dfsroot`, en die schijf was losgeraakt; die naam loste zelfs niet meer op.
Taurus wist dat ook precies ("Map bestaat niet of is niet bereikbaar: X:\…"), maar vier
dingen hielden het bij je weg.

- **De reden werd weggegooid.** Bij een mislukt herstel kreeg je een lijstje titels in
  plaats van wat de andere kant zei. Een losgeraakte netwerkschijf zag er zo uit als een
  kapotte sessie. De melding gaat nu mee.
- **De onderbreking werd de naam.** De agent schrijft `[Request interrupted by user]` zelf
  in het transcript, als een gebruikersregel, en de titel-logica pakte die op. Juist de
  sessie die je terug wilt heet dan naar het moment waarop je hem stopte. Alles tussen
  blokhaken is het harnas; we lopen door naar een regel die jij typte, en anders is de
  mapnaam het label. Die sessie heet nu gewoon `oracle-reports-migratie`.
- **De regel bood zich toch aan.** De controle keek naar de machine en naar het transcript
  — en dat transcript staat *lokaal* in `~/.claude`, dus die zei vrolijk ja terwijl de map
  onbereikbaar was. Nu wordt ook de map gecontroleerd, en staat de reden op de regel vóór
  je erop klikt.
- **Hervatte sessies kwamen nooit in de geschiedenis.** Die werd alleen gevuld bij een
  verse start. Wie zijn sessies elke dag hervat in plaats van start, hield dus een lege
  `history.json` — precies wat hier aan de hand was.

En één test die er al stond maar zonder `#[test]`, en dus nooit meeliep: hij dekt precies
dit hervat-pad. Hij slaagt.
