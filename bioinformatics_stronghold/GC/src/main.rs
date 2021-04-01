use regex::Regex;
use std::fs;

#[derive(Clone, Debug)]
struct Sequence {
    header: String,
    sequence: String,
    gc: f64,
}

fn main() {
    let filepath = "data/rosalind_gc.txt";
    let input = fs::read_to_string(filepath);
    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };

    let mut header = String::new();
    let mut sequence = String::new();
    let mut all_sequences = vec![];

    for line in input.lines() {
        // println!("{}", line);
        if line.starts_with('>') {
            if !header.is_empty() {
                let cloned_sequence = sequence.clone();
                add_sequence_entry(header, cloned_sequence, &mut all_sequences);
                sequence = String::new();
            }
            header = line.trim().to_string();
        } else {
            sequence += line.trim();
        }
    }

    add_sequence_entry(header, sequence, &mut all_sequences);

    let (max_header, max_value) = extract_max_gc(all_sequences);
    println!("{}", max_header);
    println!("{}", max_value)
}

fn extract_max_gc(all_sequences: Vec<Sequence>) -> (String, f64) {
    let mut max_value = (String::new(), 0.0);
    for item in all_sequences {
        if max_value.1 < item.gc {
            max_value = (item.header, item.gc);
        }
    }
    max_value
}

fn add_sequence_entry(header: String, sequence: String, all_sequences: &mut Vec<Sequence>) {
    let re = Regex::new(r"[CG]").unwrap();
    let gc = re.find_iter(&sequence).count() as f64 / sequence.len() as f64 * 100.0;

    let current_sequence = Sequence {
        header: header[1..].to_string(),
        sequence,
        gc,
    };
    all_sequences.push(current_sequence);
}
