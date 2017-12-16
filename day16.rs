use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn task1(start: &str, moves: &str) -> String {
    let mut result = start.chars().collect::<Vec<_>>();
    for m in moves.split_terminator(',') {
        let (a, b) = m.split_at(1);

        match a {
            "s" => {
                let n = b.parse::<usize>().unwrap();
                let cpy = result.clone();
                let l = cpy.len();
                let o = l - n;
                //println!("s {}", n);
                for c in 0..l {
                    result[c] = cpy[(c + o) % l];
                }
            }

            "x" => {
                let s = b.split_terminator('/').collect::<Vec<_>>();
                let x = s[0].parse::<usize>().unwrap();
                let y = s[1].parse::<usize>().unwrap();
                //println!("x {} {}", x, y);
                let tmp = result[x];
                result[x] = result[y];
                result[y] = tmp;
            }

            "p" => {
                let s = b.split_terminator('/').collect::<Vec<_>>();
                let a = s[0].chars().nth(0).unwrap();
                let b = s[1].chars().nth(0).unwrap();
                //println!("p {} {}", a, b);
                for i in 0..result.len() {
                    if result[i] == a {
                        result[i] = b;
                    } else if result[i] == b {
                        result[i] = a;
                    }
                }
            }

            _ => {}
        };

        //println!("{:?}", result);
    }

    result.into_iter().collect::<String>()
}

fn main() {
    let file = File::open("day16.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    println!("{}", task1("abcdefghijklmnop", &contents));

    let mut s = String::from("abcdefghijklmnop");
    let mut period = 0;
    loop {
        s = task1(&s, &contents);
        period += 1;
        if s == "abcdefghijklmnop" {
            break;
        }
    }

    println!("{}", period);

    let n = 1_000_000_000 % period;

    println!("{}", n);
    let mut s = String::from("abcdefghijklmnop");
    for _ in 0..n {
        s = task1(&s, &contents);
    }
    println!("{}", s);
}
