// Écris une fonction en Rust qui prend une slice de nombres entiers et retourne la somme de tous les nombres pairs.

// 📌 Exemple attendu
// assert_eq!(sum_even_numbers(&[1, 2, 3, 4, 5, 6]), 12); // 2 + 4 + 6 = 12
// assert_eq!(sum_even_numbers(&[10, 15, 20, 25]), 30); // 10 + 20 = 30
// assert_eq!(sum_even_numbers(&[7, 9, 11]), 0); // Aucun nombre pair
// assert_eq!(sum_even_numbers(&[]), 0); // Liste vide = 0

// 🛠️ Contraintes :

// Utilise une boucle ou un itérateur .filter() si tu veux t'entraîner dessus.
// Pas besoin de gérer les nombres négatifs, mais si tu veux le faire, c’est un plus !

// ----------------------------------------------------------------------------------------------------------------------------------------

fn sum_even_numbers(numbers: &[i32]) -> i32 {
    if numbers.is_empty() {
        return 0;
    }
    numbers.iter().filter(|&num| num % 2 == 0).sum()
}

fn main() {
assert_eq!(sum_even_numbers(&[1, 2, 3, 4, 5, 6]), 12); // 2 + 4 + 6 = 12
assert_eq!(sum_even_numbers(&[10, 15, 20, 25]), 30); // 10 + 20 = 30
assert_eq!(sum_even_numbers(&[-2, -2, -5]), -4); 
assert_eq!(sum_even_numbers(&[7, 9, 11]), 0); // Aucun nombre pair
assert_eq!(sum_even_numbers(&[]), 0); // Liste vide = 0
}

// -------------------------------------------------------------------

fn sum_even_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().filter(|&&num| num % 2 == 0).sum()
}

fn main() {
    assert_eq!(sum_even_numbers(&[1, 2, 3, 4, 5, 6]), 12); // 2 + 4 + 6 = 12
    assert_eq!(sum_even_numbers(&[10, 15, 20, 25]), 30); // 10 + 20 = 30
    assert_eq!(sum_even_numbers(&[-2, -2, -5]), -4); 
    assert_eq!(sum_even_numbers(&[7, 9, 11]), 0); // Aucun nombre pair
    assert_eq!(sum_even_numbers(&[]), 0); // Liste vide = 0
    }
    


// 💡 Différence ?

// |&num| → Prend une référence sur i32 mais garde num immuable.
// |&&num| → Déstructure directement &&i32 en i32, évitant un accès explicite *num.