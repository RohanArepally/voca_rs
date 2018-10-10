//! Returns the index of `search` in `subject`.

use split;
/// Returns the first occurrence index of `search` in `subject` or -1 if not found.
///
/// # Arguments
///
/// * `subject` - The string where to search.
/// * `search` - The string to search.
/// * `from_index` - The index to start searching
///
/// # Example
/// ```
/// use voca_rs::*;
/// index::index_of("morning", "n", 0);
/// // => 3
/// index::index_of("Zażółć gęślą jaźń", "gęślą", 0);
/// // => 7
/// index::index_of("evening", "o", 0);
/// // => -1
pub fn index_of(subject: &str, search: &str, from_index: usize) -> i8 {
    match search.len() {
        0 => 0,
        _ => {
            let string_slice = &subject[subject.char_indices().nth(from_index).unwrap().0..];
            match split::chars(string_slice)
                .iter()
                .enumerate()
                .position(|(pos, _)| {
                    match &string_slice[string_slice.char_indices().nth(pos).unwrap().0..]
                        .find(search)
                    {
                        Some(x) => *x == 0,
                        None => false,
                    }
                }) {
                Some(x) => x as i8,
                None => -1,
            }
        }
    }
}
