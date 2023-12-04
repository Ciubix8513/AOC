fn main() {
    let mut input = include_str!("./input.txt").lines().collect::<Vec<&str>>();
    let mut blank = String::new();
    for _ in 0..input.first().unwrap().len() {
        blank.push('.');
    }
    input.insert(0, &blank);
    input.push(&blank);

    let mut sum = 0;
    let mut gear_map = std::collections::HashMap::new();

    for (w_ind, w) in input.windows(3).enumerate() {
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
        let mut gear = None;
        for (index, char) in m.clone().into_iter().enumerate().skip(1) {
            if !char.is_ascii_digit() {
                //Reached the end of the number
                if !part.is_empty() {
                    if num_valid {
                        let g = part.parse::<u32>().unwrap();
                        if let Some(gear) = gear {
                            let v = gear_map.insert(gear, g);
                            if let Some(v) = v {
                                sum += g * v;
                            }
                        }
                        gear = None;

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
                || {
                    let c = t.get(index - 1).copied().unwrap_or('.');
                    if c == '*' {
                        gear = Some((index - 1, w_ind));
                    }
                    c != '.'
                }
                || {
                    let c = t.get(index + 0).copied().unwrap_or('.');
                    if c == '*' {
                        gear = Some((index, w_ind));
                    }
                    c != '.'
                }
                || {
                    let c = t.get(index + 1).copied().unwrap_or('.');
                    if c == '*' {
                        gear = Some((index + 1, w_ind));
                    }

                    c != '.'
                }
                || {
                    let c = m.get(index - 1).copied().unwrap_or('.');
                    if c == '*' {
                        gear = Some((index - 1, w_ind + 1));
                    }
                    c != '.' && !c.is_ascii_digit()
                }
                || {
                    let c = m.get(index + 1).copied().unwrap_or('.');
                    if c == '*' {
                        gear = Some((index + 1, w_ind + 1));
                    }
                    c != '.' && !c.is_ascii_digit()
                }
                || {
                    let c = b.get(index - 1).copied().unwrap_or('.');
                    if c == '*' {
                        gear = Some((index - 1, w_ind + 2));
                    }
                    c != '.'
                }
                || {
                    let c = b.get(index + 0).copied().unwrap_or('.');
                    if c == '*' {
                        gear = Some((index + 0, w_ind + 2));
                    }
                    c != '.'
                }
                || {
                    let c = b.get(index + 1).copied().unwrap_or('.');
                    if c == '*' {
                        gear = Some((index + 1, w_ind + 2));
                    }
                    c != '.'
                }
        }
    }
    println!("{sum}");
}
