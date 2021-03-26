use std::fs;

fn main() {
    let datapath = "data/rosalind_revc.txt";
    let input = fs::read_to_string(datapath);
    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };

    let mut output = String::new();
    for symbol in input.chars() {
        match symbol {
            'A' => output.push('T'),
            'T' => output.push('A'),
            'C' => output.push('G'),
            'G' => output.push('C'),
            _ => panic!("unknown character: {}", symbol),
        }
    }
    output = output.chars().rev().collect();
    println!("{}", output);
}
