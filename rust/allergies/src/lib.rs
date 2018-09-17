#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

pub struct Allergies {
    score: u32,
}

custom_derive! {
    #[derive(Debug, PartialEq, Clone, Copy, IterVariants(Var))]
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
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (*allergen as u32 & self.score) > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter_variants()
            .filter(|a| self.is_allergic_to(&a))
            .collect()
    }
}
