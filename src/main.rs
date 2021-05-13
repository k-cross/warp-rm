extern crate clap;

use clap::{App, Arg};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Hash, Eq, PartialEq)]
enum WarpFlag {
    Recursive,
    Force,
    AndSource,
}

fn main() {
    // Dear reader, if you can make this prettier, I'd like to hear about it.
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
                .multiple(true)
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
            Arg::with_name("force")
                .short("f")
                .long("force")
                .help("Removal becomes non-interactive"),
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .help("Scans directories recursively"),
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

    let mut warp_flags: HashSet<WarpFlag> = HashSet::new();
    let mut targets: HashSet<String> = matches
        .values_of("target_ext")
        .unwrap()
        .map(|x| String::from(x))
        .collect();
    let mut sources: HashSet<String> = matches
        .values_of("source_ext")
        .unwrap()
        .map(|x| String::from(x))
        .collect();
    let target_paths: Vec<String> = matches
        .values_of("target_paths")
        .unwrap()
        .map(|x| String::from(x))
        .collect();

    if matches.is_present("and_source_matches") {
        warp_flags.insert(WarpFlag::AndSource);
    }
    if matches.is_present("force") {
        warp_flags.insert(WarpFlag::Force);
    }
    if matches.is_present("recursive") {
        warp_flags.insert(WarpFlag::Recursive);
    }
    if matches.is_present("empty_source_ext") {
        sources.insert("".to_string());
    }
    if matches.is_present("empty_target_ext") {
        targets.insert("".to_string());
    }

    let file_map = build_file_map(&target_paths, &warp_flags);

    if warp_flags.contains(&WarpFlag::AndSource) {
        remove_files_and(&file_map, &targets, &sources, &warp_flags);
    } else {
        remove_files_or(&file_map, &targets, &sources, &warp_flags);
    }
}

fn remove_files_or(
    file_map: &HashMap<String, HashSet<String>>,
    targets: &HashSet<String>,
    sources: &HashSet<String>,
    warp_flags: &HashSet<WarpFlag>,
) {
    let mut files: Vec<PathBuf> = Vec::new();

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
                    PathBuf::from(s.as_str())
                } else {
                    s.push('.');
                    s.push_str(ext.as_str());
                    PathBuf::from(s.as_str())
                };

                println!("{}", &full_path.to_str().unwrap());
                files.push(full_path);
            }
        }
    }

    if files.is_empty() {
        println!("Nothing to delete");
    } else {
        actually_remove_files(&files, warp_flags.contains(&WarpFlag::Force));
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

fn remove_files_and(
    file_map: &HashMap<String, HashSet<String>>,
    targets: &HashSet<String>,
    sources: &HashSet<String>,
    warp_flags: &HashSet<WarpFlag>,
) {
    let mut files: Vec<PathBuf> = Vec::new();

    for (path_stem, exts) in file_map {
        let mut s = String::from(path_stem);

        if !sources.is_subset(&exts) {
            let existing_exts: HashSet<String> = targets
                .union(sources)
                .map(|x| String::from(x))
                .collect::<HashSet<String>>()
                .intersection(&exts)
                .map(|x| String::from(x))
                .collect::<HashSet<String>>();

            for ext in existing_exts {
                let full_path = if ext.is_empty() {
                    PathBuf::from(s.as_str())
                } else {
                    s.push('.');
                    s.push_str(ext.as_str());
                    PathBuf::from(s.as_str())
                };

                println!("{}", &full_path.to_str().unwrap());
                files.push(full_path);
            }
        }
    }

    if files.is_empty() {
        println!("Nothing to delete");
    } else {
        actually_remove_files(&files, warp_flags.contains(&WarpFlag::Force));
    }
}

fn actually_remove_files(files: &Vec<PathBuf>, force: bool) {
    println!("Do you really want to remove all the listed files? [y/n]");
    let mut input = String::new();
    if force {
        input.push('y');
    } else {
        std::io::stdin()
            .read_line(&mut input)
            .expect("couldn't read input");
    }

    if input.trim() == "y" {
        for file in files {
            fs::remove_file(file).expect("failed to delete file");
        }
    }
}

fn build_file_map<'a>(
    target_paths: &'a Vec<String>,
    warp_flags: &'a HashSet<WarpFlag>,
) -> HashMap<String, HashSet<String>> {
    let file_list = get_files(&target_paths, warp_flags.contains(&WarpFlag::Recursive));
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

fn get_files<'a>(v: &'a Vec<String>, recurse: bool) -> Vec<PathBuf> {
    let mut file_list: Vec<PathBuf> = Vec::new();

    for s in v {
        let mut temp_list = retrieve_files(&mut PathBuf::from(s), &recurse);
        file_list.append(&mut temp_list)
    }

    file_list
}

fn retrieve_files<'a>(p: &PathBuf, recurse: &bool) -> Vec<PathBuf> {
    let mut file_list: Vec<PathBuf> = Vec::new();

    for file in p.read_dir().expect("Error reading directory") {
        match file {
            Ok(fp) => {
                let fp = fp.path();

                if fp.is_file() {
                    file_list.push(fp.to_path_buf())
                } else if *recurse && fp.is_dir() {
                    let mut temp_list = retrieve_files(&fp, recurse);
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
    use super::WarpFlag;
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use std::path::PathBuf;

    fn teardown() {
        // create tmp directory
        let p = PathBuf::from("/tmp/warp_tests");
        fs::remove_dir_all(p).expect("couldn't remove directory");
    }

    fn setup() -> PathBuf {
        // create tmp directory
        let p = PathBuf::from("/tmp/warp_tests/p1");
        let p2 = PathBuf::from("/tmp/warp_tests/p2");
        fs::create_dir_all(&p).expect("couldn't create directory");
        fs::create_dir_all(&p2).expect("couldn't create directory");

        // create tmp files
        let f_set: HashSet<String> = [
            "/tmp/warp_tests/file1.txt",
            "/tmp/warp_tests/file1.csv",
            "/tmp/warp_tests/file1.html",
            "/tmp/warp_tests/file1.tex",
            "/tmp/warp_tests/file2.txt",
            "/tmp/warp_tests/file3.csv",
            "/tmp/warp_tests/file3.tex",
            "/tmp/warp_tests/p1/file.tex",
            "/tmp/warp_tests/p1/file.html",
            "/tmp/warp_tests/p2/file.tex",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        for file in f_set {
            fs::write(file, "").expect("couldn't write empty file");
        }

        p
    }

    #[test]
    fn map_generation() {
        let p = setup().parent().unwrap().to_str().unwrap().to_string();
        let v = vec![p];
        // test that map gets built properly with all files

        // recursive false
        let mut hm: HashMap<String, HashSet<String>> = HashMap::new();
        let mut set: HashSet<String> = HashSet::new();
        set.insert("txt".to_string());
        set.insert("csv".to_string());
        set.insert("html".to_string());
        set.insert("tex".to_string());
        hm.insert("/tmp/warp_tests/file1".to_string(), set);

        let mut set: HashSet<String> = HashSet::new();
        set.insert("txt".to_string());
        hm.insert("/tmp/warp_tests/file2".to_string(), set);

        let mut set: HashSet<String> = HashSet::new();
        set.insert("csv".to_string());
        set.insert("tex".to_string());
        hm.insert("/tmp/warp_tests/file3".to_string(), set);

        // perform test on map
        let wfs: HashSet<WarpFlag> = HashSet::new();
        let test_map = super::build_file_map(&v, &wfs);
        assert_eq!(test_map, hm);

        // recursive true
        let mut set: HashSet<String> = HashSet::new();
        set.insert("html".to_string());
        set.insert("tex".to_string());
        hm.insert("/tmp/warp_tests/p1/file".to_string(), set);

        let mut set: HashSet<String> = HashSet::new();
        set.insert("tex".to_string());
        hm.insert("/tmp/warp_tests/p2/file".to_string(), set);

        let mut wfs: HashSet<WarpFlag> = HashSet::new();
        wfs.insert(WarpFlag::Recursive);
        let test_map = super::build_file_map(&v, &wfs);
        assert_eq!(test_map, hm);

        teardown();
    }
}
