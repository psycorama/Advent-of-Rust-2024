#[path = "../libs/helper.rs"]
mod helper;

fn parse_input(input: Vec<String>) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut ordering_rules: Vec<(i32, i32)> = vec![];
    let mut page_numbers: Vec<Vec<i32>> = vec![];

    let mut part = 1;

    for line in input {
        if line.is_empty() {
            part = 2;
            continue;
        }
        if part == 1 {
            let values: Vec<&str> = line.trim().split("|").collect();
            ordering_rules.push((
                values[0].parse::<i32>().expect("unable to parse"),
                values[1].parse::<i32>().expect("unable to parse"),
            ));
        } else {
            let values: Vec<&str> = line.trim().split(",").collect();
            let mut num_val: Vec<i32> = vec![];
            for i in 0..values.len() {
                num_val.push(values[i].parse::<i32>().expect("unable to parse"));
            }
            page_numbers.push(num_val);
        }
    }

    return (ordering_rules, page_numbers);
}

fn check_first(ordering_rules: &Vec<(i32, i32)>, first: i32) -> bool {
    for &(_, i) in ordering_rules {
        if i == first {
            return false;
        }
    }
    return true;
}
fn check_ordering(ordering_rules: &Vec<(i32, i32)>, first: i32, second: i32) -> bool {
    for rule in ordering_rules {
        let (a, b) = rule;
        if a == &first && b == &second {
            return true;
        }
        if
    }
    return false;
}

fn main() {
    // day 5 - part 1

    let raw_input = helper::lines_from_file("./input.txt");
    let (ordering_rules, page_numbers) = parse_input(raw_input);

    // check read-in
    /*
    for i in 0..10 {
        let (a, b) = ordering_rules[i];
        println!("ordering rule: {}->{}", a, b);
    }

    for i in 0..10 {
        print!("page numbers: ");
        for n in &page_numbers[i] {
            print!("{}, ", n);
        }
        println!();
    }
    */

    let mut correct_pages = 0;

    for pages in page_numbers {
        if !check_first(&ordering_rules, pages[0]) {
            print!(".");
            continue;
        }
        for i in 1..(pages.len() - 1) {
            if !check_ordering(&ordering_rules, pages[i], pages[i + 1]) {
                break;
            }
        }
        correct_pages += 1;
    }
    println!("found {} correct pages", correct_pages);
}
