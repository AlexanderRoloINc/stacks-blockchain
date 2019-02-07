use std::collections::HashMap;
use std::collections::HashSet;

use vm::errors::{Error, InterpreterResult as Result};
use vm::types::{DefinedFunction, FunctionIdentifier, Value};
use vm::database::ContractDatabase;

const MAX_CONTEXT_DEPTH: u8 = 128;

pub struct Environment <'a> {
    pub global_context: Context <'a>,
    pub call_stack: CallStack,
    pub database: Box<ContractDatabase>
}

impl <'a> Environment <'a> {
    pub fn new(database: Box<ContractDatabase>) -> Environment<'a> {
        let global_context = Context::new();
        Environment {
            global_context: global_context,
            call_stack: CallStack::new(),
            database: database
        }
    }
}

pub struct Context <'a> {
    pub parent: Option< &'a Context<'a>>,
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, DefinedFunction>,
    depth: u8
}

impl <'a> Context <'a> {
    pub fn new() -> Context<'a> {
        Context {
            depth: 0,
            parent: Option::None,
            variables: HashMap::new(),
            functions: HashMap::new()
        }
    }
    
    pub fn extend(&'a self) -> Result<Context<'a>> {
        if self.depth >= MAX_CONTEXT_DEPTH {
            Err(Error::MaxContextDepthReached)
        } else {
            Ok(Context {
                parent: Some(self),
                variables: HashMap::new(),
                functions: HashMap::new(),
                depth: self.depth + 1
            })
        }
    }

    pub fn lookup_variable(&self, name: &str) -> Option<Value> {
        match self.variables.get(name) {
            Some(value) => Option::Some((*value).clone()),
            None => {
                match self.parent {
                    Some(parent) => parent.lookup_variable(name),
                    None => Option::None
                }
            }
        }
    }

    pub fn lookup_function(&self, name: &str) -> Option<DefinedFunction> {
        match self.functions.get(name) {
            Some(value) => {
                Option::Some(value.clone())
            },
            None => {
                match self.parent {
                    Some(parent) => parent.lookup_function(name),
                    None => Option::None
                }
            }
        }
    }
}

pub struct CallStack {
    pub stack: HashSet<FunctionIdentifier>,
}


impl CallStack {
    pub fn new() -> CallStack {
        CallStack {
            stack: HashSet::new(),
        }
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    pub fn contains(&self, user_function: &FunctionIdentifier) -> bool {
        self.stack.contains(user_function)
    }

    pub fn insert(&mut self, user_function: &FunctionIdentifier) {
        self.stack.insert(user_function.clone());
    }

    pub fn remove(&mut self, user_function: &FunctionIdentifier) {
        if !self.stack.remove(&user_function) {
            panic!("Tried to remove function from call stack, but could not find in current context.")
        }
    }
}