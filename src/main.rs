extern crate glob;
use glob::{glob};

fn main() {
    println!("Find all files in subdirectories!");

    for entry in glob("**/*").expect("Failed to read glob pattern") {
      match entry {
        Ok(path) => println!("{:?}", path.display()),
        Err(e) => println!("Failed to read file: {:?}", e),
      }
    }
}
