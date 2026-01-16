# Lerneperiode 13
## Wahl der Technologie
*Spectre.Console*
Technologie, um schönere Terminal-Applikationen zu erstellen.
Genau das wollte ich in meinen ersten Lernperioden erstellen.
Das Tutorial auf der Webseite ist ansprechend  und die Produkte auch.
Aber es ist eher ein nice-to-have skill. Damit kommt man nicht so weit wie mit den beiden anderen Technologien unten.

*React*
Es ist die Standard-Technologie für Apps, die schön aussehen.
Ich habe sie bereits verwendet, um eine mobile Applikation zu entwickeln. 
Die Unübersichtlichkeit und ihre Kompliziertheit des Codes habe ich auf die Seite geschoben. Ja, es macht mir Angst, aber ich bin bereit, es zu erlernen.
Vielleicht in der nächster Lernperiode schon. Dann kann ich meine Portfolio Webseite überarbeiten.

*Rust*
Die Technologie, die schnell und sicher sein kann.
In Rust können verschiedene Apps erstellt werden, ich sehe keine Begrenzung.
Ich benutze GlazeWM, das in Rust geschrieben ist. Deswegen wurden meine Interessen an Rust geweckt.
Es ist auch die Sprache neben React, welche nachgefragt wird. 
Für ein kurzes Lernperiode möchte ich es mal ausprobieren.

Finale Entscheid: *Rust*

## Log
### 09.01
Heute habe ich für drei Technologien kurz recherchieret und mein Entscheid begründet. Ich habe auf der Rust Webseite mit Code gespielt, und dann ein Tutorial auf Youtube gefunden. Dort wurde erklärt, wie man Umgebung einrichtet. Nach einen Moment konnte ich erfolgreich erste Rust app mit Hello World laufen lassen. Danach habe ich cargo benutzt, um Projekt schneller zu erstellen.

## 16.01

- [x] Anhand Tutorial werde ich Benutzern erlauben, ihre Name einzugeben. Dann wird der Benutzer begrüsst.
https://www.youtube.com/watch?v=PQBX-ev5q2k&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=5

- [x] Falls die Name länger als sechs Buchstaben ist, soll der Programm fragen, ob der Benutzer ein Nickname für sich selbst erstellen will. Dann wird wieder mit neuen Nickname begrüsst.
https://www.youtube.com/watch?v=MOa7ulhNYc0&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=7&pp=iAQB

- [x] Arbeitspaket anpassung: RobinTea Fragen beantworten 

- [ ] Der Benutzer kann zwischen 1 und 2 wählen, um entweder seine Name zu ändern oder eine zufällige Zahl zurückzubekommen. Die Aufgaben werden durch Funktionen übernommen.
https://www.youtube.com/watch?v=APrANyLHCtQ&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=9



Heute habe ich für den Anfang Zeit verbracht, @RobinTea mit Projektidee auszuhelfen. Es hat gut funktioniert, da ich eine Erfahrung mit Full-Stack Web-App habe. Danach habe ich den zweiten Tutorial angeschaut und paar einzelne main.rs geschrieben. Ich habe dabei gelernt (mut für variablen, trim to string) und auch ins Fehler gefallen (semicolon sind sehr wichtig, .chars().count()). (56)


## 23.01

- [ ] Number Guesser: Wenn Programm gestartet wurde und Spieler begrüsst hat, gibt ein Benutzer eine Zahl, bekommt er Nachricht, ob er zu hoch oder zu tief war

- [ ] Number Guesser: Wenn das Spiel schon angefangen hat, soll die Spiel weiter nach Zahlen fragen, bis die richtige Zahl geschrieben wird

- [ ] Number Guesser: Menu mit Hardcore mode, wenn der Benutzer nach vier Versuchen nicht getroffen hat, soll die Spiel aufhören

- [ ] Number Guesser: Die Anzahl Gewonnene und verloreene Spiele wird lokal in Datei gespeichert


## 30.1

- [ ] Der Programm wird so wenig wie möglich Heap Stack verwenden
https://www.youtube.com/watch?v=-6cnnNlAvNk&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=9


# Tutorials

https://www.youtube.com/watch?v=gvgBUY8iNO4&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=4

# snippets / commands


`cargo new projectName` init new project
`cargo build` build .exe 
`cargo run` compile and run
`rustfmt main.rs` format code
`input.trim()` remove \n from input
`let mut input = String::new();` declare string variable