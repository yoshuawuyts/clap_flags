use structopt::StructOpt;

/// An `--address` flag for Structopt.
///
/// ## Examples
/// ```rust
/// #[derive(Debug, structopt::StructOpt)]
/// struct Cli {
///   #[structopt(flatten)]
///   address: clap_flags::Address,
/// }
/// ```
#[derive(StructOpt, Debug)]
pub struct Address {
    /// Network address
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    pub address: String,
}
