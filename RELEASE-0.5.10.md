## Meekijken is een eigen vraag geworden

*Toestaan* en *Meekijken* deden hetzelfde, en dat lag aan de vraag. **Meekijken** betekent
"zet die sessie ook op mijn scherm" — een echte keuze bij een binnenkomend sessieverzoek,
waar iets nieuws start dat je wel of niet wilt volgen. Maar bij iemand die meeleest met een
sessie die hier al draait voegt die knop niets toe: die terminal stáát al op je scherm, het
is de jouwe.

Twee andere knoppen hoorden ook bij die andere vraag:

- **Vol beheer** gaat over wat een sessie krijgt als hij start. Hier start niets; de sessie
  houdt de macht die hij al had.
- **Niet meer vragen voor deze computer** zet die computer op altijd-goed, en dat vinkje
  laat óók echte sessieverzoeken ongevraagd door. Meekijken mag geen achterdeur zijn naar
  iets groters.

Dus meekijken heeft nu zijn eigen popup: weigeren of toestaan, met een waarschuwing die
zegt wat toestaan werkelijk betekent — hij leest mee met deze ene terminal en kan erin mee
typen, er start niets, en de sessie blijft van jou; sluiten kan altijd. En een sessie zonder
titel heet voortaan "een agent op deze computer" in plaats van `s3`: een id is administratie,
geen naam.

Doorgemeten tussen twee instanties: de popup komt binnen met alleen *Weigeren* en
*Toestaan*, toestaan opent gewoon het kanaal (`join-local` in het audit-spoor) en wat je
typt komt aan de overkant aan. Een verzoek dat je laat lopen wordt `share-deny`.
