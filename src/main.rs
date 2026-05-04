use std::collections::HashMap;
use std::env;
use std::fs;

struct Sedu {
    store: HashMap<String, String>,
}

impl Sedu {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn parse(&mut self, input: &str) {
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let (k, v) = line.split_once(':').expect("invalid");
            let key = k.trim();
            let val = v.trim();
            if key == "print" {
                let out = self.resolve(val);
                println!("{}", out);
            } else {
                if key.contains(' ') {
                    panic!("invalid key");
                }
                self.store.insert(key.to_string(), val.to_string());
            }
        }
    }

    fn resolve(&self, value: &str) -> String {
        if value.starts_with('$') {
            let mut current = &value[1..];
            let mut seen = HashMap::new();
            loop {
                if seen.contains_key(current) {
                    return String::from("cycle");
                }
                seen.insert(current.to_string(), true);
                match self.store.get(current) {
                    Some(v) if v.starts_with('$') => {
                        current = &v[1..];
                    }
                    Some(v) => return v.clone(),
                    None => return String::new(),
                }
            }
        } else {
            value.to_string()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("file");
    }
    let content = fs::read_to_string(&args[1]).expect("read");
    let mut sedu = Sedu::new();
    sedu.parse(&content);
}
