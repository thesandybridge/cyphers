use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// String to rotate.
    #[arg()]
    string: String,

    /// Rotation amount for the cypher.
    #[arg(default_value_t = 13)]
    rot: usize
}

fn main() {
    let args = Args::parse();
    println!("{:?}", cypher::rot(args.string, args.rot));
}
