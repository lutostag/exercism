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
    const NUCLEOTIDES: &'static [char] = &['G', 'C', 'T', 'A'];
    pub fn new(dna: &str) -> Result<DNA, usize> {
        check_sequence(dna, DNA::NUCLEOTIDES).map(|s| DNA { sequence: s })
    }

    pub fn to_rna(&self) -> RNA {
        let sequence = self.sequence
            .chars()
            .map(|n| match n {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect();

        RNA { sequence }
    }
}

impl RNA {
    const NUCLEOTIDES: &'static [char] = &['C', 'G', 'A', 'U'];
    pub fn new(rna: &str) -> Result<RNA, usize> {
        check_sequence(rna, RNA::NUCLEOTIDES).map(|s| RNA { sequence: s })
    }
}
