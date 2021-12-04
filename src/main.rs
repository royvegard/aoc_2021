mod day_01;
mod day_02;
mod day_03;

fn main() {
    println!("Day 1: Sonar Sweep");
    println!(
        "\tPart One: {}",
        day_01::part_1(String::from("input/day_01"))
    );
    println!(
        "\tPart Two: {}",
        day_01::part_2(String::from("input/day_01"))
    );

    println!("Day 2: Dive!");
    println!(
        "\tPart One: {}",
        day_02::part_1(String::from("input/day_02"))
    );
    println!(
        "\tPart Two: {}",
        day_02::part_2(String::from("input/day_02"))
    );

    println!("Day 3: Binary Diagnostic");
    println!(
        "\tPart One: {}",
        day_03::part_1(String::from("input/day_03"))
    );
    println!(
        "\tPart Two: {}",
        day_03::part_2(String::from("input/day_03"))
    );
}
