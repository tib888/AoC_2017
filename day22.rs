use std::collections::BTreeSet;
use std::collections::BTreeMap;

fn task1(input: &str, m: i32) -> i32 {
    let mut nodes = BTreeSet::<(i32, i32)>::new();
    let mut y = 0i32;
    for l in input.lines() {
        let mid = l.len() as i32 / 2;
        let mut x = 0;
        for c in l.chars() {
            if c == '#' {
                nodes.insert((x - mid, y - mid));
            }
            x += 1;
        }
        y += 1;
    }

    let mut dx = 0i32;
    let mut dy = -1i32;
    let mut posx = 0i32;
    let mut posy = 0i32;

    let mut nn = 0i32;

    for _ in 0..m {
        let infected = nodes.contains(&(posx, posy));

        if infected {
            //turn right
            let tmp = dy;
            dy = dx;
            dx = -tmp;
            nodes.remove(&(posx, posy));
        } else {
            //turn left
            let tmp = dy;
            dy = -dx;
            dx = tmp;
            nodes.insert((posx, posy));
            nn += 1;
        }

        posx += dx;
        posy += dy;
    }

    // for y in -40..40 {
    //     for x in -40..40 {
    //         if nodes.contains(&(x, y)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    nn
}

enum State {
    Weakened,
    Infected,
    Flagged,
}

fn task2(input: &str, m: i32) -> i32 {
    let mut nodes = BTreeMap::<(i32, i32), State>::new();
    let mut y = 0i32;

    for l in input.lines() {
        let mid = l.len() as i32 / 2;
        let mut x = 0;
        for c in l.chars() {
            if c == '#' {
                nodes.insert((x - mid, y - mid), State::Infected);
            }
            x += 1;
        }
        y += 1;
    }

    let mut dx = 0i32;
    let mut dy = -1i32;
    let mut posx = 0i32;
    let mut posy = 0i32;

    let mut nn = 0i32;

    for _i in 0..m {
        match nodes.get(&(posx, posy)) {
            Some(&State::Infected) => {
                //turn right
                let tmp = dy;
                dy = dx;
                dx = -tmp;
                nodes.insert((posx, posy), State::Flagged);
            }
            Some(&State::Weakened) => {
                //continue in the same direction
                nodes.insert((posx, posy), State::Infected);
                nn += 1;
            }
            Some(&State::Flagged) => {
                //flip dir
                dy = -dy;
                dx = -dx;
                nodes.remove(&(posx, posy));
            }
            None => {
                //turn left
                let tmp = dy;
                dy = -dx;
                dx = tmp;
                nodes.insert((posx, posy), State::Weakened);
            }
        }

        posx += dx;
        posy += dy;
    }

    nn
}

fn main() {
    let day22 = "##########..#.###...##..#
##....#...#....#..####.#.
#..#.##..#..##.###..#.###
.#.#.......####.....#.#..
...######....#.##########
##.#.....#.#####.#....###
#.####.#..#.#.#...#.#..##
#.##..#####..###..###.##.
#.####.#.##.##...#.#.#.##
#.#.#......##.##..###.#.#
#...#.#..#.##....#.##..##
.#.....##.##..#.####..##.
.#......#.#.########..###
##....###.#.#.###...##..#
..##.###....#.....#...#.#
....##...##...##.##.#..##
..#.#.#..#######..###..##
......#####.#####..#.#..#
.####.#......#..###..#.##
#....####.#..#.##.###.##.
####.#...##....###...#.#.
#####.#......#.#..###.##.
#.##.#..#..#..#.....#.#.#
#...#.#.##.#.####.#.#..#.
.##.##..#..###.##.....###";
    println!("{}", task1(day22, 10000));
    println!("{}", task2(day22, 10000000));
}
