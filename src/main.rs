
use std::env;

mod day1;

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path =  &args[1];
    let expect_vec = day1::load_file_to_sorted_vecs(file_path);

    match expect_vec {
        Some((vec_1, vec_2)) => {day1::process_vecs(&vec_1, &vec_2); day1::calculate_sim(&vec_1, &vec_2);},
        None => print!("Failed to load files")
    }
}

