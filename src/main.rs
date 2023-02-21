use clap::Parser;

#[derive(Parser)]
struct CLI {
    string: String,
    rot: Option<usize>
}

fn rot(s: String, r: usize) -> String {
    let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
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

#[cfg(test)]
mod rot {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty_rot() {
        assert_eq!(rot(String::from("abc"), 13), String::from("nop"));
    }

    #[test]
    fn test_number_rot() {
        assert_eq!(rot(String::from("abc"), 1), String::from("bcd"));
    }

}


fn main() {
    let args = CLI::parse();
    
    if let Some(n) = args.rot {
        println!("{:?}", rot(args.string, n));
    } else {
        println!("{:?}", rot(args.string, 13));
    }

}
