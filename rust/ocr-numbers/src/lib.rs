use std::collections::HashMap;

const DIGITS: &str = concat!(
    " _     _  _     _  _  _  _  _ \n",
    "| |  | _| _||_||_ |_   ||_||_|\n",
    "|_|  ||_  _|  | _||_|  ||_| _|\n",
    "                              "
);

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

type OcrRow<'a> = [&'a str; 4];
type OcrChar = [[u8; 4]; 3];

fn to_rows(string: &str) -> Vec<OcrRow> {
    let lines = string.lines().collect::<Vec<_>>();
    let rows = lines.chunks_exact(4).map(|x| [x[0], x[1], x[2], x[3]]);
    rows.collect()
}

fn to_chars(row: OcrRow) -> Vec<OcrChar> {
    let [a, b, c, d] = row;
    let cols = (a.bytes().zip(b.bytes().zip(c.bytes().zip(d.bytes()))))
        .map(|(a, (b, (c, d)))| [a, b, c, d])
        .collect::<Vec<_>>();
    cols.chunks_exact(3).map(|x| [x[0], x[1], x[2]]).collect()
}

pub fn convert(input: &str) -> Result<String, Error> {
    if let Some(bad_line) = input.lines().find(|line| line.len() % 3 != 0) {
        return Err(Error::InvalidColumnCount(bad_line.len()));
    } else if input.lines().count() % 4 != 0 {
        return Err(Error::InvalidRowCount(input.lines().count()));
    }

    let lookup: HashMap<_, _> = to_chars(to_rows(DIGITS)[0])
        .into_iter()
        .enumerate()
        .map(|(idx, c)| (c, idx.to_string()))
        .collect();

    let recognized_rows = to_rows(input).into_iter().map(|row| {
        to_chars(row)
            .into_iter()
            .map(|c| lookup.get(&c).unwrap_or(&"?".to_string()).clone())
            .collect::<String>()
    });
    Ok(recognized_rows.collect::<Vec<_>>().join(","))
}
