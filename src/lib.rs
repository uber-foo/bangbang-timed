//! This crate provides a [bang-bang controller](https://en.wikipedia.org/wiki/Bang%E2%80%93bang_control) implementation
//! where state transition changes can be constrained by specifying optional minimum time limits for each state.
//!
//! Suitable for regular applications using the standard library and embedded applications using
//! [`#![no_std]`](https://doc.rust-lang.org/reference/attributes.html?highlight=no_std#crate-only-attributes).
//!
//! # Example
//! ```
//! use bangbang_timed::prelude::*;
//! use core::time::Duration;
//!
//! // state transition handlers are called before a state transition occurs, provided
//! // that the time constraints have been met — if the handler returns `Err`, the state
//! // transition will be blocked
//! let mut handle_on = || Ok(());
//! let mut handle_off = || Ok(());
//!
//! // one second duration that we will use for our time constraints
//! let one_second = Duration::from_secs(1);
//!
//! // simple method to return the current time, in embedded applications you'll likely
//! // not have access to the standard library and therefor will have to obtain the
//! // milliseconds ellapsed through platform-specific means
//! let now = || {
//!     let now = ::std::time::SystemTime::now();
//!     let now = now.duration_since(::std::time::UNIX_EPOCH).unwrap();
//!     let now = now.as_secs() * 1_000 + now.subsec_nanos() as u64 / 1_000_000;
//!     now as u32
//! };
//!
//! // create a new bang-bang controller with initial state set to `on` and a minimum
//! // time constraint for the `off` state set to one second
//! let mut bang_bang = TimeConstrainedOnOff::new(
//!     // `true` == start in `on` state, `false` == start in `off` state
//!     true,
//!     // handler to call before transitioning to state `on`
//!     Some(&mut handle_on),
//!     // handler to call before transitioning to state `off`
//!     Some(&mut handle_off),
//!     // we're setting no minimum duration for the `on` state
//!     None,
//!     // minimum duration in `off` state before transition can occur
//!     Some(one_second),
//!     // method that will provide the current time
//!     &now,
//! );
//!
//! // starts in an `on` state as per our `new()` call above
//! assert!(bang_bang.is_on());
//!
//! // we can immediately trigger a transition to the `off` state
//! assert!(bang_bang.bang().is_ok());
//! assert!(bang_bang.is_off());
//!
//! // however we cannot immediately transition back to `on` due to our constraint
//! assert!(bang_bang.bang().is_err());
//!     
//! // since the state transition failed, our state remains the same
//! assert!(bang_bang.is_off());
//!     
//! // after an equitable delay...
//! ::std::thread::sleep(one_second);
//!     
//! // we can transition state back to `on`
//! assert!(bang_bang.bang().is_ok());
//! assert!(bang_bang.is_on());
//! ```
//!
//! # Crate Feature Flags
//!
//! These are the feature flags available to customize this crate. For Example,
//! to disable the default features and enable only the `log` feature you could
//! include this in your `Cargo.toml`.
//!
//! ```toml,ignore
//! [dependencies.bangbang_timed]
//! version = "0.1.0"
//! default-features = false
//! features = ["log"]
//! ```
//!
//! | Feature | Default | Description |
//! | --- | --- | --- |
//! | log | enabled | enables the [`log`] crate dependency and logging calls |
//! | all_log | enabled | enables the `log` feature locally as well as in dependencies |
#![no_std]
#![deny(warnings)]
#![deny(bad_style)]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(unused)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(box_pointers)]
#![deny(macro_use_extern_crate)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(variant_size_differences)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

use bangbang::prelude::*;
use core::fmt;
use core::time::Duration;

#[cfg(feature = "log")]
use log::{debug, trace, warn};

/// handler method to be called on a state change
type StateChangeHander = dyn FnMut() -> Result<(), BangBangError> + Sync + Send;

/// handler method to be called when the current time in milliseconds is required
type CurrentTimeMilliseconds = dyn Fn() -> u32 + Sync;

/// A convenience module appropriate for glob imports (`use bangbang_timed::prelude::*;`)
pub mod prelude {
    #[doc(no_inline)]
    pub use super::TimeConstrainedOnOff;
    #[doc(no_inline)]
    pub use bangbang::prelude::*;
}

/// on/off bang-bang controller that restricts how quickly states can be changed
pub struct TimeConstrainedOnOff<'a> {
    bang_bang: OnOff<'a>,
    minimum_on: Option<Duration>,
    minimum_off: Option<Duration>,
    last_changed: u32,
    now: &'a CurrentTimeMilliseconds,
}

impl fmt::Debug for TimeConstrainedOnOff<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TimeConstrainedOnOff {{ on: {} }}",
            self.bang_bang.is_on()
        )
    }
}

impl BangBang for TimeConstrainedOnOff<'_> {
    fn state(&self) -> BangBangState {
        self.bang_bang.state()
    }

    fn set(&mut self, new_state: BangBangState) -> Result<(), BangBangError> {
        let current_state = self.state();
        let time_delta = assess_time_delta(self.last_changed, (self.now)());

        let min_duration = match current_state {
            BangBangState::A => self.minimum_off,
            BangBangState::B => self.minimum_on,
        };
        if let Some(min_duration) = min_duration {
            if min_duration > Duration::from_millis(u64::from(time_delta)) {
                return Err(BangBangError::StateChangeTemporarilyConstrained {
                    from: current_state,
                    to: new_state,
                    code: 0,
                });
            }
        };

        self.bang_bang.set(new_state)?;
        self.last_changed = (self.now)();

        Ok(())
    }
}

impl<'a> TimeConstrainedOnOff<'a> {
    /// creates a new on/off controller with optional notification handlers for each state transition
    pub fn new(
        on: bool,
        handle_on: Option<&'a mut StateChangeHander>,
        handle_off: Option<&'a mut StateChangeHander>,
        minimum_on: Option<Duration>,
        minimum_off: Option<Duration>,
        now: &'a CurrentTimeMilliseconds,
    ) -> Self {
        let last_changed = now();

        let on_off = Self {
            bang_bang: OnOff::new(on, handle_on, handle_off),
            minimum_on,
            minimum_off,
            last_changed,
            now,
        };

        #[cfg(feature = "log")]
        debug!("instiantiated {:?}", &on_off);

        on_off
    }

    /// convienence method for checking if the controller is in the `on` state
    pub fn is_on(&self) -> bool {
        self.bang_bang.is_on()
    }

    /// convienence method for checking if the controller is in the `off` state
    pub fn is_off(&self) -> bool {
        self.bang_bang.is_off()
    }
}

fn assess_time_delta(prior_milliseconds: u32, later_milliseconds: u32) -> u32 {
    // if we have overflown our u32 ms counter or otherwise have less millisecond counted
    // now than previously, assume that the delta can be only as large as the current value
    if later_milliseconds < prior_milliseconds {
        #[cfg(feature = "log")]
        warn!(
            "time delta from {}ms to {}ms is negative, assuming counter overrun, delta is {}ms",
            prior_milliseconds, later_milliseconds, later_milliseconds
        );
        return later_milliseconds;
    };

    let time_delta = later_milliseconds - prior_milliseconds;

    #[cfg(feature = "log")]
    trace!(
        "time delta from {}ms to {}ms is {}ms",
        prior_milliseconds,
        later_milliseconds,
        time_delta,
    );

    time_delta
}
