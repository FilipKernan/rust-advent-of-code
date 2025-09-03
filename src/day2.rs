

use std::{fs, io::{self, BufRead}, path::Path, fmt};


pub fn to_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq)]
enum ReportType {
    None,
    Increasing,
    Decreasing,
    Invalid
}

impl fmt::Display for ReportType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReportType::None => write!(f, "None"),
            ReportType::Increasing => write!(f, "Increasing"),
            ReportType::Decreasing => write!(f, "Decreasing"),
            ReportType::Invalid => write!(f, "Invalid")
        }
    }
}



pub fn parse_reports(reports: io::Lines<io::BufReader<fs::File>>) -> i32
{
    let mut num_of_valid_reports = 0;
    for report in reports.map_while(Result::ok){
        let levels: Vec<&str> = report.split_whitespace().collect();
        let mut status: ReportType = ReportType::None;
        let mut last_values: Vec<i32> = vec![0];
        let mut invalid_num: u8 = 0;
        let mut can_try_again = true;
        for (i, &level) in levels.iter().enumerate(){
            if i == 0 {
                match level.parse::<i32>() {
                    Ok(number) => {
                        last_values = vec![number];
                        continue;
                    },
                    Err(e) => {
                        print!("Error: {}", e);
                    }
                }
            }
            else {
                match level.parse::<i32>() {
                    Ok(number) => {
                        let mut invalid_match = 0;
                        let mut new_level: Option<i32> = None;
                        for last_value in last_values.iter() {
                            let current_status = test_level(&status, last_value, &number);
                            match current_status {
                                (ReportType::None, _) => {
                                    invalid_match += 1;
                                }
                                (new_status, new_value) if new_status == status => {
                                    new_level = Some(new_value);
                                }
                                (new_status, new_value) if status == ReportType::None && new_status != ReportType::Invalid => {
                                    new_level = Some(new_value);
                                }
                                (new_status, new_value) => {
                                    invalid_match += 1;
                                }
                            }
                        }
                        if invalid_match == last_values.len() {
                            if invalid_match == 1 && can_try_again {
                                last_values.push(number);
                                can_try_again = false;
                            } else {
                                invalid_num += 1;
                            }
                        } else {
                            match new_level {
                                Some(value) => {
                                    last_values = vec![value];
                                },
                                None => {}
                            }
                        }
                        if invalid_num >= 2 {
                            status = ReportType::Invalid;
                        }
                    },
                    Err(e) => {
                        status = ReportType::None;
                        print!("Error: {}", e);
                    }
                }
            }
            

        }
        if status != ReportType::Invalid
        {
            num_of_valid_reports += 1;
            print!("Valid report {} \n", report);
        }
        else {
            // print!("Invalid report {} \n", report);
        }
    }

    num_of_valid_reports

}

fn test_level(status: &ReportType, last_value: &i32, level: &i32) -> (ReportType, i32) {
    let difference_within_values = (level - last_value).abs();
    match status {
        ReportType::None => {
            if difference_within_values > 3 || difference_within_values == 0 {
                return (ReportType::Invalid, *level)
            } 
            if level > last_value {
                return (ReportType::Increasing, *level)
            }
            else {
                return (ReportType::Decreasing, *level)
            }
        }
        ReportType::Increasing => {
            if last_value > level || difference_within_values > 3 || difference_within_values == 0 {
                return (ReportType::Invalid, *level)
            } else {
                return (ReportType::Increasing, *level)
            }
        },
        ReportType::Decreasing => {
            if last_value < level || difference_within_values > 3 || difference_within_values == 0 {
                return (ReportType::Invalid, *level)
            } else {
                return (ReportType::Decreasing, *level);
            }
        },
        ReportType::Invalid => {
            (ReportType::Invalid, *level)
        }
    }
}


/*
 This was reported as valid when it is not 75 70 69 67 65 63 59 It has 2 violations
 This one is the same: Valid report 9 10 13 11 9 8
*/

