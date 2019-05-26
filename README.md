# clap_flags
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Collection of reusable flags for Clap.

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust
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
```

### Output
```txt
clap_flags 0.3.0
Collection of reusable flags for Clap

USAGE:
    main [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
        --log-all      Enable log output from dependencies
    -P, --pretty       Enable pretty printing
    -q, --quiet        Suppress all log output
    -V, --version      Prints version information
    -v, --verbosity    Print more log output

OPTIONS:
    -a, --address <address>    Network address [default: 127.0.0.1]
    -p, --port <port>          Insecure HTTP port [env: PORT=]  [default: 80]
```

## Installation
```sh
$ cargo add clap_flags
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/clap_flags.svg?style=flat-square
[2]: https://crates.io/crates/clap_flags
[3]: https://img.shields.io/travis/yoshuawuyts/clap_flags.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/clap_flags
[5]: https://img.shields.io/crates/d/clap_flags.svg?style=flat-square
[6]: https://crates.io/crates/clap_flags
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/clap_flags
