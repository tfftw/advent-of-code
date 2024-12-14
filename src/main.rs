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


fn main() -> ()  {
    let input = read_lines("../input_advent/input_14.txt", ' '.to_string());
    let num_robots = input.len() as i32;

    let size_x = 101;
    let size_y = 103;
    let mut px: HashMap<(i32), i32> = HashMap::new();
    let mut py: HashMap<(i32), i32> = HashMap::new();
    let mut vx: HashMap<(i32), i32> = HashMap::new();
    let mut vy: HashMap<(i32), i32> = HashMap::new();

    for (index, value) in input.iter().enumerate()
        {let p = &value[0].get(2..).unwrap().split(",").collect::<Vec<_>>();
        let v = &value[1].get(2..).unwrap().split(",").collect::<Vec<_>>();
        px.insert(index.try_into().unwrap(), p[0].parse::<i32>().unwrap());
        py.insert(index.try_into().unwrap(), p[1].parse::<i32>().unwrap());
        vx.insert(index.try_into().unwrap(), v[0].parse::<i32>().unwrap());
        vy.insert(index.try_into().unwrap(), v[1].parse::<i32>().unwrap());
    }

    // update 100 seconds
    let gap = 100 ;
    for robot in 0..num_robots{
        let new_px = (((px.get(&robot).unwrap() + gap*vx.get(&robot).unwrap()) % size_x) + size_x) % size_x; 
        let new_py = (((py.get(&robot).unwrap() + gap*(vy.get(&robot).unwrap())) % size_y) + size_y) % size_y; 
        px.insert(robot, new_px);
        py.insert(robot, new_py);
    }

    let mut num_quadrant_left_down = 0;
    let mut num_quadrant_right_down = 0;
    let mut num_quadrant_left_up = 0;
    let mut num_quadrant_right_up = 0;

    println!("px : {:?}", px);
    println!("py: {:?}", py);
    for robot in 0..num_robots{
        let left: bool = (*px.get(&robot).unwrap() <= (size_x - 3)/2);
        let right: bool = (*px.get(&robot).unwrap() >= (size_x + 1)/2);
        let down: bool = (*py.get(&robot).unwrap() <= (size_y - 3)/2);
        let up: bool = (*py.get(&robot).unwrap() >= (size_y + 1)/2);

        println!("{:?}, {:?}, {:?}, {:?}, {:?}", robot, left, down, right, up);
        if left & down{
            num_quadrant_left_down += 1
        }
        else if left& up {
            num_quadrant_left_up += 1
        }
        else if down & right{
            num_quadrant_right_down += 1
        }
        else if up & right{
            num_quadrant_right_up += 1
        }
    }
    println!("{:?}", num_quadrant_left_down + num_quadrant_left_up + num_quadrant_right_down + num_quadrant_right_up);
    println!("{:?}", num_quadrant_left_down * num_quadrant_left_up * num_quadrant_right_down * num_quadrant_right_up)
}

