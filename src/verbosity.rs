use log::Level;

/// Easily add a `--verbose` flag to CLIs using Structopt
///
/// # Examples
///
/// ```rust
/// extern crate clap_verbosity_flag;
/// #[macro_use] extern crate structopt;
///
/// use structopt::StructOpt;
/// use clap_verbosity_flag::Verbosity;
///
/// /// Le CLI
/// #[derive(Debug, StructOpt)]
/// struct Cli {
///     #[structopt(flatten)]
///     verbose: Verbosity,
/// }
/// #
/// # fn main() {}
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct Verbosity {
    /// Print more log output
    #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
    verbosity: u8,
    /// Suppress all log output
    #[structopt(long = "quiet", short = "q")]
    quiet: bool,
}

impl Verbosity {
    /// Get the log level.
    pub fn log_level(&self) -> Option<Level> {
        if self.quiet {
            match self.verbosity {
                0 => None,
                1 => Some(Level::Error),
                2 => Some(Level::Warn),
                3 => Some(Level::Info),
                4 => Some(Level::Debug),
                _ => Some(Level::Trace),
            }
        } else {
            match self.verbosity {
                0 => Some(Level::Info),
                1 => Some(Level::Debug),
                _ => Some(Level::Trace),
            }
        }
    }
}

use std::fmt;

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.verbosity)
    }
}
