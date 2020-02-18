#[derive(structopt::StructOpt, paw_structopt::StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
struct Args {
    #[structopt(flatten)]
    address: clap_flags::Address,
    #[structopt(flatten)]
    logger: clap_flags::Log,
    #[structopt(flatten)]
    port: clap_flags::Port,
}

#[async_std::main]
#[paw::main]
async fn main(args: Args) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    args.logger.start(env!("CARGO_PKG_NAME"))?;
    let mut app = tide::new();
    app.at("/").get(|_| async move { "Hello, world!" });
    app.listen((&*args.address.address, args.port.port)).await?;
    Ok(())
}
