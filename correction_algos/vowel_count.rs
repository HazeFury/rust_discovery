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
       
       