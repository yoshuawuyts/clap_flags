use structopt::StructOpt;

/// A `--port` flag for Structopt.
///
/// This port should be used for unsecured HTTP/1.1 traffic on a TCP port.
///
/// ## Examples
/// ```rust
/// #[derive(Debug, structopt::StructOpt)]
/// struct Cli {
///   #[structopt(flatten)]
///   port: clap_flags::Port,
/// }
/// ```
#[derive(StructOpt, Debug)]
pub struct Port {
    /// The network port to listen to.
    #[structopt(short = "p", long = "port", env = "PORT", default_value = "80")]
    pub port: u16,
}

/// An `--https-port` flag for Structopt.
///
/// This port should be used to listen for both secured HTTP/1.1 and HTTP/2 traffic on a TCP port.
///
/// ## Examples
/// ```rust
/// #[derive(Debug, structopt::StructOpt)]
/// struct Cli {
///   #[structopt(flatten)]
///   https_port: clap_flags::HttpsPort,
/// }
/// ```
#[derive(StructOpt, Debug)]
pub struct HttpsPort {
    /// The network port to listen to.
    #[structopt(long = "https-port", env = "HTTPS_PORT", default_value = "443")]
    pub port: u16,
}

/// An `--http3-port` flag for Structopt.
///
/// This port should be used to listen for both HTTP/3 traffic on a UDP port.
///
/// ## Examples
/// ```rust
/// #[derive(Debug, structopt::StructOpt)]
/// struct Cli {
///   #[structopt(flatten)]
///   h3_port: clap_flags::Http3Port,
/// }
/// ```
#[derive(StructOpt, Debug)]
pub struct Http3Port {
    /// The network port to listen to.
    #[structopt(long = "http3-port", env = "HTTP3_PORT", default_value = "443")]
    pub port: u16,
}
