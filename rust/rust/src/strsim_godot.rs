use godot::prelude::*;
use strsim::{
    hamming, levenshtein, normalized_levenshtein, osa_distance,
    damerau_levenshtein, normalized_damerau_levenshtein, jaro,
    jaro_winkler, sorensen_dice
};

#[derive(GodotConvert, Var, Export, Clone, Copy, Debug, Default)]
#[godot(via = i64)]
pub enum SimilarityAlgorithm {
    Hamming,
    #[default]
    Levenshtein,
    NormalizedLevenshtein,
    OsaDistance,
    DamerauLevenshtein,
    NormalizedDamerauLevenshtein,
    Jaro,
    JaroWinkler,
    SorensenDice,
}

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct StringSimilarity {
    #[export]
    pub algorithm: SimilarityAlgorithm,
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for StringSimilarity {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            algorithm: SimilarityAlgorithm::default(),
            base,
        }
    }
}

#[godot_api]
impl StringSimilarity {
    #[func]
    pub fn compare(&self, s1: String, s2: String) -> f64 {
        match self.algorithm {
            SimilarityAlgorithm::Hamming => {
                match hamming(&s1, &s2) {
                    Ok(d) => d as f64,
                    Err(_) => -1.0, 
                }
            }
            SimilarityAlgorithm::Levenshtein => levenshtein(&s1, &s2) as f64,
            SimilarityAlgorithm::NormalizedLevenshtein => normalized_levenshtein(&s1, &s2),
            SimilarityAlgorithm::OsaDistance => osa_distance(&s1, &s2) as f64,
            SimilarityAlgorithm::DamerauLevenshtein => damerau_levenshtein(&s1, &s2) as f64,
            SimilarityAlgorithm::NormalizedDamerauLevenshtein => normalized_damerau_levenshtein(&s1, &s2),
            SimilarityAlgorithm::Jaro => jaro(&s1, &s2),
            SimilarityAlgorithm::JaroWinkler => jaro_winkler(&s1, &s2),
            SimilarityAlgorithm::SorensenDice => sorensen_dice(&s1, &s2),
        }
    }

    #[func]
    pub fn get_similarity(&self, s1: String, s2: String, algo: SimilarityAlgorithm) -> f64 {
        match algo {
            SimilarityAlgorithm::Hamming => {
                match hamming(&s1, &s2) {
                    Ok(d) => d as f64,
                    Err(_) => -1.0, 
                }
            }
            SimilarityAlgorithm::Levenshtein => levenshtein(&s1, &s2) as f64,
            SimilarityAlgorithm::NormalizedLevenshtein => normalized_levenshtein(&s1, &s2),
            SimilarityAlgorithm::OsaDistance => osa_distance(&s1, &s2) as f64,
            SimilarityAlgorithm::DamerauLevenshtein => damerau_levenshtein(&s1, &s2) as f64,
            SimilarityAlgorithm::NormalizedDamerauLevenshtein => normalized_damerau_levenshtein(&s1, &s2),
            SimilarityAlgorithm::Jaro => jaro(&s1, &s2),
            SimilarityAlgorithm::JaroWinkler => jaro_winkler(&s1, &s2),
            SimilarityAlgorithm::SorensenDice => sorensen_dice(&s1, &s2),
        }
    }
}
