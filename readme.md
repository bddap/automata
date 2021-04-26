# Preface

After reading the excellent book, "Game Programming Patterns" by Robert Nystrom, I created a toy program to experiment with concepts from the book.

This report: 

1. Documents the inner workings of the toy program.
2. Explains some patterns from Robert Nystrom's book and details their use in the program.

# Overview

Automata, the program, is inspired by Conway's Game of Life. Both feature a 2d grid, on which Finite States are displayed as square cells. Each frame, cells on the grid are updated according a Finite State Machine. This application of FSM's is often referred to as Cellular Automaton. Automata, the program, gets it's name from Cellular Automata.

Automata differs from Conway's Game of Life. While cells in Conway's Game of Life may occupy one of two states. Automata's cells may occupy one of 521 states. In the section on [State](#state), we'll explore application of the State pattern in the program.

Automata program in action:

[https://asciinema.org/a/410093](https://asciinema.org/a/410093)

Legend:

| **Color**   | **Atomata State Name** |
| -:          | :-                     |
| Blue        | Water                  |
| Light Gray  | Air                    |
| White       | RedstoneBlock          |
| Light Red   | High Powered Redstone  |
| Dark Red    | Low Powered Redstone   |
| Dark Gray   | Unpowered Redstone     |
| Gold        | Slug                   |
| Dark Yellow | Slime                  |

# Program Structure

The Automata program is written in the rust programming language.

Source code for the automata program may be found here: [https://github.com/bddap/automata](https://github.com/bddap/automata)

The program is separated into four files/modules. [Main](#main), [Automata](#automata), [Automata Field](#automata-field), and [Graphics](#graphics).

# Main

Initializes an Automata Field and graphics. Runs a [Game Loop](#game-loop) to [Update](#update-method) Automata [States](#state) and refresh graphics at a regular interval.

# Automata

Defines the core Finite State Machine. Game logic is implemented here.

The term "Automata" is overloaded in this report. "Automata" may mean one of two things.

1. The toy program which this report details.
2. The data structure used to represent a cell on the Automata Field.

Please use context to determine which meaning is intended.

The Automata data structure is an enum defined thusly:

```rust
pub enum Automata {
    Redstone(u8),
    Water(u8),
    RedstoneBlock(),
    GameOfLife(bool),
    Air(),
    Slug(Direction),
    Slime(),
}
```

Rust enums allow for associated data. `Redstone`, `Water`, `GameOfLife`, and `Slug` each include extra state: power, depth, active, and direction of movement respectively.

**Redstone** is inspired by, and behaves somewhat similarly to, Minecraft's [voxel](https://minecraft.gamepedia.com/g00/Redstone) of the same name.

**Water** flows over neighboring blocks.

**RedstoneBlock** provides redstone power.

**GameOfLife** transforms into a redstone block when powered.

**Air** does nothing.

**Slug** travels across the grid.

**Slime** is left in a trail behind Slugs.

# Automata Field

Automata Field represents a two dimensional grid of Automata. A [double buffer](#double-buffer) is used to prevent race conditions between cells.

# Graphics

Graphics prints colored squares to the terminal as part of the [Game Loop](#game-loop). Terminal graphics are employed to simplify development (no windowing libraries necessary).

# Patterns

Five patterns from "Game Programming Patterns" were employed when writing the Automata program.

- [State](#state)
- [Double Buffer](#double-buffer)
- [Game Loop](#game-loop)
- [Data Locality](#data-locality)
- [Update Method](#update-method)

# State

Game Programming Pattern's chapter on state teaches us how to use finite state automata to manage the behavior of in-game objects such as player characters or NPCs.

Modeling behaviors as FSMs makes game code less verbose, and makes a much easier to reason about.

The book gives an example of how state machines can save a game from bugs--translated into rust.

```rust
enum Input {
    PressB,
    PressDown,
    ReleaseDown,
}

...

fn handleInput(&mut self, input: Input) {
    match input {
        PressB => self.jump(),
        PressDown => if !self.isJumping {
            self.setGraphics(Ducking)
        },
        ReleaseDown => self.setGraphics(Standing),
    }
}
```

The bug in the above program occurs when someone presses B in mid-air. The above, non-FSM code will allow jumps even when the player is in the air.

Here is an example of the state pattern in action:

```rust
enum PlayerState {
    Standing,
    Jumping,
    Ducking,
    Diving
}

...

pub fn handleInput(&mut self, input: Input) {
    self.state = match (self.state, input) {
        (Standing, PressB) => (
            self.velocity.y = 1.0;
            Jumping
        ),
        (Standing, PressDown) => Ducking,
        (Ducking, ReleaseDown) => Standing,
        (s, _) => s,
    }
}
```

While the bug is avoidable without a state machine, it's much easier to catch when using the the State pattern.

Automata uses the State pattern to model cell behavior. In fact, cell behavior is completely defined as a single state machine. Automata's state machine is described as a function called `next_middle`. `next_middle` takes the surrounding cell states as input, and returns the next state of the middle cell.

```rust
pub fn next_middle(surroundings: Surroundings) -> Automata {
    if let Some(next) = surroundings.infliction_requested() {
        return next;
    }

    match surroundings.middle {
        Water(0) => Air(),                           // No water => Air
        Water(wetness) => Water(wetness.max(1) - 1), // Water drains over time
        Redstone(pow) => Redstone(pow.max(1) - 1),   // Unpowered redstone goes dark
        Slug(_) => Slime(),                          // Slugs leave a trail of slime
        a => a,                                      // Everything else stays the same
    }
}
```

Cells may only modify themselves. This limitation reduces race conditions, but imposes a limitation on Automata. Namely, how does one Automata impose a change on it's neighbor? Consider the state:

```
Slug(Direction)
```

We want this slug to crawl over every Automata in it's path. In other words, every state update, the slug needs to turn the automata it faces into a slug, and turn itself into slime. This is where the `infliction_requested()` method comes in. `infliction_requested()` asks each surrounding automata, "Do you want to change me?", if any answer yes, the middle automata accepts the state given.

`infliction_requested()` calls `inflict()` on each surrounding Automata to find out whether a change of state is requested. Here how the slug destroys all in it's path:

```rust
fn inflict(&self, other: Self, direction: Direction) -> Option<Self> {
    match self {
        ...
        Slug(slug_direction) => if slug_direction == direction {
            Some(Slug(slug_direction))
        } else {
            None
        },
        ...
    }
}
```

When neighboring Automata request an infliction, one of the inflictions is chosen using a deterministic set of rules. The rules are essentially a ranking system, the requested state with the highest rank is selected. Here's what happens when two slugs collide.

```
fn resolve_infliction(&self, other: Self) -> Self {
    match (*self, other) {
        (Slug(_), Slug(_)) => Slime(),
        ...
    }
}
```

They splat, turning into slime.

# Double Buffer

Double buffers commonly serve one of two purposes:

1. Prevent presentation of state while it is being mutated.
2. Prevent race conditions while mutating state.

The Automata program uses A double buffer for the latter.

A double buffer holds two copies of some data, primary and secondary. One copy is mutated, while the other copy is used for something else.

In our case, the double buffer represents a grid of automata. Automata do not mutate their own state. Instead, they return a new Automata which is then written to a secondary buffer. While game state is updating, the primary buffer is input, and the secondary is output. Each game tick, the primary and secondary buffers are swapped.

# Game Loop

The Automata program employs a naive game loop.

1. State is updated.
2. Game is rendered to the user.
3. Process is repeated.

# Data Locality

Data locality is a performance optimization. Avoid unpredictable memory access and your processor will thank you with a speed boost. Keeping your game state in a contiguous region of memory can increase performance significantly.

The automata program definitely does not need any performance optimization; it runs far faster than needed. That said, the program does benefit from data locality. Game state is stored directly in a pair of standard vectors. Only two heap allocated structures are are used to store the automata grid.

Dynamic dispatch is also avoided. Enums an switch statements are used in place of virtual classes.

# Update Method

Automata Field has an update method which is called each frame. It's called `tick()`, but it does the same thing.

```rust
pub fn tick(&mut self) {
    for x in 0..self.width {
        for y in 0..self.height {
            self.field_alternate[y as usize * self.width as usize + x as usize] = 
                next_middle(self.surroundings_for(x, y))
        }
    }
    mem::swap(&mut self.field, &mut self.field_alternate);
}
```

`tick()` computes the next game state, writing the results to a secondary buffer, then swaps secondary and primary buffers according to the double buffer pattern.

# What I Learned

Rust is a really nice language to work with. The rust compiler is a mentor, strict but kind, always trying nudging you in the right direction.

While cleverness should often be avoided in programming, sometimes the reduction of complexity code provides make cleverness worthwhile.

Research best practices and defacto standards before inventing your own solutions. Lots of other people probably grappled with similar problems in the past, and you will likely find a more elegant, time tested solution.

I learned how to make video games! And maintainable ones at that.

# Works Cited

Nystrom, Robert. Game Programming Patterns. Self Published, 2014. [gameprogrammingpatterns.com](http://gameprogrammingpatterns.com/)

# Try it yourself!

What to run the game on your own machine? Here's how:

```
git clone https://github.com/bddap/automata.git
cd automata
cargo run
```
