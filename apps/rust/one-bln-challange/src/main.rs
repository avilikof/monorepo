use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Hello, world!");
    let file = File::open("data.csv");

    let reader = io::BufReader::new(file.unwrap());
    let mut store: HashMap<String, String> = HashMap::new();
    for (_, line) in reader.lines().enumerate() {
        match line {
            Err(e) => println!("{e}"),
            Ok(s) => {
                let name: Vec<&str> = s.as_str().split(";").collect();
                match store.get(name[0]) {
                    None => store.insert(name[0].to_owned(), name[1].to_owned()),
                }
            }
        }
    }
}
