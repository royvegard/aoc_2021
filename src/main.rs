mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

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

    println!("Day 4: Giant Squid");
    println!(
        "\tPart One: {}",
        day_04::part_1(String::from("input/day_04"))
    );
    println!(
        "\tPart Two: {}",
        day_04::part_2(String::from("input/day_04"))
    );

    println!("Day 5: Hydrothermal Venture");
    println!(
        "\tPart One: {}",
        day_05::part_1(String::from("input/day_05"))
    );
    println!(
        "\tPart Two: {}",
        day_05::part_2(String::from("input/day_05"))
    );

    println!("Day 6: Lanternfish");
    println!(
        "\tPart One: {}",
        day_06::part_1(String::from("input/day_06"))
    );
    println!(
        "\tPart Two: {}",
        day_06::part_2(String::from("input/day_06"))
    );
}
