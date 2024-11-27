use polars::prelude::*;

fn main() {
    let mut df: DataFrame = CsvReadOptions::default()
        .with_has_header(false).try_into_reader_with_file_path(Some("input_1.txt".into()))
        .unwrap()
        .finish()
        .unwrap();

    println!("{:?}", df);
