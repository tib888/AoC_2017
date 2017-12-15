/*
#[derive(Copy, Clone)]
struct Layer
{
    range : i8,
    pos: i8,
    dir: i8,
}

fn task1(lines: &str) -> i32 {
    let mut state = [Layer { range: 0, pos: 0, dir: 1 }; 100];
    for l in lines.lines() {
        let ll = l.replace(":","");
        let d = ll.split_whitespace().collect::<Vec<_>>();
        let depth = d[0].parse::<usize>().unwrap();
        let range = d[1].parse::<i8>().unwrap();
        state[depth].range = range;
    }
    let mut sum = 0i32;
    for t in 0..100 {
        if state[t].range!=0 && state[t].pos == 0 {
            sum += (t as i32) * (state[t].range as i32);
        }
        for i in 0..100 {
            if state[i].range == 0 {
                continue;
            }
            state[i].pos += state[i].dir;
            if state[i].pos < 0 {
                state[i].dir = 1;
                state[i].pos = 1;
            } 
            else if state[i].pos >= state[i].range {
                state[i].dir = -1;
                state[i].pos -= 2;
            }
        }
    }
    sum
}

fn task2(lines: &str) -> i32 {
    let mut start = [(-1i32, 1); 100];
    let mut places = Vec::new();
    for l in lines.lines() {
        let ll = l.replace(":", "");
        let d = ll.split_whitespace().collect::<Vec<_>>();
        let depth = d[0].parse::<usize>().unwrap();
        let range = d[1].parse::<i32>().unwrap();
        places.push((depth, range));
        start[depth] = (0i32, 1);
    }

    let mut delay = 0i32;
    'outer: loop {
        let mut state = start.clone();
                
        for t in 0..100 {
            if state[t].0 == 0 {
                delay += 1;
                for p in places.iter() {
                    let mut s = &mut (start[p.0]);
                    s.0 += s.1;
                    if s.0 < 0 {
                        s.1 = 1;
                        s.0 = 1;
                    } 
                    else if s.0 >= p.1 {
                        s.1 = -1;
                        s.0 -= 2;
                    }
                }
                continue 'outer;
            }
            for p in places.iter() {
                let mut s = &mut (state[p.0]);
                s.0 += s.1;
                if s.0 < 0 {
                    s.1 = 1;
                    s.0 = 1;
                } 
                else if s.0 >= p.1 {
                    s.1 = -1;
                    s.0 -= 2;
                }
            }
        }        
        break;         
    }
    delay
}
*/

fn task1opt(lines: &str) -> i32 {
    let mut sum = 0i32;

    let mut places = Vec::new();
    for l in lines.lines() {
        let ll = l.replace(":", "");
        let d = ll.split_whitespace().collect::<Vec<_>>();
        let depth = d[0].parse::<i32>().unwrap();
        let range = d[1].parse::<i32>().unwrap();
        places.push((depth, range + range - 2, range));

        if depth % (range + range - 2) == 0 {
            sum += depth * range;
        }
    }

    sum
}

fn task2opt(lines: &str) -> i32 {
    let mut places = Vec::new();
    for l in lines.lines() {
        let ll = l.replace(":", "");
        let d = ll.split_whitespace().collect::<Vec<_>>();
        let depth = d[0].parse::<i32>().unwrap();
        let range = d[1].parse::<i32>().unwrap();
        places.push((depth, range+range-2));
    }

    let mut delay = 0i32;
    'outer: loop {
        delay += 1;
        for p in places.iter() {        
            if (delay + p.0) % p.1 == 0 {
                continue 'outer;
            }
        }
        break;         
    }

    delay
}

fn main() {
    let d13 = 
"0: 3
1: 2
2: 4
4: 4
6: 5
8: 6
10: 6
12: 6
14: 6
16: 8
18: 8
20: 8
22: 8
24: 10
26: 8
28: 8
30: 12
32: 14
34: 12
36: 10
38: 12
40: 12
42: 9
44: 12
46: 12
48: 12
50: 12
52: 14
54: 14
56: 14
58: 12
60: 14
62: 14
64: 12
66: 14
70: 14
72: 14
74: 14
76: 14
80: 18
88: 20
90: 14
98: 17";
   
    print!("{} {}\n", task1opt(d13), task2opt(d13));
}