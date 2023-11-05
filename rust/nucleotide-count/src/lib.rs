use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let tab = nucleotide_counts(dna)?;
    tab.get(&nucleotide).ok_or(nucleotide).copied()
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut tab = "ACGT".chars().map(|c| (c, 0)).collect::<HashMap<_, _>>();

    for c in dna.chars() {
        tab.get_mut(&c).map(|count| *count += 1).ok_or(c)?;
    }

    Ok(tab)
}
