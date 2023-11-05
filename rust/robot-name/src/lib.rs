use std::cell::RefCell;
use std::collections::HashSet;

use rand::Rng;

pub struct Robot {
    name: String,
}

thread_local! {
    static ROBOTS: RefCell<HashSet<String>> = RefCell::new(HashSet::new())
}

impl Robot {
    fn generate_unique_name() -> String {
        ROBOTS.with(|c| {
            let mut used = c.borrow_mut();
            loop {
                let name = Self::generate_name();
                if !used.contains(&name) {
                    used.insert(name.clone());
                    return name;
                }
            }
        })
    }

    fn generate_name() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "{}{}{:03}",
            rng.gen_range(b'A'..b'Z' + 1) as char,
            rng.gen_range(b'A'..b'Z' + 1) as char,
            rng.gen_range(0..1000)
        )
    }

    pub fn new() -> Self {
        let mut r = Self {
            name: String::new(),
        };
        r.reset_name();
        r
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        ROBOTS.with(|c| c.borrow_mut().remove(&self.name));
        self.name = Self::generate_unique_name();
    }
}
