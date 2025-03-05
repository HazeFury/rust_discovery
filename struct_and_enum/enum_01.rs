// Exercice 3 : Enum basique (débutant)
// 📌 Objectif : Définir un enum et l’utiliser avec match
// Déclare un enum TrafficLight avec trois variantes :

// Red
// Yellow
// Green

// Ajoute une méthode duration qui retourne :

// 30 secondes pour Red
// 5 secondes pour Yellow
// 25 secondes pour Green


// Exemple d'utilisation :

// let light = TrafficLight::Red;
// println!("Durée : {} secondes", light.duration()); // 30 secondes
// 🚦 Attention aux match !

// -------------------------------------------------------------------------

enum TrafficLight {
    Red,
    Yellow,
    Green
}

impl TrafficLight {
    fn duration(&self) -> usize {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 25
        }
    }
}

fn main() {
let light = TrafficLight::Red;
println!("Durée : {} secondes", light.duration()); // 30 secondes
}