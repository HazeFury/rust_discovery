// Un palindrome est un mot qui se lit de la même façon à l'endroit et à l'envers.

// 👉 Écris une fonction qui prend une &str en entrée et retourne un bool indiquant si le mot est un palindrome.

// Exemples :

// assert_eq!(is_palindrome("radar"), true);
// assert_eq!(is_palindrome("hello"), false);
// assert_eq!(is_palindrome("racecar"), true);
// assert_eq!(is_palindrome("rust"), false);


// 💡 Pistes pour t'aider :

// Tu peux utiliser .chars() pour parcourir les caractères d'une &str.
// Il existe une méthode .rev() qui permet d'inverser un itérateur.
// Attention aux références (&), tu devras peut-être comparer des char.

// -----------------------------------------------------------------------------------------------------

fn is_palindrome(text: &str) -> bool {
    let mut reverse_text = String::new();

    for c in text.chars().rev() {
        reverse_text.push(c);
    }
    text == reverse_text
}

fn main() {
assert_eq!(is_palindrome("radar"), true);
assert_eq!(is_palindrome("hello"), false);
assert_eq!(is_palindrome("racecar"), true);
assert_eq!(is_palindrome("rust"), false);
}

// --------------------------------------------------------------------

fn is_palindrome(text: &str) -> bool {
    text.chars().eq(text.chars().rev())
}

fn main() {
    assert_eq!(is_palindrome("radar"), true);
    assert_eq!(is_palindrome("hello"), false);
    assert_eq!(is_palindrome("racecar"), true);
    assert_eq!(is_palindrome("rust"), false);
    }
    