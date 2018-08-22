# Automata

Defines the core Finite State Machine. Game logic is implemented here.

The term "Automata" is overloaded in this report. "Automata" may mean one of two things.

1. The toy program which this report details.
2. The data structure used to represent a cell on the Automata Field.

Please use context to determine which meaning is intended.

The Automata data structure is an enum defined thusly:

```rust,ignore
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
