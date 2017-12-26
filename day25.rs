use std::collections::BTreeSet;

enum States {
    A,
    B,
    C,
    D,
    E,
    F,
}

fn tasks(n: i32) -> i32 {
    let mut tape = BTreeSet::new();
    let mut state = States::A;
    let mut pos = 0i32;
    for _ in 0..n {
        let x = tape.contains(&pos);
        let (x2, pos2, state2) = match (state, x) {
            (States::A, false) => (true, 1, States::B),
            (States::A, true) => (false, -1, States::F),

            (States::B, false) => (false, 1, States::C),
            (States::B, true) => (false, 1, States::D),

            (States::C, false) => (true, -1, States::D),
            (States::C, true) => (true, 1, States::E),

            (States::D, false) => (false, -1, States::E),
            (States::D, true) => (false, -1, States::D),

            (States::E, false) => (false, 1, States::A),
            (States::E, true) => (true, 1, States::C),

            (States::F, false) => (true, -1, States::A),
            (States::F, true) => (true, 1, States::A),
        };
        state = state2;

        if x2 {
            tape.insert(pos);
        } else {
            tape.remove(&pos);
        }

        pos += pos2;
    }

    tape.len() as i32
}


fn main() {
    println!("{:?}", tasks(12794428));
}
