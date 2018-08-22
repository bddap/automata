# Double Buffer

Double buffers commonly serve one of two purposes:

1. Prevent presentation of state while it is being mutated.
2. Prevent race conditions while mutating state.

The Automata program uses A double buffer for the latter.

A double buffer holds two copies of some data, primary and secondary. One copy is mutated, while the other copy is used for something else.

In our case, the double buffer represents a grid of automata. Automata do not mutate their own state. Instead, they return a new Automata which is then written to a secondary buffer. While game state is updating, the primary buffer is input, and the secondary is output. Each game tick, the primary and secondary buffers are swapped.
