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

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        // TODO Check if the key exists, if it doesn't, throw a runtime error
        // TODO Raise a runtime error "Undefined variable ..."
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_string(), value);
            Ok(())
        } else {
            Err(RuntimeError::new(
                name.ttype,
                format!("Undefined variable '{}'", name.lexeme),
            ))
        }
    }

    pub fn get(&self, name: Token) -> Result<Value, RuntimeError> {
        // TODO We should not clone here
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values.get(&name.lexeme).unwrap().clone())
        } else {
            Err(RuntimeError::new(
                name.ttype,
                format!("Undefined variable '{}'", name.lexeme),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::RuntimeError,
        token::{Token, TokenType, Value},
    };

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

        environment.define(&token, Value::Double { value: 10.0 });

        assert_eq!(Ok(Value::Double { value: 10.0 }), environment.get(token));
    }

    #[test]
    fn it_allows_us_to_assign_variables() {
        let token = Token {
            ttype: TokenType::IDENTIFIER,
            lexeme: "foo".to_string(),
            literal: None,
            line: 0,
        };

        let mut environment = Environment::new();

        environment.define(&token, Value::Double { value: 10.0 });
        environment.assign(&token, Value::Double { value: 20.0 });

        assert_eq!(Ok(Value::Double { value: 20.0 }), environment.get(token));
    }

    #[test]
    fn it_returns_an_error_if_attempting_to_get_an_undefined_variable() {
        let token = Token {
            ttype: TokenType::IDENTIFIER,
            lexeme: "foo".to_string(),
            literal: None,
            line: 0,
        };

        let environment = Environment::new();

        assert_eq!(
            Err(RuntimeError::new(
                TokenType::IDENTIFIER,
                "Undefined variable 'foo'".to_string()
            )),
            environment.get(token)
        );
    }

    #[test]
    fn it_returns_an_error_if_attempting_to_assign_an_undefined_variable() {
        let token = Token {
            ttype: TokenType::IDENTIFIER,
            lexeme: "foo".to_string(),
            literal: None,
            line: 0,
        };

        let mut environment = Environment::new();

        assert_eq!(
            Err(RuntimeError::new(
                TokenType::IDENTIFIER,
                "Undefined variable 'foo'".to_string()
            )),
            environment.assign(&token, Value::Double { value: 20.0 })
        );
    }
}
