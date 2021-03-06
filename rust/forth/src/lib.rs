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

/// Helper to create builtins. First arg passed in is the stack, the rest are popped off the
/// stack in order (reversed) ending with a statment that must return (end with) a ForthResult
/// example: builtin!(|s, z, y| Ok(s.push(y + z)))
macro_rules! builtin {
    ( | $stack:ident, $( $arg:ident ),* | $block:stmt ) => {
        Operation::Builtin(Box::new(|$stack: &mut Stack| {
            $( let $arg = $stack.pop().ok_or(Error::StackUnderflow)?; )*
            $block
        }))
    };
}

fn is_number(token: &str) -> bool {
    token.chars().all(|c| c.is_ascii_digit())
}

impl Forth {
    pub fn new() -> Self {
        let builtins = hashmap! {
            "+".to_string() => builtin!(|s, z, y| Ok(s.push(y + z))),
            "-".to_string() => builtin!(|s, z, y| Ok(s.push(y - z))),
            "*".to_string() => builtin!(|s, z, y| Ok(s.push(y * z))),
            "/".to_string() => builtin!(|s, z, y| {
                if z == 0 {
                    return Err(Error::DivisionByZero);
                }
                Ok(s.push(y / z))
            }),
            "drop".to_string() => builtin!(|s, _z| Ok(())),
            "dup".to_string() => builtin!(|s, z| Ok(s.append(&mut vec![z, z]))),
            "swap".to_string() => builtin!(|s, z, y| Ok(s.append(&mut vec![z, y]))),
            "over".to_string() => builtin!(|s, z, y| Ok(s.append(&mut vec![y, z, y]))),
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
        let mut ops = Vec::new();
        for token in input {
            if is_number(token) {
                ops.push(Operation::Number(token.parse().unwrap()));
            } else if let Some(op) = self.definitions.get(token) {
                match op {
                    builtin @ Operation::Builtin(_) => ops.push(builtin.clone()),
                    Operation::Definition(d) => ops.append(&mut d.clone()),
                    Operation::Number(_) => unreachable!(), // No definitions map directly to numbers
                }
            } else {
                return Err(Error::UnknownWord);
            }
        }
        Ok(ops)
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

        for op in self.tokenize(words)? {
            match op {
                Operation::Number(n) => self.stack.push(n),
                Operation::Builtin(f) => f(&mut self.stack)?,
                Operation::Definition(_) => unreachable!(), // Already tokenized into other operations
            }
        }
        Ok(())
    }
}
