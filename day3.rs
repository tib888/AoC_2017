const MAX: i32 = 280;

fn strip(
    m: u32,
    target: u32,
    n: &mut u32,
    x: &mut i32,
    y: &mut i32,
    xstep: i32,
    ystep: i32,
) -> Option<i32> {
    for _ in 1..m {
        *n += 1;
        *x += xstep;
        *y += ystep;

        let dx = *x - MAX;
        let dy = *y - MAX;

        if *n == target {
            return Some(dx.abs() + dy.abs());
        }
    }
    None
}

fn task1(target: u32) -> i32 {
    let mut x = MAX;
    let mut y = MAX;
    let mut n = 1u32;

    let mut m = 1u32;
    for _ in 1..MAX {
        m += 2;
        x += 1;
        y += 1;

        if let Some(result) = strip(m, target, &mut n, &mut x, &mut y, 0, -1) {
            return result;
        }
        if let Some(result) = strip(m, target, &mut n, &mut x, &mut y, -1, 0) {
            return result;
        }
        if let Some(result) = strip(m, target, &mut n, &mut x, &mut y, 0, 1) {
            return result;
        }
        if let Some(result) = strip(m, target, &mut n, &mut x, &mut y, 1, 0) {
            return result;
        }
    }

    -1
}

fn calcsum(v: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    v[y - 1][x - 1] + v[y - 1][x] + v[y - 1][x + 1] + v[y][x - 1] + v[y][x + 1] + v[y + 1][x - 1]
        + v[y + 1][x] + v[y + 1][x + 1]
}

fn task2(target: u32) -> u32 {
    let mut v = vec![vec![0u32; (MAX + MAX + 1) as usize]; (MAX + MAX + 1) as usize];

    let mut x = MAX;
    let mut y = MAX;
    v[y as usize][x as usize] = 1;

    let mut m = 1;
    'outer: for _ in 1..MAX {
        m += 2;
        x += 1;
        y += 1;

        for _ in 1..m {
            y -= 1;
            let sum = calcsum(&mut v, x as usize, y as usize);
            if sum > target {
                return sum;
            }
            v[y as usize][x as usize] = sum;
        }
        for _ in 1..m {
            x -= 1;
            let sum = calcsum(&mut v, x as usize, y as usize);
            if sum > target {
                return sum;
            }
            v[y as usize][x as usize] = sum;
        }
        for _ in 1..m {
            y += 1;
            let sum = calcsum(&mut v, x as usize, y as usize);
            if sum > target {
                return sum;
            }
            v[y as usize][x as usize] = sum;
        }
        for _ in 1..m {
            x += 1;
            let sum = calcsum(&mut v, x as usize, y as usize);
            if sum > target {
                return sum;
            }
            v[y as usize][x as usize] = sum;
        }
    }

    0
}

fn main() {
    let day3 = 289326u32;
    println!("{} {}", task1(day3), task2(day3));
}
