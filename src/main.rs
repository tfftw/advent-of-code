use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::StringRecord;
use regex::Regex;

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut lines = Vec::new();

    for result in rdr.records() {
        let record = result?;
        lines.push(StringRecord::as_slice(&record).to_owned())
    }

    Ok(lines)
}



fn main() -> Result<(), Box<dyn Error>>  {
    let filename = "../input_advent/input_1.txt";
    let lines = read_csv(filename)?;
    let re = Regex::new(r"[A-Za-z]").unwrap();

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
    Ok(())
}