//! Collection of reusable flags for Clap.
//!
//! ## Examples
//! ```no_run
//!
//! #[derive(structopt::StructOpt, paw_structopt::StructOpt)]
//! #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
//! struct Args {
//!     #[structopt(flatten)]
//!     address: clap_flags::Address,
//!     #[structopt(flatten)]
//!     logger: clap_flags::Log,
//!     #[structopt(flatten)]
//!     port: clap_flags::Port,
//! }
//!
//! #[async_std::main]
//! #[paw::main]
//! async fn main(args: Args) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//!     args.logger.start(env!("CARGO_PKG_NAME"))?;
//!     let mut app = tide::new();
//!     app.at("/").get(|_| async move { "Hello, world!" });
//!     app.listen((&*args.address.address, args.port.port)).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Output
//!
//! ```txt
//! clap_flags 0.3.0
//! Collection of reusable flags for Clap
//!
//! USAGE:
//!     main [FLAGS] [OPTIONS]
//!
//! FLAGS:
//!     -h, --help         Prints help information
//!         --log-all      Enable log output from dependencies
//!     -P, --pretty       Enable pretty printing
//!     -q, --quiet        Suppress all log output
//!     -V, --version      Prints version information
//!     -v, --verbosity    Print more log output
//!
//! OPTIONS:
//!     -a, --address <address>    Network address [default: 127.0.0.1]
//!     -p, --port <port>          Insecure HTTP port [env: PORT=]  [default: 80]
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

mod address;
mod log;
mod port;

pub use crate::log::*;
pub use address::*;
pub use port::*;
