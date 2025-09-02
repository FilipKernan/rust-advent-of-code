

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
        let mut last_value: i32 = 0;
        for (i, &level) in levels.iter().enumerate(){
            if i == 0 {
                match level.parse::<i32>() {
                    Ok(number) => {
                        last_value = number;
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
                        let difference_within_values = (number - last_value).abs();
                        if difference_within_values > 3 || difference_within_values == 0 {
                            status = ReportType::Invalid;
                            break;
                        } 
                        match status {
                            ReportType::None => {
                                if number > last_value {
                                    status = ReportType::Increasing;
                                }
                                else {
                                    status = ReportType::Decreasing;
                                }
                                last_value = number;
                                continue;
                            }
                            ReportType::Increasing => {
                                if last_value > number {
                                    status = ReportType::Invalid;
                                    break;
                                }
                                last_value = number;
                            },
                            ReportType::Decreasing => {
                                if last_value < number {
                                    status = ReportType::Invalid;
                                    break;
                                }
                                last_value = number;
                            },
                            ReportType::Invalid => {
                                break;
                            }
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
        }
    }

    num_of_valid_reports

}
