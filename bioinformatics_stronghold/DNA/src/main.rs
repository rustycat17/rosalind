use std::fs;

fn main() {
    let filepath = "data/rosalind_dna.txt";
    let input = fs::read_to_string(filepath);
    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("cannot read input file, error message {}", err),
    };

    let mut output = (0, 0, 0, 0);

    for symbol in input.chars() {
        match symbol {
            'A' => output.0 += 1,
            'C' => output.1 += 1,
            'G' => output.2 += 1,
            'T' => output.3 += 1,
            _ => panic!("unknown character {}", symbol),
        }
    }

    println!("{} {} {} {}", output.0, output.1, output.2, output.3);
}
