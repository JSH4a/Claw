use std::fs;
fn file_stuff() {
    let paths = fs::read_dir("/").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

fn main() {
    file_stuff();
}