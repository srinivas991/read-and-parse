extern crate flate2;

use std::fs::{File, read_dir};
use std::io::{prelude::*, BufReader};
use std::{env};
use serde_json::{Result, Value};

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn pretty_print(cloudtrail_entry: &str, check_str: &str) {
    let obj: Value = serde_json::from_str(cloudtrail_entry).unwrap();
    // let pretty = serde_json::to_string_pretty(&obj["Records"][0]).unwrap();
    // println!("{}", pretty);
    let records = &obj["Records"];
    // print_type_of(records);

    if let Some(arr) = records.as_array() {
        // Iterate over the elements of the array
        for element in arr {
            if element.to_string().contains(check_str) {
                let pretty = serde_json::to_string_pretty(element).unwrap();
                println!("{}", pretty);
            }
        }
    }
}

fn check_file_match(filename: &str, check_str: &str) {
    let f = File::open(filename);
    let decoder_new = flate2::read::GzDecoder::new(f.unwrap());

    let reader = BufReader::new(decoder_new);

    for line in reader.lines() {
        match line {
            Ok(v) => {
                if v.contains(check_str) {
                    pretty_print(&v, check_str);
                    println!("FOUND: {}", filename);
                }
            },
            Err(e) => println!("{}", e),
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let dir_path = &args[2];
    // let mut lst: [String; 10] = Default::default();

    let paths = read_dir(dir_path).unwrap();
    // let mut cnt = 0;
    for path in paths {
        // println!("Name: {}", path.unwrap().path().display());

        let st = String::from(path.unwrap().path().display().to_string());
        check_file_match(&st, query);
    }
}
