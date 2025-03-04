// Énoncé :
// Écris une fonction find_max qui prend en entrée une slice d'entiers et retourne le plus grand élément.

// Exemple :
// assert_eq!(find_max(&[3, 7, 2, 9, 5]), 9);
// assert_eq!(find_max(&[42, 1, 99, 23]), 99);
// assert_eq!(find_max(&[10]), 10);

// 📝 Indices :

// Une slice en Rust est définie comme &[i32].
// Tu peux utiliser une boucle for pour parcourir les éléments.
// Pense à une variable pour stocker le maximum temporaire.
// Option et .unwrap() peuvent être utiles si la slice est vide (mais il y aura toujours au moins un élément dans les tests).

// ---------------------------------------------------------------------------------------


fn find_max(numbers: &[i32]) -> Option<i32> {
    if numbers.len() == 0 {
        return None;
    }
    let mut current_max_value = numbers[0];

    for num in numbers.iter() {
        if *num > current_max_value {
            current_max_value = *num;
        }
    }
    println!("le plus élevé est : {}", current_max_value);
    Some(current_max_value)
}

fn main() {
assert_eq!(find_max(&[3, 7, 2, 9, 5]).unwrap(), 9);
assert_eq!(find_max(&[42, 1, 99, 23]).unwrap(), 99);
assert_eq!(find_max(&[10]).unwrap(), 10);
}

// ------------------------------------------------------------------

