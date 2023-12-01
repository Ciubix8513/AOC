fn main() {
    let input = include_str!("./input.txt");
    let lines = input.lines();
    let mut output = 0;
    for i in lines {
        output += calculate_value(i);
    }
    println!("{output}");
}

fn calculate_value(input: &str) -> u32 {
    let mut first_num = None;
    let mut last_num = None;

    let input = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    println!("{input}");

    for c in input.chars() {
        if let Some(num) = c.to_digit(10) {
            if first_num.is_none() {
                first_num = Some(num);
            } else {
                last_num = Some(num)
            }
        }
    }

    let first_num = first_num.unwrap();
    let last_num = last_num.unwrap_or(first_num);

    first_num * 10 + last_num
}
