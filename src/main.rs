use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::fs::read_to_string;

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



fn main() -> Result<(), Box<dyn Error>>  {
    let rules = read_lines("../input_advent/input_2024_05_00.txt", '|'.to_string());
    let codes = read_lines("../input_advent/input_2024_05_01.txt", ','.to_string());


    let mut codes_to_check: Vec<Vec<String>> = Vec::new();
    let mut codes_to_check_next_time = codes.clone();

    for rule in rules {
        codes_to_check = codes_to_check_next_time.clone();
        codes_to_check_next_time = Vec::new();
        println!("{:?}", codes_to_check.len());
        for code in codes_to_check{
    
            let check = check_code(&code, &rule);
    
            if check{
                codes_to_check_next_time.push(code)
            };
        }
    }

    // Select middle number
    let mut selected_numbers: Vec<i32> = Vec::new();
    for code in codes_to_check_next_time{
        selected_numbers.push(code[(code.len() - 1)/2].parse::<i32>().unwrap())
    }

    println!("{:?}", selected_numbers);


    let sum: i32   = selected_numbers.iter().sum();

    println!("{:?}", sum);

    Ok(())
}