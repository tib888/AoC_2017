fn reverse(l: &mut [u8; 256], pos: usize, size: usize) {
    for i in 0..(size / 2) {
        let a = (pos + i) % l.len();
        let b = (pos + size - 1 - i) % l.len();
        let tmp = l[a];
        l[a] = l[b];
        l[b] = tmp;
    }
}

fn hash_it(instr: &str) -> [i32; 128] {
    let mut l = [0u8; 256];

    for i in 0..l.len() {
        l[i] = i as u8;
    }

    let raw = instr.as_bytes();
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

    let mut binresult = [0i32; 128];

    for a in 0..16 {
        let mut r = 0;
        for b in 0..16 {
            r ^= l[a * 16 + b];
        }
        for c in 0..8 {
            if r & (1 << c) != 0 {
                binresult[a * 8 + (7 - c)] = 1;
            }
        }
    }

    binresult
}

fn fill(x: i32, y: i32, v: &mut Vec<[i32; 128]>) {
    if x < 0 || x >= 128 || y < 0 || y >= 128 {
        return;
    }
    if v[y as usize][x as usize] != 1 {
        return;
    }
    v[y as usize][x as usize] = -1;
    fill(x + 1, y, v);
    fill(x - 1, y, v);
    fill(x, y + 1, v);
    fill(x, y - 1, v);
}

fn tasks(input: &str) -> (i32, i32) {
    let mut sum = 0i32;
    let mut v = Vec::new();
    for i in 0..128 {
        let br = hash_it(&format!("{}-{}", input, i));
        for j in 0..128 {
            sum += br[j];
            //print!("{}", br[j]);
        }
        //println!("");
        v.push(br);
    }


    let mut islands = 0;
    for y in 0..128 {
        for x in 0..128 {
            if v[y][x] == 1 {
                fill(x as i32, y as i32, &mut v);
                islands += 1;
            }
        }
    }

    (sum, islands)
}

fn main() {
    println!("{:?}", tasks("ljoxqyyw"));
}
