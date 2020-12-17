use day14::parser::read_lines;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut mask = String::new();
        let mut mmap: HashMap<u64, String> = HashMap::new();

        for line in lines {
            if let Ok(text) = line {
                let split = text.split(" = ").collect::<Vec<&str>>();
                if split[0] == "mask" {
                    mask = split[1].to_string();
                } else {
                    let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
                    let cap = re.captures(&text).unwrap();

                    let addr: u64 = cap[1].parse().unwrap();
                    let value = format!("{:036b}", cap[2].parse::<u64>().unwrap());

                    let value = value
                        .chars()
                        .zip(mask.chars())
                        .map(|(v, m)| if m == 'X' { v } else { m })
                        .collect();

                    mmap.insert(addr, value);
                }
            }
        }

        let mut sum: u64 = 0;
        for (_, value) in mmap {
            sum += u64::from_str_radix(&value, 2).unwrap();
        }

        println!("sum :{}", sum);
    }
}
