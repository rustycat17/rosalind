use std::fs;

fn main() {
    let filepath = "data/input.txt";
    let input = fs::read_to_string(filepath);
    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };


    for line in input.lines() {
        println!("{}", line);
    }

}
