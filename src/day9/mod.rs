use file;

pub fn run() {
    let inputs = file::read_inputs("Day9.txt");

    println!("{:?}", solve(&"{{{},{},{{}}}}"));
    println!("{:?}", solve(&"{<a>,<a>,<a>,<a>}"));
    println!("{:?}", solve(&"{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    println!("{:?}", solve(&"{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    println!("{:?}", solve(&"{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    println!("{:?}", solve(&inputs));
}

enum State {
    Start,
    Garbage(usize, usize, usize),
    Group(usize, usize, usize),
    Cancelled(Box<State>),
}

fn solve(inputs: &str) -> (usize, usize) {
    let stream: Vec<char> = inputs.chars().collect();
    let mut state = State::Start;

    for c in stream {
        match state {
            State::Start => {
                if c == '{' {
                    state = State::Group(1, 1, 0);
                } else {
                    panic!("Invalid data")
                }
            },
            State::Garbage(nest_level, score, garbage_score) => {
                if c == '!' {
                    state = State::Cancelled(Box::new(state));
                } else if c == '>' {
                    state = State::Group(nest_level, score, garbage_score);
                } else {
                    state = State::Garbage(nest_level, score, garbage_score + 1);
                }
            },
            State::Group(nest_level, score, garbage_score) => {
                if c == '{' {
                    state = State::Group(nest_level + 1, score + nest_level + 1, garbage_score);
                } else if c == '}' {
                    state = State::Group(nest_level - 1, score, garbage_score);
                } else if c == '<' {
                    state = State::Garbage(nest_level, score, garbage_score);
                } else if c == '!' {
                    state = State::Cancelled(Box::new(state));
                } else if c == ',' {
                    // just move on
                } else {
                    panic!("Invalid data")
                }
            },
            State::Cancelled(old_state) => {
                state = *old_state;
            },
        }
    }

    match state {
        State::Group(_, score, garbage_score) => return (score, garbage_score),
        _ => panic!("Invalid state")
    }
}