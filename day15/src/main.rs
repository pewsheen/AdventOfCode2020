use std::fs;
use std::collections::HashMap;

fn get_start_numbers() -> Vec<u32> {
  let contents = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
  contents.split(',').map(|n| n.parse::<u32>().unwrap()).collect()
}

fn main() {
  /* target */
  let target_round: u32 = 10;

  /* init */
  let mut seen: HashMap<u32, (u32, u32)> = HashMap::new();
  let mut round = 1;
  let mut last_num = 0;

  for num in get_start_numbers() {
    seen.insert(num, (round, round));
    last_num = num;
    round = round + 1;
  }

  /* brrrr */
  while round <= target_round {
    let turns = seen.get(&last_num);

    if let Some(turns) = turns{
      if turns.0 == turns.1 {
        last_num = 0;
      } else {
        last_num = turns.1 - turns.0;
      }
    }

    seen.entry(last_num).and_modify(|e| *e = (e.1, round)).or_insert((round,round));
    round = round + 1;
  }

  println!("Round {} : {}", target_round, last_num);
}
