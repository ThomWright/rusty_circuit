# rusty_circuit

An electronic circuit simulator, written using an [ECS](https://en.wikipedia.org/wiki/Entity%E2%80%93component%E2%80%93system) in Rust.

## Building

`cargo build --features "dev"` - requires nightly for linting

## TODO

- [x] Non-linear, time-invariant circuit solver
- [ ] Circuit elements
    - [x] Resistor
    - [x] Voltage source
    - [x] Current source
    - [x] Wire
    - [ ] Ground
- [ ] Transient analysis
    - [ ] Capacitors
    - [ ] Inductors
- [ ] Time-varying sources
- [ ] Assign node IDs based on connector coordinates
- [ ] Interaction events (create, delete, move)
