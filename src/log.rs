use env_logger::Builder as LoggerBuilder;
use log::Level;
use pretty_env_logger::formatted_builder;
use structopt::StructOpt;

/// Add log functionality to Structopt.
#[derive(StructOpt, Debug)]
pub struct Log {
    /// Enable log output from dependencies
    #[structopt(long = "log-all")]
    log_all: bool,
    /// Enable pretty printing
    #[structopt(short = "P", long = "pretty")]
    pretty: bool,
    /// Print more log output
    #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
    verbosity: u8,
    /// Suppress all log output
    #[structopt(long = "quiet", short = "q")]
    quiet: bool,
}

impl Log {
    /// Get the log level.
    #[inline]
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

    /// Initialize `env_logger` and set the log level for the given package.
    ///
    /// All other modules default to printing warnings.
    #[inline]
    pub fn start(
        &self,
        own_pkg_name: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let level_filter = match self.log_level() {
            Some(level) => level.to_level_filter(),
            None => return Ok(()),
        };

        let mut builder = if self.pretty {
            formatted_builder()
        } else {
            LoggerBuilder::new()
        };

        if self.log_all {
            builder.filter(Some(&own_pkg_name.replace("-", "_")), level_filter);
        }

        builder
            .filter(None, Level::Warn.to_level_filter())
            .try_init()?;
        Ok(())
    }
}
