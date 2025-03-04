// Énoncé :
// Écris une fonction qui prend une chaîne de caractères en entrée et retourne le nombre de voyelles (a, e, i, o, u) qu’elle contient.

// Exemples :
// count_vowels("hello") ➞ 2  
// count_vowels("rust") ➞ 1  
// count_vowels("xyz") ➞ 0  
// count_vowels("aeiou") ➞ 5  

// Instructions :

// La fonction doit retourner un entier (usize).
// Les voyelles sont : a, e, i, o, u (uniquement en minuscule, pas besoin de gérer les majuscules pour l’instant).
// Tu peux utiliser une boucle (for), un if ou un **match` pour vérifier chaque caractère.

// ----------------------------------------------------------------------------


fn count_vowels(input: &str) -> usize {
    
    let mut result : usize = 0;
    
    
     for element in input.chars() {
     match element {
         'a' | 'e' | 'i' | 'o' | 'u' => {
             result = result + 1;
         }
         _ => ()
       }
     }
    
    print!("{}", result);
    return result;
    }
    
    
    fn main() {
       count_vowels("hello");
    }
    
    // -------------------------------------------

    fn count_vowels(input: &str) -> usize {
        let vowels_array : [char;5] = ['a', 'e', 'i', 'o', 'u'];
        
        let mut result : usize = 0;
        
        for vowel in vowels_array {
         for element in input.chars() {
             if element == vowel {
                 result = result + 1;
             }
         }
        } 
        print!("{}", result);
        return result;
        }
        
        
        fn main() {
           count_vowels("aeiou");
        }
        
    // --------------------------------------

    fn count_vowels(input: &str) -> usize {
        input.chars().filter(|c| "aeiou".contains(*c)).count()
       }
       
       fn main() {
           println!("{}", count_vowels("hello"));
       }
       
       