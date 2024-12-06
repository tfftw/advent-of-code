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



fn main() -> Result<(), Box<dyn Error>>  {
    let rules = read_lines("../input_advent/input_2024_05_00.txt", '|'.to_string());
    let codes = read_lines("../input_advent/input_2024_05_01.txt", ','.to_string());
    println!("{:?}", rules);
    println!("{:?}", codes);
    /*

    let mut answers = Vec::new();
    let mut my_map = Vec::new();
 
    my_map.push(("one", "1"));
    my_map.push(("two", "2"));
    my_map.push(("three", "3"));
    my_map.push(("four", "4"));
    my_map.push(("five", "5"));
    my_map.push(("six", "6"));
    my_map.push(("seven", "7"));
    my_map.push(("eight", "8"));
    my_map.push(("nine", "9"));


    for line in lines {
        let mut first_number = "".to_owned();
        
        let mut last_number = "".to_owned();

        // forward search for first number
        for i in 0..=line.len() {
            let mut partial_line = line[..i].to_owned();
            for (key, value) in &my_map {
                partial_line = partial_line.replace(key, value);
            }

            let numbers_in_line = re.replace_all(&partial_line, "");

            if numbers_in_line.len() == 1
                {
                first_number = numbers_in_line[..1].to_string();
                break;
                }
        }

        // backward search for last number
        for i in 0..=line.len() {
            let mut partial_line = line[line.len() - i ..].to_owned();
            for (key, value) in &my_map {
                partial_line = partial_line.replace(key, value);
            }

            let numbers_in_line = re.replace_all(&partial_line, "");

            if numbers_in_line.len() == 1
                {
                last_number = numbers_in_line[..1].to_string();
                break;
                }
        }
        
        let answer = first_number.to_owned() + &last_number;
        answers.push(answer.parse::<i32>().unwrap());

    }

    println!("{:?}", answers);
    let sum: i32  = answers.iter().sum();

    println!("{:?}", sum);
    */
    Ok(())
}