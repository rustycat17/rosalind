use std::fs;

fn main() {
    let filepath = "data/rosalind_rna.txt";
    let input = fs::read_to_string(filepath);
    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("cannot read file, error message: {}", err),
    };

    let output = input.replace("T", "U");
    println!("{}", output);
}
