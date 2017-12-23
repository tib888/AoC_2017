use std::collections::BTreeMap;

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
    let mut n = 0;
    loop {
        if pc >= lines.len() as i64 {
            break;
        }
        let s = lines[pc as usize].split_whitespace().collect::<Vec<_>>();

        match s[0] {
            "set" => {
                let b = eval(s[2], &registers);
                registers.insert(s[1].chars().next().unwrap(), b);
                pc += 1;
            }
            "sub" => {
                let a = eval(s[1], &registers);
                let b = eval(s[2], &registers);
                registers.insert(s[1].chars().next().unwrap(), a - b);
                pc += 1;
            }
            "mul" => {
                let a = eval(s[1], &registers);
                let b = eval(s[2], &registers);
                registers.insert(s[1].chars().next().unwrap(), a * b);
                pc += 1;
                n += 1;
            }
            "jnz" => {
                let a = eval(s[1], &registers);
                if a != 0 {
                    let b = eval(s[2], &registers);
                    pc += b;
                } else {
                    pc += 1;
                }
            }
            _ => {
                pc += 1;
            }
        }
    }
    n
}
/*
fn task2(input: &str) -> (i64, i64) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut registers: BTreeMap<char, i64> = BTreeMap::new();
    let mut pc = 0i64;
    let mut n = 0;
    registers.insert('a', 1);
    loop {
        n += 1;
        //println!("{:?}", registers);
        //println!("{}", lines[pc as usize]);
        if pc >= lines.len() as i64 {
            break;
        }
        let s = lines[pc as usize].split_whitespace().collect::<Vec<_>>();

        match s[0] {
            "set" => {
                let b = eval(s[2], &registers);
                registers.insert(s[1].chars().next().unwrap(), b);
                pc += 1;
            }
            "sub" => {
                let a = eval(s[1], &registers);
                let b = eval(s[2], &registers);
                registers.insert(s[1].chars().next().unwrap(), a - b);
                pc += 1;
            }
            "mul" => {
                let a = eval(s[1], &registers);
                let b = eval(s[2], &registers);
                registers.insert(s[1].chars().next().unwrap(), a * b);
                pc += 1;
            }
            "jnz" => {
                let a = eval(s[1], &registers);
                if a != 0 {
                    let b = eval(s[2], &registers);
                    pc += b;
                } else {
                    pc += 1;
                }
            }
            _ => {
                pc += 1;
            }
        }
    }
    (n, *registers.get(&'h').unwrap())
}
*/

fn task2opt() -> i64
{  
    //count the nonprime numbers
    //in the set 109900 + n*17
    //where n = [0...1000]

    let mut b = 109900; 
    let mut h = 0;

    let c = b + 17000;
    loop {
        for d in 2..b {
            if b % d == 0 {
                h += 1;
                break;
            }   
            if d*d > b { break; } //some extta optimization: break at square root :)
        }        
        if b >= c { 
            break; 
        }
        b += 17;
    }
    h
}

fn main() {    
    let d23 = "set b 99
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23";
    println!("{:?} {:?}", task1(d23), task2opt());
}
