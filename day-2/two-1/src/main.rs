fn main() {
    let input = include_str!("./input.txt").lines();

    let mut sum = 0;

    for l in input {
        let mut split = l.split(":");

        let id = split
            .next()
            .unwrap()
            .trim_start_matches("Game ")
            .parse::<u32>()
            .unwrap();

        let split = split.next().unwrap().split(";");
        let mut valid = true;

        for i in split {
            for j in i.split(",") {
                let mut split = j.trim().split(" ");
                let num = split.next().unwrap().parse::<u32>().unwrap();
                match split.next().unwrap() {
                    "red" => {
                        if num > 12 {
                            valid = false;
                            break;
                        }
                    }
                    "green" => {
                        if num > 13 {
                            valid = false;
                            break;
                        }
                    }
                    "blue" => {
                        if num > 14 {
                            valid = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if !valid {
                break;
            }
        }
        if !valid {
            continue;
        }

        sum += id;
    }

    println!("{sum}");
}
