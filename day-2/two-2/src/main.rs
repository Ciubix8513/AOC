fn main() {
    let input = include_str!("./input.txt").lines();

    let mut sum = 0;

    for l in input {
        let split = l.split(":").nth(1).unwrap().split(";");
        let mut rs = Vec::new();
        let mut gs = Vec::new();
        let mut bs = Vec::new();

        for i in split {
            for j in i.split(",") {
                let mut split = j.trim().split(" ");
                let num = split.next().unwrap().parse::<u32>().unwrap();
                match split.next().unwrap() {
                    "red" => {
                        rs.push(num);
                    }
                    "green" => {
                        gs.push(num);
                    }
                    "blue" => {
                        bs.push(num);
                    }
                    _ => {}
                }
            }
        }
        rs.sort();
        gs.sort();
        bs.sort();

        sum += rs.last().unwrap() * bs.last().unwrap() * gs.last().unwrap();
    }

    println!("{sum}");
}
