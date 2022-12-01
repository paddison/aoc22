use std::time::Instant;

mod d01;

fn main() {
    println!("Day\t\truntime\tresult");
    let start_all = Instant::now();

    let start = Instant::now();
    let result = d01::get_solution_1();
    println!("Day  1.1\t{}us\t{}", start.elapsed().as_micros(), result);

    let start = Instant::now();
    let result = d01::get_solution_2();
    println!("Day  1.2\t{}us\t{}", start.elapsed().as_micros(), result);

    println!("\nTotal runtime: {}ms", start_all.elapsed().as_millis());
}
