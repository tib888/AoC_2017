
fn task1(mut astate: i64, mut bstate: i64) -> i32 {
    let mut sum = 0;
    for __i in 0..40_000_000 {
        astate = astate * 16807 % 2147483647;
        bstate = bstate * 48271 % 2147483647;
        if astate & 0xFFFF == bstate & 0xFFFF {
            sum += 1;
        }
    }

    sum
}

fn task2(mut astate: i64, mut bstate: i64) -> i32 {
    let mut sum = 0;
    for __i in 0..5_000_000 {
        loop {
            astate = astate * 16807 % 2147483647;
            if astate & 3 == 0 {
                break;
            }
        }
        loop {
            bstate = bstate * 48271 % 2147483647;
            if bstate & 7 == 0 {
                break;
            }
        }
        if astate & 0xFFFF == bstate & 0xFFFF {
            sum += 1;
        }
    }

    sum
}

fn main() {
    println!("{} {}", task1(277i64, 349i64), task2(277i64, 349i64));
}
