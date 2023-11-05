use std::collections::HashSet;

use strum::EnumIter;
use strum::IntoEnumIterator;

pub struct Allergies {
    allergies: HashSet<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, EnumIter)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    fn is_allergic(score: u32, allergen: Allergen) -> bool {
        score & (allergen as u32) != 0
    }

    pub fn new(score: u32) -> Self {
        let mut allergies = HashSet::new();

        for allergy in Allergen::iter() {
            if Allergies::is_allergic(score, allergy) {
                allergies.insert(allergy);
            }
        }

        Self { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.iter().cloned().collect()
    }
}
