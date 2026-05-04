use std::collections::HashMap;
use pyo3::prelude::*;

struct Sedu {
    store: HashMap<String, String>,
}

impl Sedu {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn parse(&mut self, input: &str) -> Vec<String> {
        let mut output = vec![];

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let (k, v) = line.split_once(':').expect("invalid");
            let key = k.trim();
            let val = v.trim();

            if key == "print" {
                output.push(self.resolve(val));
            } else {
                self.store.insert(key.to_string(), val.to_string());
            }
        }

        output
    }

    fn resolve(&self, value: &str) -> String {
        if value.starts_with('$') {
            let mut current = &value[1..];
            let mut seen = std::collections::HashMap::new();

            loop {
                if seen.contains_key(current) {
                    return "cycle".to_string();
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

#[pyfunction]
fn run_sedu(input: String) -> Vec<String> {
    let mut sedu = Sedu::new();
    sedu.parse(&input)
}

#[pymodule]
fn sedu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_sedu, m)?)?;
    Ok(())
}
