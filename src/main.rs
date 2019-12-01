use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::cmp::max;

//EX1
fn main() {
    let file_name = "/Users/pierangelo.cecchetto/Documents/projects/rust/advent-of-code-20109/resources/ex1-1";
    let lines: Vec<String> = lines_from_file(file_name);

    //PART 1
    let res: i32 = lines.iter()
        .fold(0, |acc, x| {
            fuel_for_mass(str_to_int(x.to_string()) + acc)
        });
    println!("PART 1: The amount of fuel required is {}", res);

    //PART 2
    let res2: i32 = lines.iter()
        .fold(0, |acc, x| {
            fuel_for_mass_rec(str_to_int(x.to_string())) + acc
        });
    println!("PART 2: The amount of fuel required is {}", res2);


}

fn str_to_int(s: String) -> i32 {
    match s.parse::<i32>()  {
        Ok(res) => res,
        Err(_err) => 0
    }
}

fn fuel_for_mass(mass: i32) -> i32 {
    max(mass / 3 - 2, 0)
}

fn fuel_for_mass_rec(mass: i32) -> i32 {
    let fuel_mass = fuel_for_mass(mass);
//    println!("fuel_for_mass: {}: {}", mass, fuel_mass);
    if fuel_mass == 0 {
        0
    } else {
        fuel_mass + fuel_for_mass_rec(fuel_mass)
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}