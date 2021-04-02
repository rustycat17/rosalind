use std::fs;

fn main() {
    let filepath = "data/rosalind_subs.txt";
    let input = fs::read_to_string(filepath);

    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };

    let out:Vec<&str> = input.split("\n").collect();
    let sequence = out[0];
    let pattern = out[1];
    let mut res:Vec<usize> = vec![];

    let mut offset = 0;
    loop {
        let index = sequence[offset..].find(pattern);
        let matched_index = match index {
            Some(matched_index) => matched_index + 1 + offset,
            None => break,
        };
        offset = matched_index;
        res.push(offset);
    }

    println!("{:?}", res);
}
