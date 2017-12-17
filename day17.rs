
fn task1(step: usize) -> i32 {
    let mut v = Vec::new();
    v.push(0);
    let mut pos = 0 as usize;
    for i in 0..2017 {
        pos = (pos + step) % v.len() + 1;
        v.insert(pos, i + 1);
    }
    v[pos + 1]
}

fn task2(step: usize) -> i32 {
    let mut zeropos = 0;
    let mut res = 0;
    let mut pos = 0 as usize;
    let mut len = 1;
    for i in 0..50_000_000 {
        pos = (pos + step) % len + 1;
        if pos < zeropos {
            zeropos += 1;
        } else if pos == zeropos + 1 {
            res = i + 1;
        }
        len += 1;
    }
    res
}

fn main() {
    println!("{} {}", task1(329), task2(329));
}
