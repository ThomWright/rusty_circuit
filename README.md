# rusty_circuit

An electronic circuit simulator, written using an [ECS](https://en.wikipedia.org/wiki/Entity%E2%80%93component%E2%80%93system) in Rust.

## Building

`cargo build --features "dev"` - requires nightly for linting

## TODO

- [x] Non-linear, time-invariant circuit solver
- [x] Circuit elements
    - [x] Resistor
    - [x] Voltage source
    - [x] Current source
    - [x] Wire
    - [x] Ground
- [x] Transient analysis
    - [x] Capacitors
    - [ ] Inductors
- [ ] Time-varying sources
- [ ] Assign node IDs based on connector coordinates
- [ ] Interaction events (create, delete, move)

## Notes on using `specs`

Anything dependent on `Delta` (i.e. in the 'update' phase of a game loop) I'm writing as a `specs::System`, e.g. solving dynamic circuits, controlling time-varying voltage sources.

Anything outside the 'update' phase (e.g. creating a resistor, initialising circuit equation, rendering) I'm writing using `specs::World`/`specs::Planner` directly.

See [Ruga](https://github.com/thiolliere/ruga) as an example.
