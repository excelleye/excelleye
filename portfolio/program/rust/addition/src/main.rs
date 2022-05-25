//《練習》足し算プログラム

use std::io;
use std::string::String;

fn main() {
    add();
}

//Addition Program
fn add() {
        println!("Addition program...");

    //First numerical entry.
        println!("Please enter the first number.");
        let mut mystring = String::new();
        io::stdin()
            .read_line(&mut mystring)
            .expect("Input error!");

        let first_num : i64 = match mystring
            .trim()
            .parse(){
                Ok(num) => num,
                Err(_) => 0,
            
            };

    //Second numerical entry.
        println!("Please enter the second number.");
        let mut mystring = String::new();
        io::stdin()
            .read_line(&mut mystring)
            .expect("Input error!");
        
        let second_num : i64 = match mystring
            .trim()
            .parse(){
                Ok(num) => num,
                Err(_) => 0,
            };
    //Output of calculation results.
        let answer =first_num + second_num;

        println!("Calculation Results...");
        println!("{} + {} = {}",first_num,second_num,answer);

 }