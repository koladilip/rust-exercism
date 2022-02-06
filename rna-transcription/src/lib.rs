use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: Vec<char>
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: Vec<char>
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let valid_nucleotides = HashSet::from(['G', 'C', 'T', 'A']);
        let nucleotides = get_valid_nucleotides(dna, &valid_nucleotides)?;
        Ok(Dna{nucleotides})
    }

    pub fn into_rna(self) -> Rna {
        self.into()
    }
}

fn get_valid_nucleotides(nucleotides_string: &str, valid_nucleotides: &HashSet<char>) -> Result<Vec<char>, usize> {
    if let Some(i) = nucleotides_string.chars().position(|c| !valid_nucleotides.contains(&c)) {
        return Err(i);
    }
    Ok(nucleotides_string.chars().collect())
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let valid_nucleotides = HashSet::from(['G', 'C', 'U', 'A']);
        let nucleotides = get_valid_nucleotides(rna, &valid_nucleotides)?;
        Ok(Rna{nucleotides})
    }
}

impl From<Dna> for Rna {
    fn from(dna: Dna) -> Self {
        let dna_rna_map = HashMap::from([('G', 'C'),('C', 'G'),('T', 'A'),('A', 'U')]);

        Rna{nucleotides: dna.nucleotides.iter().map(|c| dna_rna_map[c]).collect()}
    }
}