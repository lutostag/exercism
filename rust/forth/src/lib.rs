#![allow(clippy::unit_arg)] // to make builtin!( ... Ok(...)) more compact/readable
use maplit::hashmap;
use std::collections::HashMap;

pub type Value = i32;
pub type Stack = Vec<Value>;
pub type ForthResult = Result<(), Error>;

#[derive(Clone)]
enum Operation {
    Number(Value),
    Builtin(Box<fn(&mut Stack) -> ForthResult>),
    Definition(Vec<Operation>),
}

#[derive(Default)]
pub struct Forth {
    stack: Stack,
    definitions: HashMap<String, Operation>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

/// Helper to create builtins pass in number of arguments (popped off the stack) and a closure
/// that is called with the mutable stack, and those arguments. Closure must return a ForthResult
macro_rules! builtin {
    ( 1, $( $func:expr )? ) => {
        {
            Operation::Builtin(Box::new(|stack: &mut Stack| {
                let func = $( $func )?;
                let z = stack.pop().ok_or(Error::StackUnderflow)?;
                func(stack, z)
            }))
        }
    };
    ( 2, $( $func:expr )? ) => {
        {
            Operation::Builtin(Box::new(|stack: &mut Stack| {
                let func = $( $func )?;
                let z = stack.pop().ok_or(Error::StackUnderflow)?;
                let y = stack.pop().ok_or(Error::StackUnderflow)?;
                func(stack, y, z)
            }))
        }
    };
}

fn is_number(token: &str) -> bool {
    token.chars().all(|c| c.is_ascii_digit())
}

impl Forth {
    pub fn new() -> Self {
        let builtins = hashmap! {
            "+".to_string() => builtin!(2, |s: &mut Stack, y, z| Ok(s.push(y + z))),
            "-".to_string() => builtin!(2, |s: &mut Stack, y, z| Ok(s.push(y - z))),
            "*".to_string() => builtin!(2, |s: &mut Stack, y, z| Ok(s.push(y * z))),
            "/".to_string() => builtin!(2, |s: &mut Stack, y, z| {
                if z == 0 {
                    return Err(Error::DivisionByZero);
                }
                Ok(s.push(y / z))
            }),
            "drop".to_string() => builtin!(1, |_, _| Ok(())),
            "dup".to_string() => builtin!(1, |s: &mut Stack, z| Ok(s.append(&mut vec![z, z]))),
            "swap".to_string() => builtin!(2, |s: &mut Stack, y, z| Ok(s.append(&mut vec![z, y]))),
            "over".to_string() => builtin!(2, |s: &mut Stack, y, z| Ok(s.append(&mut vec![y, z, y]))),
        };
        Self {
            stack: Vec::new(),
            definitions: builtins,
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    /// Tokenizes an input into evaluable Operations
    fn tokenize<'a>(&self, input: impl Iterator<Item = &'a str>) -> Result<Vec<Operation>, Error> {
        let mut tokens = Vec::new();
        for token in input {
            match token {
                _ if is_number(token) => tokens.push(Operation::Number(token.parse().unwrap())),
                _ if self.definitions.contains_key(token) => {
                    match self.definitions.get(token).unwrap() {
                        builtin @ Operation::Builtin(_) => tokens.push(builtin.clone()),
                        Operation::Definition(d) => tokens.append(&mut d.clone()),
                        Operation::Number(_) => unreachable!(), // No definitions map directly to numbers
                    }
                }
                _ => return Err(Error::UnknownWord),
            }
        }
        Ok(tokens)
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let input = input.to_lowercase();
        let mut words = input.split(' ').take_while(|w| w != &";").peekable();

        if words.peek() == Some(&":") {
            let word = words.nth(1).ok_or(Error::InvalidWord)?;
            if is_number(word) || !input.ends_with(';') {
                return Err(Error::InvalidWord);
            }
            self.definitions.insert(
                word.to_string(),
                Operation::Definition(self.tokenize(words)?),
            );
            return Ok(());
        }

        for token in self.tokenize(words)? {
            match token {
                Operation::Number(n) => self.stack.push(n),
                Operation::Builtin(f) => f(&mut self.stack)?,
                Operation::Definition(_) => unreachable!(), // Already tokenized into other operations
            }
        }
        Ok(())
    }
}
