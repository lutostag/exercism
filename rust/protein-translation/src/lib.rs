use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codons: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
        (0..rna.len())
            .step_by(3)
            .map(|idx| rna.get(idx..idx + 3).and_then(|codon| self.name_for(codon)))
            .take_while(|protein| protein != &Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        codons: pairs.into_iter().collect(),
    }
}
