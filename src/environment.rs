use std::collections::HashMap;

use crate::{
    interpreter::RuntimeError,
    token::{Token, Value},
};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Value) {
        // TODO Check if the key exists, if it doesn't, throw a runtime error
        // TODO Raise a runtime error "Undefined variable ..."
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Value, RuntimeError> {
        // TODO possibly check if the variable is defined? We shouldn't return null if it was never defined
        // TODO We should not clone here
        Ok(self.values.get(&name.lexeme).unwrap().clone())
        // TODO Raise a runtime error "Undefined variable ..."
    }
}
