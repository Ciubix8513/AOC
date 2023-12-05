use std::collections::hash_set::HashSet;

fn main() {
    let mut input = include_str!("./input.txt").split("\n\n");
    let seeds = input
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let seeds = seeds
        .chunks(2)
        .map(|i| (i[0]..=(i[0] + i[1])).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    println!("Total of {} seeds", seeds.len());

    let maps = input
        .map(|i| {
            let mut o = i
                .lines()
                .skip(1)
                .map(|i| {
                    let mut data = i.split(' ');
                    (
                        data.next().unwrap().parse::<i64>().unwrap(),
                        data.next().unwrap().parse::<i64>().unwrap(),
                        data.next().unwrap().parse::<i64>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            o.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            o
        })
        .collect::<Vec<_>>();

    let mut o = Vec::new();
    // let mut map = HashSet::new();

    for s in seeds {
        // if map.contains(&s) {
        //     // println!("Duplicate");
        //     continue;
        // }
        let mut input = s;
        for m in &maps {
            for i in m {
                if in_range(input, i.1, i.2) {
                    let offset = i.0 - i.1;
                    input = input + offset;
                    break;
                }
            }
        }
        // map.insert(input);
        o.push(input);
    }
    o.sort();
    println!("{}", o.first().unwrap());
}

fn in_range<T>(value: T, start: T, len: T) -> bool
where
    T: PartialEq
        + std::ops::Add<T>
        + std::cmp::PartialEq
        + std::cmp::PartialOrd
        + From<<T as std::ops::Add<T>>::Output>,
{
    value > start && value < (start + len).into()
}
