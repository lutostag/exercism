use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    numerals: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.numerals)
    }
}

fn romanify(num: u32) -> String {
    let bigger = |n, ten, five, one| {
        romanify(num / n)
            .replace("X", ten)
            .replace("V", five)
            .replace("I", one)
            + &romanify(num % n)
    };
    match num {
        0 => String::new(),
        1 => format!("I"),
        2 => format!("II"),
        3 => format!("III"),
        4 => format!("IV"),
        5 => format!("V"),
        6 => format!("VI"),
        7 => format!("VII"),
        8 => format!("VIII"),
        9 => format!("IX"),
        10...99 => bigger(10, "C", "L", "X"),
        100...999 => bigger(100, "M", "D", "C"),
        1000...3999 => bigger(1000, "", "", "M"),
        _ => panic!("Cannot convert {} to roman numerals, it is too large", num),
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman {
            numerals: romanify(num),
        }
    }
}
