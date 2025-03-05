// Exercice 4 : Enum avec valeurs associées (avancé)
// 📌 Objectif : Enum avec des valeurs différentes selon la variante
// Crée un enum Shape qui peut être :

// Circle(f64) (avec un rayon)
// Rectangle(f64, f64) (avec largeur et hauteur)

// Ajoute une méthode area qui retourne l’aire selon la forme choisie.

// Exemple d'utilisation :

// let c = Shape::Circle(10.0);
// println!("Aire du cercle: {}", c.area());

// let r = Shape::Rectangle(5.0, 10.0);
// println!("Aire du rectangle: {}", r.area());

// 📏 Indice : Utilise match self pour calculer l’aire !

// -----------------------------------------------------------------------------------

enum Shape {
    Circle(f64),
    Rectangle(f64, f64), 
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 3.14 * radius * radius, 
            Shape::Rectangle(width, height) => width * height,
        }
    }
}

fn main() {
    let c = Shape::Circle(10.0);
    println!("Aire du cercle: {}", c.area());

    let r = Shape::Rectangle(5.0, 10.0);
    println!("Aire du rectangle: {}", r.area());
}
