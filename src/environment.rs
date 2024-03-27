use std::{collections::HashMap, sync::Mutex};

use crate::{
    interpreter::RuntimeError,
    token::{Token, Value},
};

pub struct Environment {
    values: Mutex<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: Mutex::new(HashMap::new()),
        }
    }

    pub fn define(&self, name: &Token, value: Value) {
        let mut values_changer = self.values.lock().unwrap();
        values_changer.insert(name.lexeme.to_string(), value);
    }

    pub fn assign(&self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        let mut values_changer = self.values.lock().unwrap();

        if values_changer.contains_key(&name.lexeme) {
            values_changer.insert(name.lexeme.to_string(), value);
            Ok(())
        } else {
            Err(RuntimeError::new(
                name.ttype,
                format!("Undefined variable '{}'", name.lexeme),
            ))
        }
    }

    pub fn get(&self, name: Token) -> Result<Value, RuntimeError> {
        let mut values_changer = self.values.lock().unwrap();
        // TODO We should not clone here
        if values_changer.contains_key(&name.lexeme) {
            Ok(values_changer.get(&name.lexeme).unwrap().clone())
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
