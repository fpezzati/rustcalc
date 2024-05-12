use std::fs;

pub fn load_modules_from(modules_path: &str) {
    let paths = fs::read_dir(modules_path).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}