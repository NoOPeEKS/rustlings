// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => format!("{}{}", first.to_uppercase(), chars.collect::<String>()),
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let words: Vec<_> = words.iter().collect();
    let mut vec_ret: Vec<String> = Vec::new();
    for word in words {
        vec_ret.push(capitalize_first(word));
    }
    println!("{vec_ret:?}");
    vec_ret
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    let words = words.iter().collect::<Vec<_>>();
    
    let mut string_total = String::new();
    for word in words {
       let cap = capitalize_first(word); 
       string_total.push_str(&cap);
    }
    string_total
}

fn main() {
    // You can optionally experiment here.
    println!("{cap}", cap = capitalize_first("fasdf"));
    capitalize_words_vector(&["hello", "world"]);
    let _ = capitalize_words_vector(&["hello", " ", "world"]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
