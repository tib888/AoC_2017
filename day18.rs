use std::collections::BTreeMap;
use std::collections::VecDeque;

fn eval(z: &str, registers: &BTreeMap<char, i64>) -> i64 {
    if let Ok(n) = z.parse::<i64>() {
        n
    } else {
        if let Some(m) = registers.get(&z.chars().next().unwrap()) {
            *m
        } else {
            0
        }
    }
}

fn task1(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut registers: BTreeMap<char, i64> = BTreeMap::new();
    let mut pc = 0i64;
    let mut sound = 0;
    loop {
        //println!("{:?}", registers);
        //println!("{}", lines[pc as usize]);
        let s = lines[pc as usize].split_whitespace().collect::<Vec<_>>();

        match s[0] {
            "set" => {
                let b = eval(s[2], &registers);
                registers.insert(s[1].chars().next().unwrap(), b);
                pc += 1;
            }
            "mul" => {
                let a = eval(s[1], &registers);
                let b = eval(s[2], &registers);
                registers.insert(s[1].chars().next().unwrap(), a * b);
                pc += 1;
            }
            "jgz" => {
                let a = eval(s[1], &registers);
                if a > 0 {
                    let b = eval(s[2], &registers);
                    pc += b;
                } else {
                    pc += 1;
                }
            }
            "add" => {
                let a = eval(s[1], &registers);
                let b = eval(s[2], &registers);
                registers.insert(s[1].chars().next().unwrap(), a + b);
                pc += 1;
            }
            "mod" => {
                let a = eval(s[1], &registers);
                let b = eval(s[2], &registers);
                registers.insert(s[1].chars().next().unwrap(), a % b);
                pc += 1;
            }
            "snd" => {
                let a = eval(s[1], &registers);
                sound = a;
                pc += 1;
            }
            "rcv" => {
                let a = eval(s[1], &registers);
                if a != 0 {
                    registers.insert(s[1].chars().next().unwrap(), sound);
                    break;
                }
                pc += 1;
            }
            _ => {
                pc += 1;
            }
        }
    }
    sound
}

fn task2(input: &str) -> i64 {
    let mut result = 0;
    let lines = input.lines().collect::<Vec<_>>();
    let mut registers: [BTreeMap<char, i64>; 2] = [BTreeMap::new(), BTreeMap::new()];
    let mut pc = [0i64; 2];
    let mut fifo: [VecDeque<i64>; 2] = [VecDeque::new(), VecDeque::new()];

    let mut waiting = 0;
    let mut this = 0;

    registers[0].insert('p', 0);
    registers[1].insert('p', 1);

    loop {
        //println!("{}: {}", this, lines[pc[this] as usize]);
        let s = lines[pc[this] as usize]
            .split_whitespace()
            .collect::<Vec<_>>();

        match s[0] {
            "set" => {
                let b = eval(s[2], &registers[this]);
                registers[this].insert(s[1].chars().next().unwrap(), b);
                pc[this] += 1;
            }
            "mul" => {
                let a = eval(s[1], &registers[this]);
                let b = eval(s[2], &registers[this]);
                registers[this].insert(s[1].chars().next().unwrap(), a * b);
                pc[this] += 1;
            }
            "jgz" => {
                let a = eval(s[1], &registers[this]);
                if a > 0 {
                    let b = eval(s[2], &registers[this]);
                    pc[this] += b;
                } else {
                    pc[this] += 1;
                }
            }
            "add" => {
                let a = eval(s[1], &registers[this]);
                let b = eval(s[2], &registers[this]);
                registers[this].insert(s[1].chars().next().unwrap(), a + b);
                pc[this] += 1;
            }
            "mod" => {
                let a = eval(s[1], &registers[this]);
                let b = eval(s[2], &registers[this]);
                registers[this].insert(s[1].chars().next().unwrap(), a % b);
                pc[this] += 1;
            }
            "snd" => {
                let a = eval(s[1], &registers[this]);
                fifo[this].push_back(a);
                //println!("0 - {:?}", fifo[0]);
                //println!("1 - {:?}", fifo[1]);
                if this == 1 {
                    result += 1;
                }
                pc[this] += 1;
            }
            "rcv" => {
                //println!("0 - {:?}", fifo[0]);
                //println!("1 - {:?}", fifo[1]);
                if let Some(msg) = fifo[this ^ 1].pop_front() {
                    registers[this].insert(s[1].chars().next().unwrap(), msg);
                } else {
                    this ^= 1;
                    match waiting {
                        0 => {
                            waiting = 1;
                        }
                        1 => if let Some(msg) = fifo[this ^ 1].pop_front() {
                            registers[this].insert(s[1].chars().next().unwrap(), msg);
                        } else {
                            waiting = 2;
                            return result;
                        },
                        _ => {
                            return -1;
                        }
                    }
                }
                pc[this] += 1;
            }
            _ => {
                pc[this] += 1;
            }
        }
    }
}

fn main() {
    let d18 = "set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 680
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19";
    println!("{} {}", task1(d18), task2(d18));
}
