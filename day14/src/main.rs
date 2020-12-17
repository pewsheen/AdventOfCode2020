use day14::parser::read_lines;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part2() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut mask = String::new();
        let mut mmap: HashMap<u64, u64> = HashMap::new();

        for line in lines {
            if let Ok(text) = line {
                let split = text.split(" = ").collect::<Vec<&str>>();
                if split[0] == "mask" {
                    mask = split[1].to_string();
                } else {
                    let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
                    let cap = re.captures(&text).unwrap();

                    let float_addr: String = format!("{:036b}", cap[1].parse::<u64>().unwrap());
                    let value: u64 = cap[2].parse::<u64>().unwrap();
                    let addrs: Vec<u64> = mem_combination(&float_addr, &mask);

                    for addr in addrs {
                        mmap.insert(addr, value);
                    }
                }
            }
        }

        let mut sum: u64 = 0;
        for (_, value) in mmap {
            sum += value;
        }

        println!("sum :{}", sum);
    }
}

fn mem_combination(addr: &String, mask: &String) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let mut float_pos: Vec<usize> = vec![];
    let mut base: String = addr.clone();

    mask.chars().enumerate().for_each(|(index, digit)| {
        if digit == 'X' {
            float_pos.push(index);
        } else if digit == '1' {
            base.replace_range(index..index+1, "1");
        }
    });

    let float_count = float_pos.len();
    for n in 0 .. 2u64.pow(float_count as u32) {
        let b = format!("{:01$b}", n, float_count).clone();
        let mut m: String = base.clone();

        for index in 0..float_count {
            let pos: usize = float_pos[index];
            let digit: &str = &b.chars().nth(index).unwrap().to_string();
            m.replace_range(pos..pos+1, digit);
        }
        result.push(u64::from_str_radix(&m, 2).unwrap());
    }

    result
}

fn part1() {
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
