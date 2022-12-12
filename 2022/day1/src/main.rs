fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let mut most_calories = 0;
    let mut calorie_cnt = 0;
    for line in lines {
        if line.is_empty() {
            // Done adding calories
            if calorie_cnt > most_calories {
                most_calories = calorie_cnt;
            }
            calorie_cnt = 0;
        } else {
            // Still adding calories...
            calorie_cnt += line.parse::<i32>().unwrap();
        }
    }

    println!("MOST CALORIES: {most_calories}")
}

fn part2() {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let mut calories_by_elf: Vec<i32> = vec![];
    let mut calorie_cnt = 0;
    for line in lines {
        if line.is_empty() {
            // Done adding calories
            calories_by_elf.push(calorie_cnt);
            calorie_cnt = 0;
        } else {
            // Still adding calories...
            calorie_cnt += line.parse::<i32>().unwrap();
        }
    }
    calories_by_elf.sort(); // largest calories now at the end
    let top_3_calories = calories_by_elf.pop().unwrap()
        + calories_by_elf.pop().unwrap()
        + calories_by_elf.pop().unwrap();

    println!("TOP 3 CALORIES: {top_3_calories}")
}
