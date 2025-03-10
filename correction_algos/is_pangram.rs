// 🔹 Exercice : Vérifier si une chaîne est un pangramme
// Un pangramme est une phrase qui contient toutes les lettres de l’alphabet au moins une fois.
// Écris une fonction qui vérifie si une phrase est un pangramme (uniquement en minuscules, sans ponctuation).

// Exemple :

// assert_eq!(is_pangram("the quick brown fox jumps over the lazy dog"), true);
// assert_eq!(is_pangram("hello world"), false);

// ---------------------------------------------------------------------------------------------------

fn is_pangram(string: &str) -> bool {
    let mut alphabet = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    for char in string.chars() {
        let index = alphabet.iter().position(|x| *x == char);
        if index.is_none() {
            continue;
        }
        else {
            alphabet.remove(index.unwrap());
        }
    }
  alphabet.len() == 0
}

fn main() {
assert_eq!(is_pangram("the quick brown fox jumps over the lazy dog"), true);
assert_eq!(is_pangram("hello world"), false);
}

// -------------------------------------------------

use std::collections::HashSet;

fn is_pangram(string: &str) -> bool {
    let mut letters: HashSet<char> = HashSet::new();
    
    for c in string.chars().filter(|c| c.is_ascii_alphabetic()) {
        letters.insert(c.to_ascii_lowercase());
    }

    letters.len() == 26
}

fn main() {
    assert_eq!(is_pangram("The quick brown fox jumps over the lazy dog"), true);
    assert_eq!(is_pangram("hello world"), false);
}

// Détails : 

// HashSet<char> en Rust est un équivalent à new Set() en TypeScript.
// .filter(|c| c.is_ascii_alphabetic()) enlève tout sauf les lettres (on ignore chiffres, espaces, ponctuation, etc.).
// .insert(c.to_ascii_lowercase()) stocke chaque lettre en minuscule et sans doublon automatiquement.
// letters.len() == 26 → Vérifie si on a exactement les 26 lettres différentes de l'alphabet.

// 💡 Petite précision :
// Si une lettre est déjà dans le HashSet, elle ne sera pas ajoutée une deuxième fois (pas besoin de vérifier nous-mêmes).

