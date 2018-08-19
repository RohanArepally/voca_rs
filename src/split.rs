//! Splits `subject` into an chuncks according to given rules.

/// Splits `subject` into an array of characters.
///
/// # Arguments
///
/// * `string: &str` - The string to split into characters.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// split::chars("cloud");
/// // => ["c", "l", "o", "u", "d"]
/// ```
pub fn chars(string: &str) -> Vec<&str> {
    if string.len() == 0 {
        return vec![""];
    }
    string.split_terminator("").skip(1).collect::<Vec<_>>()
}

/// Splits `subject` into an array of chunks by `separator`.
///
/// # Arguments
///
/// * `string: &str` - The string to split into characters.
/// * `pattern: &str` - The pattern to match the separator.
///
/// # Example
///
/// ```rust
/// use voca_rs::*;
/// split::split("rage against the dying of the light", "");
/// // => ["rage", "against", "the", "dying", "of", "the", "light"]
/// ```
pub fn split(string: &'static str, pattern: &str) -> Vec<&'static str> {
    if string.len() == 0 {
        return vec![""];
    }
    if pattern.len() == 0 {
        return vec![string];
    }
    string.split_terminator(pattern).collect::<Vec<_>>()
}

use unicode_segmentation::UnicodeSegmentation;
/// Splits `subject` into an array of words.
///
/// # Arguments
///
/// * `string: &str` - The string to split into characters.
///
/// # Example
///
/// ```rust
/// use voca_rs::*;
/// split::words("Sześć звёзд are dying");
/// // => ["Sześć", "звёзд", "are", "dying"]
/// split::words("LazyLoad with XMLHttpRequest and snake_case");
/// // => ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
/// ```
pub fn words(s: &str) -> Vec<String> {
    fn transform(v: &str) -> String {
        // https://github.com/withoutboats/heck/blob/master/src/lib.rs
        #[derive(Clone, Copy, PartialEq)]
        enum WordMode {
            /// There have been no lowercase or uppercase characters in the current word.
            Boundary,
            /// The previous cased character in the current word is lowercase.
            Lowercase,
            /// The previous cased character in the current word is uppercase.
            Uppercase,
        }
        let w = v.unicode_words().collect::<Vec<&str>>();
        let string = w.join(" ");
        let mut res = String::new();
        let mut char_indices = string.char_indices().peekable();
        let mut mode = WordMode::Boundary;
        let mut add_space = false;
        while let Some((_, c)) = char_indices.next() {
            if let Some(&(_, next)) = char_indices.peek() {
                let next_mode = if c.is_lowercase() {
                    WordMode::Lowercase
                } else if c.is_uppercase() {
                    WordMode::Uppercase
                } else {
                    mode
                };

                // not uppercase and next is uppercase
                if next_mode == WordMode::Lowercase && next.is_uppercase() {
                    add_space = true;
                    mode = WordMode::Boundary;
                // Otherwise if current and previous are uppercase and next
                // is lowercase, word boundary before
                } else if mode == WordMode::Uppercase && c.is_uppercase() && next.is_lowercase() {
                    res.push_str(" ");
                    mode = WordMode::Boundary;
                // Otherwise no word boundary, just update the mode
                } else {
                    mode = next_mode;
                }
            }

            res.push(c);
            if add_space {
                res.push_str(" ");
                add_space = false;
            }
        }
        res
    }

    let string = s.to_string().replace("-", " ").replace("_", " ");
    let res = transform(&string);
    let return_vector: Vec<String> = res.unicode_words().map(String::from).collect();
    return_vector
}

/// Splits `subject` into an array of graphemes
///
/// # Arguments
///
/// * `string: &str` - The string to split into characters.
///
/// # Example
///
/// ```rust
/// use voca_rs::*;
/// split::graphemes("a̐éö̲\r\n");
/// // => ["a̐", "é", "ö̲", "\r\n"]
/// ```
pub fn graphemes(string: &str) -> Vec<&str> {
    if string.len() == 0 {
        return vec![""];
    }
    UnicodeSegmentation::graphemes(string, true).collect::<Vec<&str>>()
}
