use std::fs;

fn main() {
    let file_content = fs::read_to_string("input.txt")
        .expect("Couldn't find input file.");
    let lines = file_content
        .split("\n")
        .collect::<Vec<_>>();

    let mut elves_calories: Vec<u32> = Vec::new();
    let mut calories_accumulator: u32 = 0;

    for line in lines {
        match line.parse::<u32>() {
            Ok(calories) => calories_accumulator += calories,
            Err(_) => {
                elves_calories.push(calories_accumulator);
                calories_accumulator = 0;
            }
        }
    }

    if calories_accumulator > 0
    {
        elves_calories.push(calories_accumulator);
    }

    elves_calories
        .sort_by(|a, b| b.partial_cmp(a)
            .expect("Not comparable."));

    let most_calories = elves_calories[0];

    println!("part 1: {}", most_calories);

    
    let top_three: u32 = elves_calories
        .iter()
        .take(3)
        .sum();

    println!("part 2: {}", top_three);
}
