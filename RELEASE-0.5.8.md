## Een oudere Taurus aan de overkant zegt dat nu zelf

Uit de praktijk, meteen na 0.5.7: hier 0.5.7, op ursu nog 0.5.4, en de verse knop
"open die agent" leverde dit op:

```
'TAURUS-JOIN-LOCAL' is not recognized as an internal or external command
```

Er was niets kapot. Een Taurus van vóór 0.5.6 kent het join-verb niet, behandelt het als
een gewoon commando, vraagt keurig toestemming — en geeft de regel daarna aan cmd. Wat jij
ziet is een shell-fout in een verse tab, en dat leest als een knop die stuk is.

De aanroep die de sessies van die machine al ophaalt, vraagt nu ook zijn versie. Dat gaat
over de gewone ssh-route, dus het kost aan de overkant geen popup. Is die Taurus ouder dan
0.5.6, dan worden zijn agents nog steeds **getoond** maar niet **aangeboden**, met de reden
op de regel: *"Taurus daar is te oud"*, en in de tooltip welke versie er draait. Een versie
die niet te lezen is verandert niets — een knop weghalen op een vermoeden is erger dan een
fout die je kunt lezen.

## Twee agents in dezelfde map zijn twee agents

Ook op ursu gemeten: daar draaien twee tabs in `C:\Users\arjen`, en er kwam er één door. De
ontdubbeling is er om een herdr-sessie en de Taurus-tab op hetzelfde pad niet dubbel te
tonen, maar hij vergeleek tegen álles wat er al stond — dus tegen de eerste tab. Nu vouwt
alleen een herdr-regel een tab op, nooit een tab een andere tab.

En stilletjes eronder: PowerShell geeft `Get-Content -Raw` na `ConvertTo-Json` terug als
object met een `value`-veld, waardoor de hele lijst leeg terugkwam zonder dat iets zei
waarom. Het commando cast nu naar `[string]`, en de lezer accepteert allebei de vormen.

Doorgemeten tegen de echte ursu: versie `0.5.4` gelezen, beide agents in beeld, geen van
beide met een knop.
