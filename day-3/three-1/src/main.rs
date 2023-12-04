fn main() {
    let mut input = include_str!("./input.txt").lines().collect::<Vec<&str>>();
    let mut blank = String::new();
    for _ in 0..input.first().unwrap().len() {
        blank.push('.');
    }
    input.insert(0, &blank);
    input.push(&blank);

    let mut sum = 0;

    for w in input.windows(3) {
        let mut i = w.iter().copied();
        let mut t = i.next().unwrap().chars().collect::<Vec<_>>();
        let mut m = i.next().unwrap().chars().collect::<Vec<_>>();
        let mut b = i.next().unwrap().chars().collect::<Vec<_>>();

        t.insert(0, '.');
        m.insert(0, '.');
        b.insert(0, '.');
        t.push('.');
        m.push('.');
        b.push('.');

        let mut part = String::new();
        let mut num_valid = false;
        for (index, char) in m.clone().into_iter().enumerate().skip(1) {
            if !char.is_ascii_digit() {
                //Reached the end of the number
                if !part.is_empty() {
                    if num_valid {
                        sum += part.parse::<u32>().unwrap();
                        num_valid = false;
                    }
                    part.clear();
                }
                continue;
            }
            part.push(char);

            //check surrounding digits
            //Kinda dumb but should work
            num_valid = num_valid
                || t.get(index - 1).copied().unwrap_or('.') != '.'
                || t.get(index + 0).copied().unwrap_or('.') != '.'
                || t.get(index + 1).copied().unwrap_or('.') != '.'
                || {
                    let c = m.get(index - 1).copied().unwrap_or('.');
                    c != '.' && !c.is_ascii_digit()
                }
                || {
                    let c = m.get(index + 1).copied().unwrap_or('.');
                    c != '.' && !c.is_ascii_digit()
                }
                || b.get(index - 1).copied().unwrap_or('.') != '.'
                || b.get(index + 0).copied().unwrap_or('.') != '.'
                || b.get(index + 1).copied().unwrap_or('.') != '.'
        }
    }
    println!("{sum}");
}
