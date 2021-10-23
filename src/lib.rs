//! Create a service which will call your runnable periodically after specified time
#![warn(missing_docs)]

use log::trace;

/// for passing milli-seconds
pub struct MilliSeconds(usize);

/// Service runner functionality
pub trait ServiceRunner {
    /// attach your runned and mentioned call back period
    fn attach_runner<F>(run_again_in: MilliSeconds, runner: F)
    where
        F: Fn();
}
