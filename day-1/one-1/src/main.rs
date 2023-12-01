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
