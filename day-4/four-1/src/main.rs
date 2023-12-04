fn main() {
    let cards = include_str!("./input.txt")
        .lines()
        .map(|i| {
            let mut it = i.split(':').nth(1).unwrap().split('|');
            (
                it.next()
                    .unwrap()
                    .trim()
                    .replace("  ", " ")
                    .split(' ')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
                it.next()
                    .unwrap()
                    .trim()
                    .replace("  ", " ")
                    .split(' ')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for card in &cards {
        let mut value = None;
        for number in &card.1 {
            for winning in &card.0 {
                if number == winning {
                    if value.is_some() {
                        value = Some(value.unwrap() << 1u32);
                    } else {
                        value = Some(1)
                    }
                    break;
                }
            }
        }
        sum += value.unwrap_or(0);
    }
    println!("{sum}");
}
