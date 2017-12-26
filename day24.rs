use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn insert(m: &mut BTreeMap<i32, BTreeSet<i32>>, k: i32, v: i32) {
    if let Some(s) = m.get_mut(&k) {
        s.insert(v);
        return;
    }

    let mut s = BTreeSet::new();
    s.insert(v);
    m.insert(k, s);
}

struct Item {
    a: i32,
    b: i32,
    used: bool,
}

fn search1(k: i32, m: &BTreeMap<i32, BTreeSet<i32>>, p: &mut Vec<Item>) -> i32 {
    let mut max = 0i32;

    if let Some(x) = m.get(&k) {
        for &i in x.iter() {
            if p[i as usize].used {
                continue;
            }
            p[i as usize].used = true;

            let m = if p[i as usize].a == k {
                p[i as usize].a + search1(p[i as usize].b, m, p)
            } else {
                p[i as usize].b + search1(p[i as usize].a, m, p)
            };

            p[i as usize].used = false;

            if m > max {
                max = m;
            }
        }
    }

    k + max
}

fn search2(k: i32, m: &BTreeMap<i32, BTreeSet<i32>>, p: &mut Vec<Item>) -> (i32, i32) {
    let mut maxs = 0i32;
    let mut maxl = 0i32;

    if let Some(x) = m.get(&k) {
        for &i in x.iter() {
            if p[i as usize].used {
                continue;
            }
            p[i as usize].used = true;

            let (s, l) = if p[i as usize].a == k {
                let (s, l) = search2(p[i as usize].b, m, p);
                (p[i as usize].a + s, l)
            } else {
                let (s, l) = search2(p[i as usize].a, m, p);
                (p[i as usize].b + s, l)
            };

            p[i as usize].used = false;

            if l > maxl {
                maxl = l;
                maxs = s;
            } else if l == maxl {
                if s > maxs {
                    maxs = s;
                }
            }
        }
    }

    (k + maxs, 1 + maxl)
}

fn tasks(input: &str) -> (i32, i32) {
    let lines = input.lines().collect::<Vec<_>>();

    let mut pieces = Vec::new();
    let mut index = BTreeMap::new();
    let mut i = 0;
    for l in lines {
        let x = l.split('/').collect::<Vec<_>>();
        let a = x[0].parse::<i32>().unwrap();
        let b = x[1].parse::<i32>().unwrap();
        pieces.push(Item {
            a: a,
            b: b,
            used: false,
        });
        insert(&mut index, a, i);
        insert(&mut index, b, i);
        i += 1;
    }

    let strongest = search1(0, &index, &mut pieces);
    let (strength_of_longest, _length) = search2(0, &index, &mut pieces);
    (strongest, strength_of_longest)
}


fn main() {
    let d24 = "25/13
4/43
42/42
39/40
17/18
30/7
12/12
32/28
9/28
1/1
16/7
47/43
34/16
39/36
6/4
3/2
10/49
46/50
18/25
2/23
3/21
5/24
46/26
50/19
26/41
1/50
47/41
39/50
12/14
11/19
28/2
38/47
5/5
38/34
39/39
17/34
42/16
32/23
13/21
28/6
6/20
1/30
44/21
11/28
14/17
33/33
17/43
31/13
11/21
31/39
0/9
13/50
10/14
16/10
3/24
7/0
50/50";
    println!("{:?}", tasks(d24));
}
