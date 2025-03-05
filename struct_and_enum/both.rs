// 📝 Énoncé :
// Tu dois modéliser un véhicule qui peut être une voiture ou un vélo.

// Crée une struct Car qui a :

// une marque (brand: String),
// une vitesse (speed: u32).

// Crée une struct Bicycle qui a :

// un type (bicycle_type: String).
// Crée un enum Vehicle qui peut être :

// soit une voiture (Car)
// soit un vélo (Bicycle)


// Implémente une méthode describe pour Vehicle qui affiche :

// "C'est une voiture de marque {brand}, vitesse: {speed} km/h" si c'est une voiture.
// "C'est un vélo de type {bicycle_type}" si c'est un vélo.
// 🔹 Exemples attendus :
// Si tu exécutes ce code :


// fn main() {
//     let my_car = Vehicle::Car(Car { brand: String::from("Toyota"), speed: 120 });
//     let my_bike = Vehicle::Bicycle(Bicycle { bicycle_type: String::from("VTT") });

//     my_car.describe();
//     my_bike.describe();
// }

// La sortie doit être :

// C'est une voiture de marque Toyota, vitesse: 120 km/h
// C'est un vélo de type VTT

// ---------------------------------------------------------------------------------

struct Car {
    brand: String,
    speed: u32,
}

struct Bicycle {
    bicycle_type: String,
}

enum Vehicle {
    Car(Car),
    Bicycle(Bicycle),
}

impl Vehicle {
    fn describe(&self) {
        match self {
            Vehicle::Car(car) => println!(
                "C'est une voiture de marque {}, vitesse: {} km/h",
                car.brand, car.speed
            ),
            Vehicle::Bicycle(bike) => println!("C'est un vélo de type {}", bike.bicycle_type),
        }
    }
}

fn main() {
    let my_car = Vehicle::Car(Car { 
        brand: String::from("Toyota"), 
        speed: 120 
    });
    
    let my_bike = Vehicle::Bicycle(Bicycle { 
        bicycle_type: String::from("VTT") 
    });

    my_car.describe();
    my_bike.describe();
}
