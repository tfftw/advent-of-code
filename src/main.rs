use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str, sep: String) -> Vec<Vec<String>> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.split(&sep).map(|s| s.to_string()).collect::<Vec<_>>())
    }

    result
}

fn check_code(code: &Vec<String>, rule: &Vec<String>) -> bool {
    let mut second_number_found = false;
    let mut correct = true;
    for number in code {
        if number == &rule[1]{
            second_number_found = true;
        }
        else if number == &rule[0]{
            if second_number_found
            {correct = false}
            else {
                break}
        }

    }

    correct
}

fn correct_code(code: &Vec<String>, rules: &Vec<Vec<String>>) -> (Vec<String>, bool){

    let mut temp_code = code.clone();

    let mut new_code: Vec<String>;
    let mut corrected_anything_at_all = false;
    while true{
        let mut corrected_something = false;
        for rule in rules{
            if !check_code(&temp_code, rule){
                // Fix code
                new_code = Vec::new();

                for number in &temp_code {
                    if *number == rule[1]{
                        new_code.push(rule[0].clone());
                    }
                    else if *number == rule[0]{
                        new_code.push(rule[1].clone());
                    }
                    else {
                        new_code.push(number.to_string())
                    }            
                }
                temp_code = new_code.clone();
                corrected_something = true;
                corrected_anything_at_all = true
        }
    }
        if !corrected_something{
            break}
    }

    (temp_code, corrected_anything_at_all)
}



fn main() -> Result<(), Box<dyn Error>>  {
    let mut rules = read_lines("../input_advent/input_2024_05_00.txt", '|'.to_string());
    let codes = read_lines("../input_advent/input_2024_05_01.txt", ','.to_string());

    let mut codes_to_check = codes.clone();
    let mut corrected_codes: Vec<Vec<String>> = Vec::new();
    let mut relevant_rules_per_code: HashMap<Vec<String>, Vec<Vec<String>>> = HashMap::new();

    for code in codes{
        let mut relevant_rules: Vec<Vec<String>> = Vec::new();
        for rule in &rules{
            if code.contains(&rule[0]) && code.contains(&rule[1]){
                relevant_rules.push(rule.to_vec())}}

        relevant_rules_per_code.insert(code.clone(), relevant_rules.clone());
    }


    for code in codes_to_check{
        let relevant_rules = &relevant_rules_per_code[&code];
        if relevant_rules.len() > 0{
            let (corrected_code, corrected_anything_at_all) = correct_code(&code, relevant_rules);
            if corrected_anything_at_all{corrected_codes.push(corrected_code)
            }
        }
    }

    println!("{:?}", corrected_codes.len());

    // Select middle number
    let mut selected_numbers: Vec<i32> = Vec::new();
    for code in corrected_codes{
        selected_numbers.push(code[(code.len() - 1)/2].parse::<i32>().unwrap())
    }

    println!("{:?}", selected_numbers);


    let sum: i32   = selected_numbers.iter().sum();

    println!("{:?}", sum);


    Ok(())
}