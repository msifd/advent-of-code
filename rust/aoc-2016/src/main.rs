extern crate md5;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

fn main() {
    day_1();
    day_2();
    day_3();
    day_4();
    day_5();
    day_6();
    day_7();
}

fn day_1() {
    use day_1::*;
    let inst = parse_instructions(include_str!("inputs/day_1.txt"));
    println!("[Day 1] distance: {}", get_distance(inst.clone()));
    println!("[Day 1] first loop: {}", get_first_loop(inst.clone()));
}

fn day_2() {
    use day_2::*;
    let inst = parse_input(include_str!("inputs/day_2.txt"));
    println!("[Day 2] code: {}", decode(&inst));
    println!("[Day 2] code on real keypad: {}", decode_real(&inst));
}

fn day_3() {
    use day_3::*;
    let input = include_str!("inputs/day_3.txt");
    println!("[Day 3] valid: {}", count_valid(&parse_input(input)));
    println!("[Day 3] valid in columns: {}", count_valid(&parse_input_in_columns(input)));
}

fn day_4() {
    use day_4::*;
    let input = parse_input(include_str!("inputs/day_4.txt"));
    println!("[Day 4] sector sum: {}", count_sectors(&input));
    println!("[Day 4] sector with North Pole objects: {}", find_northpole_objects(&input));
}

fn day_5() {
    use day_5::*;
    let input = "reyedfim";
    println!("[Day 5] pass: {}", hack_pass(input));
    println!("[Day 5] second pass: {}", hack_second_pass(input));
}

fn day_6() {
    use day_6::*;
    let input = include_str!("inputs/day_6.txt");
    println!("[Day 6] message: {}", correct_message(input));
    println!("[Day 6] correct message: {}", even_more_correct_message(input));
}

fn day_7() {
    use day_7::*;
    let input = parse_input(include_str!("inputs/day_7.txt"));
    println!("[Day 7] tls ips: {}", count_tls_ips(&input));
    println!("[Day 7] ssl ips: {}", count_ssl_ips(&input));
}