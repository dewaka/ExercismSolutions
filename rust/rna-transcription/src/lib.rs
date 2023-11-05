use crate::Strand::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Strand {
    A,
    T,
    C,
    G,
    U,
}

impl Strand {
    pub fn from_char(c: char) -> Option<Strand> {
        match c.to_ascii_uppercase() {
            'A' => Some(A),
            'T' => Some(T),
            'C' => Some(C),
            'G' => Some(G),
            'U' => Some(U),
            _ => None,
        }
    }

    pub fn into_rna(&self) -> Self {
        match &self {
            G => C,
            C => G,
            T => A,
            A => U,
            _ => panic!("Unexpected DNA strand: {:?}", &self),
        }
    }

    pub fn is_dna(&self) -> bool {
        match &self {
            G | C | T | A => true,
            _ => false,
        }
    }

    pub fn is_rna(&self) -> bool {
        match &self {
            C | G | A | U => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    strands: Vec<Strand>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    strands: Vec<Strand>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut strands = vec![];
        for (p, c) in dna.chars().enumerate() {
            match Strand::from_char(c) {
                Some(s) if s.is_dna() => strands.push(s),
                _ => return Err(p),
            }
        }

        Ok(Dna { strands })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            strands: self.strands.iter().map(|s| s.into_rna()).collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut strands = vec![];
        for (p, c) in rna.chars().enumerate() {
            match Strand::from_char(c) {
                Some(s) if s.is_rna() => strands.push(s),
                _ => return Err(p),
            }
        }

        Ok(Rna { strands })
    }
}
