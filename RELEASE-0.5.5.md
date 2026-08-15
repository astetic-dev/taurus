## ✋ Een hulpvraag die je ook echt kunt aannemen

0.5.3 introduceerde de vraagmodus. Je kon je hand opsteken, en aan de andere kant
verscheen het verzoek keurig in beeld — maar erop klikken deed niets. Vier oorzaken,
en elk daarvan is op zich al genoeg voor precies dat.

- **Verzoeken van machines die je al kent werden weggefilterd.** Dat filter stamt uit
  de tijd dat mDNS *aanwezigheid* aankondigde, waar "toon geen machine die je al
  hebt" verstandig was. Bij een opgestoken hand is het omgekeerd: dat ursu in je
  `hosts.json` staat is geen reden om te verbergen dat ursu om hulp vraagt.
- **De knop werd onder je cursor vervangen.** De zoekronde loopt elke 1,5 seconde en
  bouwde de hele lijst opnieuw op. Een klik die over zo'n hertekening heen valt landt
  op een element dat niet meer bestaat, en er gebeurt helemaal niets. De lijst wordt
  nu alleen opnieuw getekend als de verzameling verzoeken echt verandert.
- **Alleen dat kleine knopje was klikbaar.** De balk leest als één geheel, dus op de
  tekst klikken is de logische beweging — en die deed niets. De hele balk is nu de
  knop, met een handje en een hover-kleur om dat te zeggen.
- **En als het misging, zag je niet waarom.** Foutmeldingen gingen naar een element
  dat binnen het ingeklapte "machine toevoegen"-formulier zit: een nette melding in
  een verborgen doosje. Ze verschijnen nu gewoon in beeld.

Verder verdwijnt een verzoek uit de lijst zodra je het beantwoordt, zodat er geen
dode knop blijft staan waarvan het token al op is.
