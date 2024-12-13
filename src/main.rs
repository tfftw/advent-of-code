use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::fs::read_to_string;
use std::collections::HashMap;

fn apply_rules(stone: &i64) -> Vec<i64> {

    let mut new_stones = Vec::new();
    let stone_as_string = stone.to_string();
    let string_length = stone_as_string.len();
    if *stone == 0{
        new_stones.push(1);
    }
    else if string_length % 2 == 0{
        new_stones.push(stone_as_string.get(..string_length/2).unwrap().parse::<i64>().unwrap());
        new_stones.push(stone_as_string.get(string_length/2..).unwrap().parse::<i64>().unwrap())
    }
    else {

        new_stones.push(stone*2024)}


    new_stones
}

fn len_after_iterations(stone: &i64, iterations: &i64, register: &mut HashMap<(i64, i64), i64>) -> i64 {
    let mut len = 0;

    match register.get(&(*stone, *iterations)) {
        Some(value) => 
        {len = *value;},
        None => 
    if *iterations != 0{
        let new_stones = apply_rules(stone);
        for new_stone in new_stones{
            len += len_after_iterations(&new_stone, &(iterations - 1), register)
        }
        register.insert((*stone, *iterations), len);
    }
    else{len = 1}}
    
    len
}


fn main() -> ()  {
    let mut stones = vec![475449, 2599064, 213, 0, 2, 65, 5755, 51149];
    let mut register : HashMap<(i64, i64), i64> = HashMap::new();
    let num_iterations = 75;
    let mut len = 0;
    for stone in stones{
        let len_of_stone = len_after_iterations(&stone, &num_iterations, &mut register);
        println!("{:?}, {:?}", stone, len_of_stone);
        len += len_of_stone
    }

    println!("{:?}", len)
}

