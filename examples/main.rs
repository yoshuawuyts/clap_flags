extern crate clap_flags;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate log;
extern crate futures;
extern crate hyper;
extern crate tokio;

use futures::prelude::*;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbosity: clap_flags::Verbosity,
    #[structopt(flatten)]
    logger: clap_flags::Log,
    #[structopt(flatten)]
    port: clap_flags::Port,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let listener = args.port.bind()?;

    args.logger.log_all(args.verbosity.log_level())?;
    info!("Server listening on {}", listener.local_addr()?);

    let handle = tokio::reactor::Handle::current();
    let listener = tokio::net::TcpListener::from_std(listener, &handle)?;

    let server = Server::builder(listener.incoming())
        .serve(|| service_fn_ok(|_| Response::new(Body::from("Hello World"))))
        .map_err(|e| error!("server error: {}", e));
    tokio::run(server);

    Ok(())
}
