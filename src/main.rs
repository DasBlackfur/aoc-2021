fn main() {
    aoc_driver::aoc_complete! {
        session_file: "session.txt"
        input_dir: "input"
        challenges: [
            {
                "2021-1-1": day_one_part_one
            }
        ]
    }
}

fn day_one_part_one(input: &str) -> String {
    let numbers: Vec<u16> = input.split_terminator('\n').map(|x| x.parse::<u16>().unwrap()).collect();
    let mut previous = u16::MAX;
    let mut count = 0;

    for number in numbers {
        if number > previous {
            count += 1;
        }
        previous = number;
    }
    count.to_string()
}
