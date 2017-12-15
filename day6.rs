use std::collections::BTreeSet;
use std::collections::BTreeMap;

fn task1(line: &str) -> i32 {
    let mut numbers = line.split_whitespace()
        .map(|word| word.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let n = numbers.len();
    let mut already = BTreeSet::new();

    let mut sum = 0;

    loop {
        if already.contains(&numbers) {
            return sum;
        }
        already.insert(numbers.clone());

        let mut max: usize = 0;
        let mut p: usize = 0;
        for i in 0..n {
            if numbers[i] > max {
                p = i;
                max = numbers[i];
            }
        }

        numbers[p] = 0;
        for _ in 0..max {
            p = (p + 1) % n;
            numbers[p] += 1;
        }

        sum += 1;
    }
}

fn task2(line: &str) -> i32 {
    let mut numbers = line.split_whitespace()
        .map(|word| word.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let n = numbers.len();
    let mut already = BTreeMap::new();

    let mut sum = 0;

    loop {
        if already.contains_key(&numbers) {
            return sum - already[&numbers];
        }
        already.insert(numbers.clone(), sum);

        let mut max: usize = 0;
        let mut p: usize = 0;
        for i in 0..n {
            if numbers[i] > max {
                p = i;
                max = numbers[i];
            }
        }

        numbers[p] = 0;
        for _ in 0..max {
            p = (p + 1) % n;
            numbers[p] += 1;
        }

        sum += 1;
    }
}


fn main() {
    let day6 = "4	1	15	12	0	9	9	5	5	8	7	3	14	5	12	3";
    println!("{} {}", task1(day6), task2(day6));
}
