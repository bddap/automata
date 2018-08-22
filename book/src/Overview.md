# Overview

Automata, the program, is inspired by Conway's Game of Life. Both feature a 2d grid, on which Finite States are displayed as square cells. Each frame, cells on the grid are updated according a Finite State Machine. This application of FSM's is often referred to as Cellular Automaton. Automata, the program, gets it's name from Cellular Automata.

Automata differs from Conway's Game of Life. While cells in Conway's Game of Life may occupy one of two states. Automata's cells may occupy one of 521 states. In the section on [State](./State.html), we'll explore application of the State pattern in the program.

Automata program in action:

<script src='https://asciinema.org/a/xlJ1V1STsW7k3VMDQwX0zj10C.js' id='asciicast-xlJ1V1STsW7k3VMDQwX0zj10C' async data-size="medium" data-theme="tango"></script>

Legend:

|             |                        |
| -:          | :-                     |
| **Color**   | **Atomata State Name** |
| Blue        | Water                  |
| Light Gray  | Air                    |
| White       | RedstoneBlock          |
| Light Red   | High Powered Redstone  |
| Dark Red    | Low Powered Redstone   |
| Dark Gray   | Unpowered Redstone     |
| Gold        | Slug                   |
| Dark Yellow | Slime                  |
