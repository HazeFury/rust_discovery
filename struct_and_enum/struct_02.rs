// Exercice 2 : Struct avec méthode de calcul (intermédiaire)
// 📌 Objectif : Ajouter des méthodes associées
// Définis une structure Rectangle avec :

// width (f64)
// height (f64)


// Ajoute une méthode area qui retourne l'aire du rectangle.

// Exemple d'utilisation :

// let rect = Rectangle { width: 5.0, height: 10.0 };
// println!("Aire: {}", rect.area()); // Devrait afficher 50.0
// 💡 Conseil : Utilise self dans la méthode area

// -----------------------------------------------------------------------

struct Rectangle {
    height: f64,
    width: f64
}

impl Rectangle {
    fn area(&self) -> f64 {
    self.height * self.width
    }
}

fn main() {
let rect = Rectangle { width: 5.0, height: 10.0 };
println!("Aire: {}", rect.area()); // affiche 50.0
}