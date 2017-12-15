
fn reverse(l: &mut [u8; 256], pos: usize, size: usize) {
    for i in 0..(size / 2) {
        let a = (pos + i) % l.len();
        let b = (pos + size - 1 - i) % l.len();
        let tmp = l[a];
        l[a] = l[b];
        l[b] = tmp;
    }
}

fn task1() -> i32 {
    let mut l = [0u8; 256];

    for i in 0..l.len() {
        l[i] = i as u8;
    }

    let input: [usize; 16] = [
        120,
        93,
        0,
        90,
        5,
        80,
        129,
        74,
        1,
        165,
        204,
        255,
        254,
        2,
        50,
        113,
    ];

    let mut skip = 0;
    let mut pos = 0;
    for s in input.iter() {
        reverse(&mut l, pos, *s);
        pos += s + skip;
        skip += 1;
    }
   
   (l[0] as i32) * (l[1] as i32)
}

fn task2() -> String {
    let mut l = [0u8; 256];

    for i in 0..l.len() {
        l[i] = i as u8;
    }

    let raw = "120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113".as_bytes();
    let input = [raw, &[17, 31, 73, 47, 23]].concat();

    let mut skip = 0;
    let mut pos = 0;
    for _ in 0..64 {
        for s in input.iter() {
            reverse(&mut l, pos, *s as usize);
            pos += (*s as usize) + skip;
            skip += 1;
        }
    }

    let mut result = String::new();

    for a in 0..16 {
        let mut r = 0;
        for b in 0..16 {
            r ^= l[a * 16 + b];
        }
        result += &format!("{:02x}", r);
    }

    result
}


fn main() {
    println!("{} {}", task1(), task2());
}
