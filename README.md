# bangbang-timed

[![Build Status](https://travis-ci.org/uber-foo/bangbang-timed.svg?branch=master)](https://travis-ci.org/uber-foo/bangbang-timed)
[![Latest Version](https://img.shields.io/crates/v/bangbang-timed.svg)](https://crates.io/crates/bangbang-timed)
[![docs](https://docs.rs/bangbang-timed/badge.svg)](https://docs.rs/bangbang-timed)

This crate provides an implementation of a [bang-bang controller] that can optionally
constrain state transitions by enforcing a minimum-time-in-state. Suitable for regular
applications using the standard library and embedded applications using [no_std].

Bang-bang controllers are relatively simple machines, able to flip-flop solely between two
mutually exclusive states. A common example of a bang-bang controller is a thermostat—the
furnace can be turned either on or off with no other states of operation between those two
extremes. A bang-bang that enforces a minimum operating time at each state could help
ensure that the furnace does not cycle too frequently which could damge or reduce its lifespan.

# Example
```rust
use bangbang::prelude::*;

// Simple struct to hold our current state
struct FlipFlop {
    current_state: BangBangState,
}

// Simplest implementation of a bang-bang controller
impl BangBang for FlipFlop {
    // Return the current active state
    fn state(&self) -> BangBangState {
        self.current_state
    }
    
    // Change the current active state
    fn set(&mut self, new_state: BangBangState) -> Result<(), BangBangError> {
        // Normally there would be logic here to ensure a state transition is possible

        // For the example, we'll just simply change the state as requested
        self.current_state = new_state;

        // No failure cases in this example
        Ok(())
    }
}

fn run_example() -> Result<(), BangBangError> {
    // Create a new bang-bang controller with initial state set to `A`
    let mut flip_flop = FlipFlop { current_state: BangBangState::A };
    assert_eq!(flip_flop.state(), BangBangState::A);

    // Trigger the bang-bang controller, flipping the state to `B`
    flip_flop.bang()?;
    assert_eq!(flip_flop.state(), BangBangState::B);

    // Trigger the bang-bang controller, flipping the state back to `A`
    flip_flop.bang()?;
    assert_eq!(flip_flop.state(), BangBangState::A);

    Ok(())
}

run_example();
```

## License

Licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.

[bang-bang controller]:https://en.wikipedia.org/wiki/Bang%E2%80%93bang_control
[no_std]:https://doc.rust-lang.org/reference/attributes.html?highlight=no_std#crate-only-attributes