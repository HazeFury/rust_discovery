// Exercice 1 : Struct simple (débutant)
// 📌 Objectif : Définir et utiliser une struct
// Crée une structure Person avec les champs suivants :

// name (String)
// age (u32)

// Ajoute une méthode greet qui affiche :
// 📢 "Bonjour, je m'appelle <name> et j'ai <age> ans."

// Exemple d'utilisation :

// let alice = Person {
//     name: String::from("Alice"),
//     age: 25,
// };
// alice.greet();


// ---------------------------------------------------------------

struct Person {
    name: String,
    age: u8
}

impl Person {
    fn greet(&self) {
        println!("Bonjour, je m'appelle {} et j'ai {} ans.", self.name, self.age)
    }
}

fn main() {
    let alice = Person {
    name: String::from("Alice"),
    age: 25,
};
alice.greet();
}