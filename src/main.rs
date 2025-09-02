
use std::env;

#[allow(dead_code)]
mod day1;
mod day2;

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path =  &args[1];
    if let Ok(lines) = day2::to_lines(file_path){
        print!("Number of valid reports {} ", day2::parse_reports(lines));
    }
    else {
        print!("Could not read file");
    }
}

