use clap::{crate_version, Parser, Subcommand};

use aardvark_dns::commands::run;

#[derive(Parser, Debug)]
#[clap(version = crate_version!())]
struct Opts {
    /// Path to configuration directory
    #[clap(short, long)]
    config: Option<String>,
    /// Host port for aardvark servers, defaults to 5533
    #[clap(short, long)]
    port: Option<u32>,
    /// Filters search domain for backward compatiblity with dnsname/dnsmasq
    #[clap(short, long)]
    filter_search_domain: Option<String>,
    /// Aardvark-dns trig command
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap(version = crate_version!())]
    /// Runs the aardvark dns server with the specified configuration directory.
    Run(run::Run),
}

fn main() {
    env_logger::builder().format_timestamp(None).init();
    let opts = Opts::parse();

    let dir = opts.config.unwrap_or_else(|| String::from("/dev/stdin"));
    let port = opts.port.unwrap_or_else(|| 5533 as u32);
    let filter_search_domain = opts
        .filter_search_domain
        .unwrap_or_else(|| String::from(".dns.podman"));
    let result = match opts.subcmd {
        SubCommand::Run(run) => run.exec(dir, port, filter_search_domain),
    };

    match result {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
