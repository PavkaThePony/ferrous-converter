use crate::McCoordinates;
use std::io::{self, Write};

pub fn get_single_value(response: &mut String) -> f64{
    response.clear();
    print!("Enter value (float 64bit precision) »» ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(response).expect("Couldn't read line.");

    let response = response.trim().parse::<f64>();
    match response {
        Ok(value)  => value,
        Err(_) => {
            println!("Failed to parse the number. Returning 0. Did you enter something besides a number?");
            0_f64
        }
    }
}

pub fn get_coordinates(response: &mut String) -> McCoordinates {
    loop {
        response.clear();
        println!("Enter coordinates (delimit with whitespace)");
        print!(" x y z »» ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(response).expect("Couldn't read line");
        
        // core::str:split_whitespace() automatically looks for any whitespaces, including \n,
        // so no .trim() is needed
        let mut iter = response.split_whitespace();
        let x = match iter.next() {
            Some(x) => x,
            None => {
                println!("Invalid input.");
                continue;
            }
        };
        let y = match iter.next() {
            Some(y) => y,
            None => {
                println!("Invalid input.");
                continue;
            }
        };
        let z = match iter.next() {
            Some(z) => z,
            None => {
                println!("Invalid input.");
                continue;
            }
        };
        let x = match x.parse::<f64>() {
            Ok(x) => x,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };
        let y = match y.parse::<f64>() {
            Ok(y) => y,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };
        let z = match z.parse::<f64>() {
            Ok(z) => z,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };
        let coords = McCoordinates { x, y, z };
        // loop {...} never stops on its own, i have to explicitly return.
        return coords
    }
}

