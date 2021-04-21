extern crate clap;
extern crate glob;

use clap::{App, Arg};
use glob::glob;

fn main() {
    // Dear reader, if you can make this prettier, I'd like to hear about it.
    let help_sensitive = "Makes the extension list in both targets and sources case sensitive.
As an example:
`wrm -t raw cr2 -s jpeg jpg`

This will remove `RAW`, `raw`, `Cr2` and all other combinations.
Sources will match on combinations like `JpEg`.
";

    let help_and_sources = "Instead of applying an `or` match on sources, apply an `and` match.
This means that all extensions are required to exist.
Partial matches are considered incomplete and will be removed.
As an example:

targets: jpg, gif, exe
sources: raw, csv

If both `raw` and `csv` files do not exist under the same filename,
then any `raw`, `csv`, `jpg`, `gif`, or `exe` will be deleted.
";

    let matches = App::new("Warp-Utilities Remove")
        .version("0.1.0")
        .about("Warp file remover removes files with unmatching extensions")
        .author("Ken Cross")
        .arg(
            Arg::with_name("target_ext")
                .short("t")
                .long("targets")
                .multiple(true)
                .required(true)
                .takes_value(true)
                .help("Specify extensions that will be removed when sourcefiles don't exist"),
        )
        .arg(
            Arg::with_name("source_ext")
                .short("s")
                .long("source-extensions")
                .multiple(true)
                .required(true)
                .takes_value(true)
                .help("Specify extensions in which existence is required"),
        )
        .arg(
            Arg::with_name("case_sensitive")
                .short("S")
                .long("case-sensitive")
                .help(help_sensitive),
        )
        .arg(
            Arg::with_name("empty_source_ext")
                .short("E")
                .long("empty-source-extension")
                .help("Match on source files without extensions"),
        )
        .arg(
            Arg::with_name("empty_target_ext")
                .short("e")
                .long("empty-target-extension")
                .help("Match on target files without extensions"),
        )
        .arg(
            Arg::with_name("and_source_matches")
                .short("a")
                .long("and-source-matches")
                .help(help_and_sources),
        )
        .get_matches();

    if let Some(targets) = matches.values_of("target_ext") {
        for m in targets {
            println!("targets {}\n", m);
        }
    }

    if let Some(sources) = matches.values_of("source_ext") {
        for m in sources {
            println!("sources {}\n", m);
        }
    }

    //for entry in glob("**/*").expect("Failed to read glob pattern") {
    //    match entry {
    //        Ok(path) => println!("{:?}", path.display()),
    //        Err(e) => println!("Failed to read file: {:?}", e),
    //    }
    //}
}

pub fn build_glob_pattern() {}
