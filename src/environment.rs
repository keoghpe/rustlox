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

    pub fn define(&mut self, name: &Token, value: Value) {
        self.values.insert(name.lexeme.to_string(), value);
    }

    pub fn assign(&mut self, name: &Token, value: Value) {
        // TODO Check if the key exists, if it doesn't, throw a runtime error
        // TODO Raise a runtime error "Undefined variable ..."
        self.values.insert(name.lexeme.to_string(), value);
    }

    pub fn get(&self, name: Token) -> Result<Value, RuntimeError> {
        // TODO possibly check if the variable is defined? We shouldn't return null if it was never defined
        // TODO We should not clone here
        Ok(self.values.get(&name.lexeme).unwrap().clone())
        // TODO Raise a runtime error "Undefined variable ..."
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenType, Value};

    use super::Environment;

    #[test]
    fn it_allows_us_to_define_variables() {
        let token = Token {
            ttype: TokenType::IDENTIFIER,
            lexeme: "foo".to_string(),
            literal: None,
            line: 0,
        };

        let mut environment = Environment::new();

        environment.assign(&token, Value::Double { value: 10.0 });

        assert_eq!(
            Value::Double { value: 10.0 },
            environment.get(token).unwrap()
        );
    }
}
