use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");
    // panic!("Problem opening file {:?}", err)
    let filename = "hello.txt";
    // let f = File::open(&filename).expect("Failed to open hello.txt");
    // let f = File::open(&filename).unwrap();
    let f = File::open(&filename);
    let f = match f {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };
}