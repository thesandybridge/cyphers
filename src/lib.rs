const ALPHABET: [char;26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

/// Rot cyhpher algorithm.
///
/// # Arguments
///
/// * `s` - String to rotate.
/// * `r` - Rotation amount for the cypher.
///
/// # Examples
///
/// ```
/// let rot13 = cypher::rot(String::from("abc"), 13);
/// // test for standard rotation of 13 characters.
/// assert_eq!(rot13, String::from("nop"));
///
/// let custom = cypher::rot(String::from("abc"), 1);
/// // test for custom rotation.
/// assert_eq!(custom, String::from("bcd"));
/// ```
pub fn rot(s: String, r: usize) -> String {
    let alphabet = &ALPHABET;

    let upper_alphabet = alphabet
        .iter()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<_>>();

    let rot = s
        .chars()
        .map(|c| *alphabet.iter()
             .chain(alphabet.iter())
             .chain(upper_alphabet.iter())
             .chain(upper_alphabet.iter())
             .skip_while(|&x| *x != c)
             .nth(r)
             .unwrap_or(&c)
            )
        .collect::<String>();

    return rot;
}
