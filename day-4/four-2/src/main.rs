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

    let mut count = (0..cards.len()).map(|_| 1u32).collect::<Vec<_>>();
    for (index, card) in cards.iter().enumerate() {
        //Number of cards to affect
        let v = solve_card(card);
        for i in index + 1..=(index + v) {
            if i > count.len() {
                break;
            }
            let old = count.get(i).unwrap();
            *(count.get_mut(i).unwrap()) = old + count.get(index).unwrap();
        }
    }

    let sum: u32 = count.iter().sum();
    println!("{sum}");
}

fn solve_card(card: &(Vec<u32>, Vec<u32>)) -> usize {
    let mut total = 0;
    for number in &card.1 {
        for winning in &card.0 {
            if number == winning {
                total += 1;
                break;
            }
        }
    }
    total
}
