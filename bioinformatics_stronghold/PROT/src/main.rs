use std::collections::HashMap;
use std::fs;

fn main() {
    let codon_table = build_codon_table();

    let filepath = "data/input.txt";
    let input = fs::read_to_string(filepath);

    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };

    let mut triplet = String::new();
    let mut result = String::new();

    for c in input.chars() {
        triplet.push(c);
        if triplet.len() == 3 {
            let current_codon =  &codon_table[&triplet];
            if current_codon == "*" {
                break;
            }

            result.push_str(&current_codon);
            triplet = String::new();
        }
    }

    println!("{}", result);

}

fn build_codon_table() -> HashMap<String, String> {
    let mut codon_table = HashMap::new();

    codon_table.insert("UUU".to_string(), "F".to_string());
    codon_table.insert("UUC".to_string(), "F".to_string());
    codon_table.insert("UUA".to_string(), "L".to_string());
    codon_table.insert("UUG".to_string(), "L".to_string());

    codon_table.insert("UCU".to_string(), "S".to_string());
    codon_table.insert("UCC".to_string(), "S".to_string());
    codon_table.insert("UCA".to_string(), "S".to_string());
    codon_table.insert("UCG".to_string(), "S".to_string());

    codon_table.insert("UAU".to_string(), "Y".to_string());
    codon_table.insert("UAC".to_string(), "Y".to_string());
    codon_table.insert("UAA".to_string(), "*".to_string());
    codon_table.insert("UAG".to_string(), "*".to_string());

    codon_table.insert("UGU".to_string(), "C".to_string());
    codon_table.insert("UGC".to_string(), "C".to_string());
    codon_table.insert("UGA".to_string(), "*".to_string());
    codon_table.insert("UGG".to_string(), "W".to_string());

    codon_table.insert("CUU".to_string(), "L".to_string());
    codon_table.insert("CUC".to_string(), "L".to_string());
    codon_table.insert("CUA".to_string(), "L".to_string());
    codon_table.insert("CUG".to_string(), "L".to_string());

    codon_table.insert("CCU".to_string(), "P".to_string());
    codon_table.insert("CCC".to_string(), "P".to_string());
    codon_table.insert("CCA".to_string(), "P".to_string());
    codon_table.insert("CCG".to_string(), "P".to_string());

    codon_table.insert("CAU".to_string(), "H".to_string());
    codon_table.insert("CAC".to_string(), "H".to_string());
    codon_table.insert("CAA".to_string(), "Q".to_string());
    codon_table.insert("CAG".to_string(), "Q".to_string());

    codon_table.insert("CGU".to_string(), "R".to_string());
    codon_table.insert("CGC".to_string(), "R".to_string());
    codon_table.insert("CGA".to_string(), "R".to_string());
    codon_table.insert("CGG".to_string(), "R".to_string());

    codon_table.insert("AUU".to_string(), "I".to_string());
    codon_table.insert("AUC".to_string(), "I".to_string());
    codon_table.insert("AUA".to_string(), "I".to_string());
    codon_table.insert("AUG".to_string(), "M".to_string());

    codon_table.insert("ACU".to_string(), "T".to_string());
    codon_table.insert("ACC".to_string(), "T".to_string());
    codon_table.insert("ACA".to_string(), "T".to_string());
    codon_table.insert("ACG".to_string(), "T".to_string());

    codon_table.insert("AAU".to_string(), "N".to_string());
    codon_table.insert("AAC".to_string(), "N".to_string());
    codon_table.insert("AAA".to_string(), "K".to_string());
    codon_table.insert("AAG".to_string(), "K".to_string());

    codon_table.insert("AGU".to_string(), "S".to_string());
    codon_table.insert("AGC".to_string(), "S".to_string());
    codon_table.insert("AGA".to_string(), "R".to_string());
    codon_table.insert("AGG".to_string(), "R".to_string());

    codon_table.insert("GUU".to_string(), "V".to_string());
    codon_table.insert("GUC".to_string(), "V".to_string());
    codon_table.insert("GUA".to_string(), "V".to_string());
    codon_table.insert("GUG".to_string(), "V".to_string());

    codon_table.insert("GCU".to_string(), "A".to_string());
    codon_table.insert("GCC".to_string(), "A".to_string());
    codon_table.insert("GCA".to_string(), "A".to_string());
    codon_table.insert("GCG".to_string(), "A".to_string());

    codon_table.insert("GAU".to_string(), "D".to_string());
    codon_table.insert("GAC".to_string(), "D".to_string());
    codon_table.insert("GAA".to_string(), "E".to_string());
    codon_table.insert("GAG".to_string(), "E".to_string());

    codon_table.insert("GGU".to_string(), "G".to_string());
    codon_table.insert("GGC".to_string(), "G".to_string());
    codon_table.insert("GGA".to_string(), "G".to_string());
    codon_table.insert("GGG".to_string(), "G".to_string());

    codon_table
}
