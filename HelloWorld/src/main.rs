use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Fruit {
    color:i32
}
impl Fruit {
    fn get(&self) -> i32 {
        self.color
    }
    fn set(&mut self,  value: &mut i32) {
        println!("set *value={}, value={}", *value, value);
        if *value>10 {
            *value = 10;
        }
        self.color = *value;
    }
}

fn loadFileToString() -> String {
    let file = File::open("sample.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);
    contents
}

fn scan_modal_alias(e:(&str, &json::JsonValue)) -> bool {
    let (k, v) = e;
    println!("{}", k);
    let alias_list = v.members();
    for alias in alias_list {
        println!("{}", alias);
    }
    false
}

fn main() {
    let mut array: [i32; 3] = [0; 3];
    let mut f = Fruit{color:3};
    let mut i :i32 = 899;
    f.set(&mut i);
    println!("Hello, world i = {}, f={}!", i, f.get());

    let jsonStr = loadFileToString();
    let parsed = json::parse(&jsonStr).unwrap();
    let entries = parsed.entries();
    for e in entries {
        scan_modal_alias(e);
        // match e {
        //     (k, v) => println!("{}", k),
        //     _ => println!("no entry")
        // }
        // println!("{}", e.value());
    }
    
}
