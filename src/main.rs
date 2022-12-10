use std::{io, process};

use clap::{crate_authors, crate_version, Arg, ArgMatches, Command};
use mdbook::{
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor},
};
use mdbook_svgbob::Bob;

fn main() {
    env_logger::init();

    let matches = Command::new("mdbook-svgbob")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("mdbook preprocessor to convert svgbobo to svg")
        .subcommand(
            Command::new("supports")
                .about("Check whether a renderer is supported by this preprocessor")
                .arg(Arg::new("renderer").required(true)),
        )
        .get_matches();

    let pre = Bob::new();

    match matches.subcommand() {
        Some(("supports", sub_matches)) => {
            handle_supports(&pre, sub_matches);
        }
        _ => {
            if let Err(e) = handle_preprocessing(&pre) {
                log::debug!("{}", e);
                process::exit(1);
            }
        }
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    // let book_version = Version::parse(&ctx.mdbook_version)?;
    // let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    // if !version_req.matches(&book_version) {
    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, renderer: &ArgMatches) -> ! {
    let renderer = renderer
        .get_one::<String>("renderer")
        .expect("Required argument");

    let supported = pre.supports_renderer(&renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
