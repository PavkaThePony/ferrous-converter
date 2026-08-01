use std::io::{self, Write};

use crate::{ConvResult, McCoordinates};

pub fn print_result(res: &ConvResult){
    println!("┌────────────────────────────────────────┐");
    println!("│ {} {}",             res.value, res.type_m);
    println!("└────────────────────────────────────────┘");
    println!();
    let mut blank = String::new();
    print!("Enter anything to proceed ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut blank).expect("Couldn't read line");
    blank.clear();
}

pub fn print_result_mc_coordinates(coords: &McCoordinates) {
    println!("┌────────────────────────────────────────┐");
    println!("│ x: {}, y: {}, z: {}", coords.x, coords.y, coords.z);
    println!("└────────────────────────────────────────┘");
    println!();
    let mut blank = String::new();
    print!("Enter anything to proceed ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut blank).expect("Couldn't read line");
    blank.clear();
}
