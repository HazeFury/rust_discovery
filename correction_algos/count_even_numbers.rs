// 🔹 Exercice: Compter les chiffres pairs
// Écris une fonction qui compte le nombre de chiffres pairs dans un tableau d'entiers.

// Exemple :

// assert_eq!(count_even_numbers(&[1, 2, 3, 4, 5, 6]), 3); // 2, 4, 6
// assert_eq!(count_even_numbers(&[7, 9, 11]), 0); // Aucun nombre pair
// assert_eq!(count_even_numbers(&[]), 0); // Liste vide

// ----------------------------------------------------------------------------------------

fn count_even_numbers(numbers: &[u32]) -> usize {
    numbers.iter().filter(|&num| num % 2 == 0).count()
}

fn main() {
assert_eq!(count_even_numbers(&[1, 2, 3, 4, 5, 6]), 3); // 2, 4, 6
assert_eq!(count_even_numbers(&[7, 9, 11]), 0); // Aucun nombre pair
assert_eq!(count_even_numbers(&[]), 0); // Liste vide
}