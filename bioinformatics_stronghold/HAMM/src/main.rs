use std::fs;

fn main() {
    let filepath = "data/input.txt";
    let input = fs::read_to_string(filepath);
    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };

    let out: Vec<&str> = input.split('\n').collect();
    let sequence1 = out[0];
    let sequence2 = out[1];

    let mut counter = 0;
    for i in sequence1.chars().zip(sequence2.chars()) {
        let (j, k) = i;
        if j != k {
            counter += 1;
        }
    }

    println!("{}", counter);
}
