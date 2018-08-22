#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate log;
#[macro_use]
extern crate structopt;

mod verbosity;

extern crate clap_log_flag;
extern crate clap_port_flag;

pub use clap_log_flag::Log;
pub use clap_port_flag::Port;
pub use verbosity::Verbosity;
