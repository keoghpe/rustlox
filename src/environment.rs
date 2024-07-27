use std::{collections::HashMap, rc::Rc, sync::Mutex};

use log::debug;

use crate::{
    interpreter::{ExpressionResult, InterpreterError},
    token::{Token, Value},
};

#[derive(Default, Debug)]
pub struct Environment {
    values: Mutex<HashMap<String, Value>>,
    pub enclosing: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<Environment>>) -> Environment {
        Environment {
            values: Mutex::new(HashMap::new()),
            enclosing,
        }
    }

    pub fn define(&self, name: &Token, value: &Value) {
        debug!("Defining variable: {}", name.lexeme);

        let mut values_changer = self.values.lock().unwrap();
        values_changer.insert(name.lexeme.to_string(), value.clone());
    }

    pub fn assign(&self, name: &Token, value: &Value) -> ExpressionResult {
        debug!("Assigning variable: {}", name.lexeme);
        let mut values_changer = self.values.lock().unwrap();

        if values_changer.contains_key(&name.lexeme) {
            values_changer.insert(name.lexeme.to_string(), value.clone());
            Ok(value.clone())
        } else {
            match &self.enclosing {
                Some(enclosing_environment) => enclosing_environment.assign(name, value),
                None => Err(InterpreterError::new_runtime_error(
                    name.ttype,
                    format!("Undefined variable '{}'", name.lexeme),
                )),
            }
        }
    }

    pub fn get(&self, name: Token) -> ExpressionResult {
        debug!("Getting variable: {}", name.lexeme);
        let values_changer = self.values.lock().unwrap();
        // TODO We should not clone here
        if values_changer.contains_key(&name.lexeme) {
            Ok(values_changer.get(&name.lexeme).unwrap().clone())
        } else {
            match &self.enclosing {
                Some(enclosing_environment) => {
                    debug!("Getting from enclosing env");

                    enclosing_environment.get(name)
                }
                None => {
                    debug!("No enclosing env");

                    Err(InterpreterError::new_runtime_error(
                        name.ttype,
                        format!("Undefined variable '{}'", name.lexeme),
                    ))
                }
            }
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

        let mut environment = Environment::new(None);

        environment.define(&token, &Value::Double { value: 10.0 });

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

        let mut environment = Environment::new(None);

        environment.define(&token, &Value::Double { value: 10.0 });
        environment.assign(&token, &Value::Double { value: 20.0 });

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

        let environment = Environment::new(None);

        assert_eq!(
            Err(InterpreterError::new_runtime_error(
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

        let mut environment = Environment::new(None);

        assert_eq!(
            Err(InterpreterError::new_runtime_error(
                TokenType::IDENTIFIER,
                "Undefined variable 'foo'".to_string()
            )),
            environment.assign(&token, &Value::Double { value: 20.0 })
        );
    }

    #[test]
    fn it_delegates_get_to_the_enclosing_environment_if_not_found() {
        let foo_token = Token {
            ttype: TokenType::IDENTIFIER,
            lexeme: "foo".to_string(),
            literal: None,
            line: 0,
        };

        let bar_token = Token {
            ttype: TokenType::IDENTIFIER,
            lexeme: "bar".to_string(),
            literal: None,
            line: 0,
        };

        let parent_environment = Environment::new(None);
        parent_environment.define(&foo_token, &Value::Double { value: 10.0 });

        let environment = Environment::new(Some(Box::new(parent_environment)));
        environment.define(&bar_token, &Value::Double { value: 20.0 });

        assert_eq!(
            Ok(Value::Double { value: 20.0 }),
            environment.get(bar_token)
        );

        assert_eq!(
            Ok(Value::Double { value: 10.0 }),
            environment.get(foo_token)
        );
    }

    #[test]
    fn it_returns_an_error_if_not_found_in_parent_scope() {
        let foo_token = Token {
            ttype: TokenType::IDENTIFIER,
            lexeme: "foo".to_string(),
            literal: None,
            line: 0,
        };

        let bar_token = Token {
            ttype: TokenType::IDENTIFIER,
            lexeme: "bar".to_string(),
            literal: None,
            line: 0,
        };

        let parent_environment = Environment::new(None);

        let environment = Environment::new(Some(Box::new(parent_environment)));
        environment.define(&bar_token, &Value::Double { value: 20.0 });

        assert_eq!(
            Ok(Value::Double { value: 20.0 }),
            environment.get(bar_token)
        );

        assert_eq!(
            Err(InterpreterError::new_runtime_error(
                TokenType::IDENTIFIER,
                "Undefined variable 'foo'".to_string()
            )),
            environment.get(foo_token)
        );
    }

    #[test]
    fn it_delegates_assign_to_the_enclosing_environment_if_not_found() {
        let foo_token = Token {
            ttype: TokenType::IDENTIFIER,
            lexeme: "foo".to_string(),
            literal: None,
            line: 0,
        };

        let bar_token = Token {
            ttype: TokenType::IDENTIFIER,
            lexeme: "bar".to_string(),
            literal: None,
            line: 0,
        };

        let parent_environment = Environment::new(None);
        parent_environment.define(&foo_token, &Value::Double { value: 10.0 });

        let environment = Environment::new(Some(Box::new(parent_environment)));
        environment.define(&bar_token, &Value::Double { value: 20.0 });

        let _ = environment.assign(&foo_token, &Value::Double { value: 20.0 });

        assert_eq!(
            Ok(Value::Double { value: 20.0 }),
            environment.get(foo_token)
        );
    }
}
