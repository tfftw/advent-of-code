use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::StringRecord;



fn read_csv<P: AsRef<Path>>(filename: P) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut numbers = Vec::new();

    for result in rdr.records() {
        let record = result?;
        numbers.push((StringRecord::as_slice(&record).to_owned()))
    }

    Ok((numbers))
}



fn main() -> Result<(), Box<dyn Error>>  {
    let filename = "../input_advent/input_1_dev.txt";
    let lines = read_csv(filename);

    for line in lines {
        println!("{:?}", line)
    }
    Ok(())
}