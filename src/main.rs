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

fn display_robots(num_robots: &i32, px: &HashMap<(i32), i32>, py: &HashMap<(i32), i32>){
    let mut arr: [String; 103] = std::array::from_fn(|_| ".".repeat(101));
    for robot in 0..*num_robots{
        let robot_px = px.get(&robot).unwrap();
        let robot_py = py.get(&robot).unwrap();
        let mut chars: Vec<char> = arr[*robot_py as usize].chars().collect();

        chars[*robot_px as usize] = 'o';
    
        // Convert the Vec<char> back to a String
        let new_string: String = chars.into_iter().collect();

        // Convert back to string and assign it to the array
        arr[*robot_py as usize] = new_string;
    }
    println!(" ");
    for line in arr{
        println!{"{:?}", line}
    }
    println!(" ");
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

    for iteration in 0..200{
    let gap = 1 ;
    for robot in 0..num_robots{
        let new_px = (((px.get(&robot).unwrap() + gap*vx.get(&robot).unwrap()) % size_x) + size_x) % size_x; 
        let new_py = (((py.get(&robot).unwrap() + gap*(vy.get(&robot).unwrap())) % size_y) + size_y) % size_y; 
        px.insert(robot, new_px);
        py.insert(robot, new_py);
    }
    println!("{:?}", iteration);
    display_robots(&num_robots, &px, &py)
    }
    
}

