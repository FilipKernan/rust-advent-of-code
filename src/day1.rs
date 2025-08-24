

use std::{cmp, collections::HashMap, fs};

pub fn load_file_to_sorted_vecs(file_path: &String) -> Option<(Vec<i128>, Vec<i128>)> {

    let file_string = fs::read_to_string(file_path);
    if file_string.is_err() {
        print!("Unable to read file {}", file_path);
        return None;
    }
    let file_string = file_string.unwrap();
    let numbers: Vec<&str>  = file_string.split_whitespace().collect();

    let mut vec_1: Vec<i128> = Vec::new();
    let mut vec_2: Vec<i128> = Vec::new();

    for (i, val) in numbers.iter().enumerate(){
        let is_even: bool = i % 2 == 0;
        let int_val = val.parse::<i128>();
        if int_val.is_err(){
            print!("Failed to parse {}", val);
            return None;
        }
        let int_val = int_val.unwrap();
        if is_even {
            vec_1.push(int_val);
        } else {
            vec_2.push(int_val);
        }

    } 
    vec_1.sort();
    vec_2.sort();
    return Some((vec_1, vec_2))
}

fn diff_between_vecs(vec_1: &Vec<i128>, vec_2: &Vec<i128> ) -> Vec<i128> {

    let min_size = cmp::min(vec_1.len(), vec_2.len());

    let mut result_vec: Vec<i128> = Vec::new();
    for i in 0..min_size {
        result_vec.push((vec_1[i] - vec_2[i]).abs());
    }
   result_vec
}

pub fn process_vecs(vec_1: &Vec<i128>, vec_2: &Vec<i128> ) {
    let diff = diff_between_vecs(vec_1, vec_2);
    print!("Total diff {} sum", diff.iter().sum::<i128>());
}

pub fn calculate_sim(vec_1: &Vec<i128>, vec_2: &Vec<i128>) {
    let mut count_hash = HashMap::new();
    for &val in vec_2 {
        *count_hash.entry(val).or_insert(0) += 1;
    }

    let mut sum: i128 = 0;

    for &val in vec_1 {
      sum += val * count_hash.get(&val).copied().unwrap_or(0);
    }
    print!("Total sim score {}  ", sum);
}


