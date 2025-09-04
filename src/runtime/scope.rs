use std::collections::HashMap;
use crate::parser::Literal;

#[derive(Debug)]
pub struct Scope {
    scopes: Vec<HashMap<String, Literal>>
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            scopes: vec![HashMap::new()]  // Global scope
        }
    }

    pub fn enter(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit(&mut self) {
        self.scopes.pop();
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<Literal> {
        let key = name.as_ref();
        self.scopes.iter().rev()
            .find_map(|scope| scope.get(key).cloned())
    }

    pub fn set(&mut self, name: impl AsRef<str>, value: Literal) {
        let key = name.as_ref();
        self.scopes.last_mut().unwrap().insert(key.to_string(), value);
    }
}