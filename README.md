# boids-rs
An implementation of Craig Reynold's [Boids algorithm](https://en.wikipedia.org/wiki/Boids) 
implemented in [Rust](https://www.rust-lang.org/) using the [Amethyst game engine](https://github.com/amethyst/amethyst).

Boids simulates a flock of birds with coordinated group behaviour.
Several rules applied to each boid in turn. These rules allow the flock to exhibit various behavioural characteristics.

### Rules
* **Separation**: Steer to avoid crowding local flockmates
* **Alignment**: Steer towards the average heading of local flockmates
* **Cohesion**: Steer to move towards the average position (center of mass) of local flockmates

![alt text](boids.gif)

## Dependencies

If you are compiling on Linux, make sure to install the dependencies below.

### Debian/Ubuntu

```sh
apt install gcc pkg-config openssl libasound2-dev cmake build-essential python3 libfreetype6-dev libexpat1-dev libxcb-composite0-dev libssl-dev libx11-dev libfontconfig1-dev
```

### Other operating systems

See [Amethyst README](https://github.com/amethyst/amethyst/blob/main/README.md).

### Build and Run
1. Ensure you have version 1.47.0 of the rust compiler installed.
   There is [bug](https://github.com/amethyst/amethyst/issues/2524) in amethyst version 0.15 preventing compilation
   with newer versions of rustc. You can override rustc version for current directory with following command 
   `$ rustup default 1.47.0`.
2. Build the project `$ cargo build --release` (NOTE: There is a large performance difference when compiling without optimizations, so I recommend alwasy using `--release` to enable to them)
3. Once complete, the binary will be located at `target/release/boids`
4. Use `$ cargo run --release` to build and then run, in one step
