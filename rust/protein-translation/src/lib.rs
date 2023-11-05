use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    cmap: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn empty() -> Self {
        Self {
            cmap: HashMap::new(),
        }
    }
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        if let Some(name) = self.cmap.get(codon) {
            Some(*name)
        } else {
            None
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let vec: Vec<char> = rna.chars().collect();
        let mut stopped = false;

        let mut dna: Vec<_> = Vec::new();

        for cs in vec.chunks(3) {
            let k = String::from_iter(cs);
            if let Some(res) = self.cmap.get(k.as_str()) {
                if *res == "stop codon" {
                    stopped = true;
                    break;
                } else {
                    dna.push(*res);
                }
            } else {
                break;
            }
        }

        if dna.is_empty() || (!stopped && vec.len() % 3 != 0) {
            None
        } else {
            Some(dna)
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut codons = CodonsInfo::empty();

    pairs.iter().for_each(|(r, n)| {
        codons.cmap.insert(r, n);
    });

    codons
}
