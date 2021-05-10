extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Dear reader, if you can make this prettier, I'd like to hear about it.
    let help_sensitive = "Makes the extension list in both targets and sources case sensitive.
As an example with targets `raw cr2` and sources `jpeg jpg`,
this will remove `RAW`, `raw`, `Cr2` and all other combinations.
Sources will match on all combinations too.
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
        // positional
        .arg(
            Arg::with_name("target_paths")
                // TODO: add multiple later
                .multiple(false)
                .required(true)
                .help("Specify the directory paths to scan"),
        )
        // options
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
        // flags
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .help("Scans directories recursively"),
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

    let targets: HashSet<String> = matches
        .values_of("target_ext")
        .unwrap()
        .map(|x| String::from(x))
        .collect();
    let sources: HashSet<String> = matches
        .values_of("source_ext")
        .unwrap()
        .map(|x| String::from(x))
        .collect();
    let file_map = build_file_map(&matches);

    if matches.is_present("and_source_matches") {
        //remove_files_and(&file_map, &targets, &sources);
        println!("not implemented");
    } else {
        remove_files_or(&file_map, &targets, &sources);
    }
}

fn remove_files_or(
    file_map: &HashMap<String, HashSet<String>>,
    targets: &HashSet<String>,
    sources: &HashSet<String>,
) {
    for (path_stem, exts) in file_map {
        let mut s = String::from(path_stem);

        if check_removal_or(&exts, &sources) {
            let existing_exts: HashSet<String> = targets
                .union(sources)
                .map(|x| String::from(x))
                .collect::<HashSet<String>>()
                .intersection(&exts)
                .map(|x| String::from(x))
                .collect::<HashSet<String>>();

            for ext in existing_exts {
                let full_path = if ext.is_empty() {
                    Path::new(s.as_str())
                } else {
                    s.push('.');
                    s.push_str(ext.as_str());
                    Path::new(s.as_str())
                };

                fs::remove_file(full_path).expect("failed to delete file");
            }
        }
    }
}

fn check_removal_or(exts: &HashSet<String>, sources: &HashSet<String>) -> bool {
    let mut result = true;

    for ext in sources {
        if exts.contains(ext) {
            result = false;
            break;
        }
    }

    result
}

fn build_file_map<'a>(matches: &'a ArgMatches) -> HashMap<String, HashSet<String>> {
    // TODO: add multiple target paths later
    let target_path = Path::new(matches.value_of("target_paths").unwrap());
    let file_list = get_files(target_path, matches.is_present("recursive"));
    let mut file_table: HashMap<String, HashSet<String>> = HashMap::new();

    for file in file_list {
        match file.parent() {
            Some(parent) => {
                let stem = file.file_stem().unwrap().to_str().unwrap();
                construct_key_value(&file, &stem, &parent.to_str().unwrap(), &mut file_table);
            }
            _ => {
                let stem = file.file_stem().unwrap().to_str().unwrap();
                construct_key_value(&file, &stem, &"", &mut file_table);
            }
        }
    }

    dbg!(&file_table);
    file_table
}

fn construct_key_value<'a>(
    file: &'a Path,
    stem: &'a str,
    parent: &'a str,
    file_table: &'a mut HashMap<String, HashSet<String>>,
) -> &'a mut HashMap<String, HashSet<String>> {
    // clean up to just referenece String directly
    let mut path_stem = String::from(parent);
    path_stem.push('/');
    path_stem.push_str(stem);

    if let Some(extension) = file.extension() {
        insert_file_table(file_table, path_stem, &extension.to_str().unwrap())
    } else {
        insert_file_table(file_table, path_stem, &"")
    }
}

fn insert_file_table<'a>(
    file_table: &'a mut HashMap<String, HashSet<String>>,
    path: String,
    ext: &'a str,
) -> &'a mut HashMap<String, HashSet<String>> {
    let exts = file_table.entry(path).or_insert(HashSet::new());
    exts.insert(String::from(ext));

    file_table
}

fn get_files<'a>(p: &'a Path, recurse: bool) -> Vec<PathBuf> {
    let mut file_list: Vec<PathBuf> = Vec::new();

    for file in p.read_dir().expect("Error reading directory") {
        match file {
            Ok(fp) => {
                let fp = fp.path();

                if fp.is_file() {
                    file_list.push(fp.to_path_buf())
                } else if recurse && fp.is_dir() {
                    let mut temp_list = get_files(&fp, recurse);
                    file_list.append(&mut temp_list)
                }
            }
            Err(e) => println!("Failed to read file: {:?}", e),
        }
    }

    file_list
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};
    use std::fs;

    #[test]
    fn map_generation() {
        // create tmp directory
        let p = "/tmp/warp_tests";
        fs::create_dir_all(p).expect("couldn't create directory");

        // create tmp files
        let f_set = [
            "/tmp/warp_tests/file1.txt",
            "/tmp/warp_tests/file1.csv",
            "/tmp/warp_tests/file1.html",
            "/tmp/warp_tests/file1.tex",
            "/tmp/warp_tests/file2.txt",
            "/tmp/warp_tests/file3.csv",
            "/tmp/warp_tests/file3.tex",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<HashSet<String>>();

        // test that map gets built properly with all files
    }
}
