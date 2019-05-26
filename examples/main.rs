#![feature(async_await)]

#[derive(structopt::StructOpt, paw_structopt::StructOpt)]
#[structopt(author = "", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Args {
    #[structopt(flatten)]
    address: clap_flags::Address,
    #[structopt(flatten)]
    logger: clap_flags::Log,
    #[structopt(flatten)]
    port: clap_flags::Port,
}

#[runtime::main]
#[paw::main]
async fn main(args: Args) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    args.logger.start(env!("CARGO_PKG_NAME"))?;
    let mut app = tide::App::new(());
    app.at("/").get(async move |_| "Hello, world!");
    app.serve((&*args.address.address, args.port.port))?;
    Ok(())
}
