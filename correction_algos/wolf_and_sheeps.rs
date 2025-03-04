fn main() {
    let queue: [&str; 3] = ["sheep", "sheep", "wolf"];
    let array_length = queue.len();

    // Trouver l'index du loup
    let wolf_index = queue.iter().position(|&x| x == "wolf").unwrap(); // On sait qu'il y a toujours un loup

    // Si le loup est à la fin du tableau
    if wolf_index == array_length - 1 {
        println!("Pls go away and stop eating my sheep");
    } else {
        let sheep_number = array_length - (wolf_index + 1);
        println!("Oi! Sheep number {}! You are about to be eaten by a wolf!", sheep_number);
    }
}
