pub mod day1a;
pub mod day1b;

pub fn exec_day(day: usize, part: usize) {
    match day {
        1 => match part {
            1 => day1a::main(),
            2 => day1b::main(),
            _ => println!("The selected part does not exist or is not implemented yet."),
        },
        _ => println!("The selected day does not exist or is not implemented yet."),
    }
}
