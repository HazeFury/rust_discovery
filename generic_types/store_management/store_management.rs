#[allow(dead_code)]
#[derive(Debug)]

struct Product<T> {
    name: String,
    price: f64,
    stock: u32,
    category: T
}

    struct Store<T> {
        products: Vec<Product<T>>
    }

impl<T> Store<T> {
    fn find_product(&mut self, name: &str) -> Option<&mut Product<T>> {
    self.products.iter_mut().find(|p| p.name == name)
}

    fn buy_product(&mut self, name: &str, quantity: u32) -> Result<f64, String> {
       if let Some(product_to_buy) = self.find_product(name) {
        if product_to_buy.stock >= quantity {
            product_to_buy.stock -= quantity;
          return Ok(product_to_buy.price * quantity as f64);
        } else {
           return Err(String::from("il n'y a plus assez de stock de ce produit !"));
        }} else {
           Err(String::from("Aucun produit ne correspond à votre recherche"))
        }
       
    }
    
}

fn main() {
    let mut store = Store {
        products: vec![
            Product { name: String::from("Pomme"), price: 0.5, stock: 10, category: "Alimentaire" },
            Product { name: String::from("Ordinateur"), price: 999.99, stock: 3, category: "Électronique" },
        ],
    };

    // Recherche un produit existant
     if let Some(found) = store.find_product("Pomme") {
    println!("Produit trouvé : {:?}", found);
    } else {
    println!("Produit non trouvé !");
    }
    // Achat réussi
    match store.buy_product("Pomme", 5) {
        Ok(total) => println!("Achat réussi ! Total: {}€", total),
        Err(e) => println!("Erreur: {}", e),
    }

    // Achat échoué (stock insuffisant)
    match store.buy_product("Ordinateur", 10) {
        Ok(total) => println!("Achat réussi ! Total: {}€", total),
        Err(e) => println!("Erreur: {}", e),
    }
}
