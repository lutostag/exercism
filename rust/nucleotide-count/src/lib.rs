use std::collections::HashMap;

const VALID_NUCLEOTIDES: &'static [char; 4] = &['A', 'C', 'T', 'G'];


pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }
    dna.chars().try_fold(0, |sum, c| match c {
        n if n == nucleotide => Ok(sum + 1),
        n if VALID_NUCLEOTIDES.contains(&n) => Ok(sum),
        c => Err(c),
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    for &nuc in VALID_NUCLEOTIDES {
        counts.insert(nuc, count(nuc, dna)?);
    }
    Ok(counts)
}
