use crate::{
    app::app_main::App,
    ast::expr_ast::FunctionObject,
    token::token_main::{Token, TokenLiterals},
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SplaxDeclarations {
    Literals(Box<TokenLiterals>),
    Functions(Box<FunctionObject>),
}

// Top level 'Environment' Data structure to store state of the interpreter.
#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, SplaxDeclarations>,
}

impl Environment {
    // constructor.
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        spdlog::debug!("creating new environment hashmap.");
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    /// Defines new variable in environment.
    /// # Arguments
    /// * `name` - String name of the variable.
    /// * `value` - Literal value of the variable in form of token literal,
    pub fn define(&mut self, name: String, value: SplaxDeclarations) {
        spdlog::debug!("defining variable with name : {name} and value : {value:?}");
        self.values.insert(name, value);
    }

    /// Assigns value to an already existing entry in the environment
    /// and return its value.
    /// Otherwise results panic error if the binding does not exist.
    /// # Arguments
    /// * `name` - Variable as Token
    /// * `value` - Assignment value.
    pub fn assign(&mut self, name: Token, value: SplaxDeclarations) -> Option<SplaxDeclarations> {
        // call internal assign_from_str
        if let Some(value) = self.assign_from_str(&name.lexeme, value) {
            return Some(value);
        }

        // throw a runtime error if we couldn't find the indentifier.
        App::runtime_error(
            name.line,
            format!("Reference to undefined variable '{}'", name.lexeme),
        );
        panic!();
    }

    /// Assigns value to an already existing entry in the environment
    /// and return its value.
    /// Otherwise results panic error if the binding does not exist.
    /// # Arguments
    /// * `name` - String name of the variable.
    /// * `value` - Assignment value.
    pub fn assign_from_str(
        &mut self,
        name: &String,
        value: SplaxDeclarations,
    ) -> Option<SplaxDeclarations> {
        spdlog::debug!("trying to assign '{name}' to '{value:?}'");

        // assign value if the value exists in this environment.
        if self.values.contains_key(name) {
            spdlog::trace!("assigning value '{name}' to '{value:?}'");
            return self.values.insert(name.to_string(), value);
        }

        // recursively find in enclosing environments.
        if let Some(enclosing) = &mut self.enclosing {
            spdlog::trace!("trying to assign in enclosing environment");
            return enclosing.assign_from_str(name, value);
        }

        None
    }

    /// Retrieves variable values from the environment, throws runtime error if not found.
    /// # Arguments
    /// * `name` - The token whose's lexeme value will be searched for.
    pub fn get(&self, name: Token) -> SplaxDeclarations {
        match self.get_from_str(name.lexeme.clone()) {
            Some(value) => value,
            None => {
                // throw a runtime error if we couldn't find the indentifier.
                App::runtime_error(
                    name.line,
                    format!("Reference to undefined variable '{}'", name.lexeme),
                );
                panic!();
            }
        }
    }

    pub fn get_from_str(&self, name: String) -> Option<SplaxDeclarations> {
        spdlog::debug!("finding variable with name : {name}");
        // searching the indentifier in the environment itself.
        if let Some(value) = self.values.get(&name) {
            spdlog::trace!("found variable {name} value : '{value:?}'");
            return Some(value.to_owned());
        }

        // searching the indentifier in enclosing environment.
        if let Some(enclosing) = &self.enclosing {
            spdlog::trace!("trying to find '{name}' in enclosing environment");
            return enclosing.get_from_str(name);
        }

        None
    }
}
