## Deep Magic

### Mathmatical Background

Deep Magic is a project designed to investigate unsolved problems in number theory, more specifically, unsolved problems related to magic squares. The problems we are focused on solving are the unsolved enigmas posted by Chris Boyer on his site [MultiMagie](http://www.multimagie.com/English/Enigmas.htm). These problems are chosen because of the prize money offered for their solutions, as well as being a good organization of the open problems in the space.

Additionally, the relative longevity of these problems (They were first proposed in 2010, and 8/11 remain unsolved), promise to offer an engaging challenge well into the forseeable future.

### Technical Background

All of the unsolved engimas require the mathmatician to construct various kinds of magic squares with various properties, or prove that such a construction is impossible. Theorems governing the consturction of magic squares and rules covering broad classes of these squares are elusive, thus the current approach often involves a computerized search through a solution space, checking the generated squares to see if the statisfy the required conditions.

This repository will contain code that generates these squares, searching through the soltion space, but additionally seeking to examine the generated squares, seeking intution and insight into the factors governing the possiblity and impossibility of constructing these squares.

The searching code will be written in Rust. Rust was for its fast runtimes, as well as to provide an learning opportunity for the author. In addition, Rust was used by Sébastien Miquel to solve [enigma #4c](http://www.multimagie.com/English/SquaresOfCubes.htm#7x7), suggesting that it may be well suited to these problems.

### Current Progress

- [x] Write the README
- [ ] Build out the checking infrasturcture
- [ ] Detail the approach to Enigma #6
- [ ] Build out the approach to Enigma #6
- [ ] .....
- [ ] profit
