use std::fs;

fn item_priority(item: char) -> u32 {
    if item.is_uppercase() {
        (item as u32 - 'A' as u32) + 27
    } else {
        (item as u32 - 'a' as u32) + 1
    }
}

fn main() {
    let file_content = fs::read_to_string("input.txt")
        .expect("Couldn't find input file.");
    let lines = file_content
        .split("\n")
        .collect::<Vec<_>>();

    let mut total_priority: u32 = 0;

    for line in lines.as_slice() {
        let compartments = line.split_at(line.len()/2);
        assert_eq!(compartments.0.len(), compartments.1.len());

        let mut items_in_both: Vec<char> = compartments.0.chars()
            .filter(|c| compartments.1.contains(*c))
            .collect();
        items_in_both.sort();
        items_in_both.dedup();

        if let Some(priority) =  items_in_both.iter()
            .map(|c| item_priority(*c))
            .reduce(|acc, p| acc + p ) {
                total_priority += priority;
        }
    }

    println!("part 1: {:?}", total_priority);

    let group_priorities: Vec<u32> = lines.chunks(3)
        .map(|group| {
            assert_eq!(group.len(), 3);

            let mut group_badge_priority = 0;

            let badge_items: Vec<char> = group[0].chars().filter(|c| 
                group[1].contains(*c) && group[2].contains(*c)).collect();

            if let Some(priority) =  badge_items.iter().take(1)
                .map(|c| item_priority(*c))
                .reduce(|acc, p| acc + p ) {
                    group_badge_priority += priority;
            }

            group_badge_priority
        }).collect();

    let total_group_priority = group_priorities.iter()
        .map(|v| *v)
        .reduce(|acc: u32, p: u32| acc + p).unwrap_or(0);

    println!("part 2: {:?}", total_group_priority);
}
