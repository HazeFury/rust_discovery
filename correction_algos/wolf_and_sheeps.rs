// Wolves have been reintroduced to Great Britain. You are a sheep farmer, and are now plagued by wolves which pretend to be sheep. Fortunately, you are good at spotting them.

// Warn the sheep in front of the wolf that it is about to be eaten. Remember that you are standing at the front of the queue which is at the end of the array:

// [sheep, sheep, sheep, sheep, sheep, wolf, sheep, sheep]      (YOU ARE HERE AT THE FRONT OF THE QUEUE)
//    7      6      5      4      3            2      1
// If the wolf is the closest animal to you, return "Pls go away and stop eating my sheep". Otherwise, return "Oi! Sheep number N! You are about to be eaten by a wolf!" where N is the sheep's position in the queue.

// Note: there will always be exactly one wolf in the array.

// Examples
// Input: ["sheep", "sheep", "sheep", "wolf", "sheep"]
// Output: "Oi! Sheep number 1! You are about to be eaten by a wolf!"

// Input: ["sheep", "sheep", "wolf"]
// Output: "Pls go away and stop eating my sheep"

// ----------------------------------------------------------------------------

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
