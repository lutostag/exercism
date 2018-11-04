use std::collections::HashMap;

static DNA_NUCLEOTIDES: &'static [char] = &['G', 'C', 'T', 'A'];
static RNA_NUCLEOTIDES: &'static [char] = &['C', 'G', 'A', 'U'];

#[derive(Debug, PartialEq)]
pub struct DNA {
    sequence: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    sequence: String,
}

/// Validate a sequence contains only allowed characters, otherwise returns location of first
/// invalid character
fn check_sequence(sequence: &str, allowed: &[char]) -> Result<String, usize> {
    match sequence.chars().position(|c| !allowed.contains(&c)) {
        Some(idx) => Err(idx),
        None => Ok(sequence.to_string()),
    }
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        check_sequence(dna, DNA_NUCLEOTIDES).map(|s| DNA { sequence: s })
    }

    pub fn to_rna(self) -> RNA {
        let dna_to_rna: HashMap<_, _> =
            DNA_NUCLEOTIDES.iter().zip(RNA_NUCLEOTIDES.iter()).collect();
        let sequence = self.sequence
            .chars()
            .map(|n| dna_to_rna.get(&n).unwrap().clone())
            .collect::<String>();

        RNA { sequence }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        check_sequence(rna, RNA_NUCLEOTIDES).map(|s| RNA { sequence: s })
    }
}
