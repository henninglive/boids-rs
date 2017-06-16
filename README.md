# boids-rs
A simple implementation of [Craig Reynold's boids algorithm](https://en.wikipedia.org/wiki/Boids) implemented in [Rust](https://www.rust-lang.org/).
Boids represent a flock of birds with coordinated group behaviour.
Several rules applied to each boid in turn.
These rules allow the flock to exhibit various behavioural characteristics.

![alt text](boids.gif)

#### Build and Run
1. Ensure you have current version of `cargo` and [Rust](https://www.rust-lang.org/) installed
2. Clone the project `$ git clone https://github.com/henninglive/boids-rs/ && cd boids-rs`
3. Build the project `$ cargo build --release` (NOTE: There is a large performance differnce when compiling without optimizations, so I recommend alwasy using `--release` to enable to them)
4. Once complete, the binary will be located at `target/release/boids-rs`
5. Use `$ cargo run --release` to build and then run, in one step

#### Dependencies
 - [amethyst](https://github.com/amethyst/amethyst) - Fast, data-oriented, and data-driven game engine
 - [gfx-rs](https://github.com/gfx-rs/gfx) - High-performance, bindless graphics API
 - [specs](https://github.com/slide-rs/specs) - Parallel and high performance Entity-Component System
 - [nalgebra](https://github.com/sebcrozet/nalgebra) - Linear algebra library
