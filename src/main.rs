use std::time::Instant;

mod helpers;
mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d20;

fn main() {
    println!("Day\t\truntime\tresult");
    let start_all = Instant::now();

    let start = Instant::now();
    let result = d01::get_solution_1();
    println!("Day  1.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d01::get_solution_2();
    println!("Day  1.2\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d02::get_solution_1();
    println!("Day  2.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d02::get_solution_2();
    println!("Day  2.2\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d03::get_solution_1();
    println!("Day  3.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d03::get_solution_2();
    println!("Day  3.2\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d04::get_solution_1();
    println!("Day  4.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d04::get_solution_2();
    println!("Day  4.2\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d05::get_solution_1();
    println!("Day  5.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d05::get_solution_2();
    println!("Day  5.2\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d06::get_solution_1();
    println!("Day  6.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d06::get_solution_2();
    println!("Day  6.2\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d07::get_solution_1();
    println!("Day  7.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d07::get_solution_2();
    println!("Day  7.2\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d08::get_solution_1();
    println!("Day  8.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d08::get_solution_2();
    println!("Day  8.2\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d09::get_solution_1();
    println!("Day  9.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d09::get_solution_2();
    println!("Day  9.2\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d10::get_solution_1();
    println!("Day 10.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d10::get_solution_2();
    println!("Day 10.2\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d11::get_solution_1();
    println!("Day 11.1\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d11::get_solution_2();
    println!("Day 11.2\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d12::get_solution_1();
    println!("Day 12.1\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d12::get_solution_2();
    println!("Day 12.2\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d13::get_solution_1();
    println!("Day 13.1\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d13::get_solution_2();
    println!("Day 13.2\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d14::get_solution_1();
    println!("Day 14.1\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d14::get_solution_2();
    println!("Day 14.2\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d15::get_solution_1();
    println!("Day 15.1\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d15::get_solution_2();
    println!("Day 15.2\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d16::get_solution_1();
    println!("Day 16.1\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d16::get_solution_2();
    println!("Day 16.2\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d17::get_solution_1();
    println!("Day 17.1\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d18::get_solution_1();
    println!("Day 18.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d18::get_solution_2();
    println!("Day 18.2\t{}us\t{}", start.elapsed().as_micros(), result);
    
    let start = Instant::now();
    let result = d20::get_solution_1();
    println!("Day 20.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d20::get_solution_2();
    println!("Day 20.2\t{}us\t{}", start.elapsed().as_micros(), result);

    println!("\nTotal runtime: {}ms", start_all.elapsed().as_millis());
}
