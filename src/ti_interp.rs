use std::collections::HashMap;
use std::io::*;

pub struct Interp {
    memory: HashMap<char, f32>,
    stdin: Stdin,
}

impl Interp {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
            stdin: stdin()
        }
    }
    pub fn ld(&mut self, var: &char) -> f32 {
        if self.memory.contains_key(&var) {
            self.memory[&var]
        } else {
            0f32
        }
    }
    pub fn asg(&mut self, var: char, value: f32) {
        self.memory.insert(var, value);
    }

    pub fn input(& self) -> f32{
        let mut buf = String::new();
        self.stdin.read_line(&mut buf);
        buf.trim().parse().unwrap()
    }

    pub fn output(&self, o: f32) {
        println!("{}",o);
    }
}

