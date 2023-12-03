fn main() {
    let mut sum = 0;
    for l in std::io::stdin().lines() {
        sum += calibration_value(l.expect("could not get stdin line"));
    }
    println!("{}", sum);
}

fn calibration_value(line: String) -> u64 {
    let mut matchers: Vec<(Matcher, u64)> = vec![
        (Matcher::new("1"), 1),
        (Matcher::new("one"), 1),
        (Matcher::new("2"), 2),
        (Matcher::new("two"), 2),
        (Matcher::new("3"), 3),
        (Matcher::new("three"), 3),
        (Matcher::new("4"), 4),
        (Matcher::new("four"), 4),
        (Matcher::new("5"), 5),
        (Matcher::new("five"), 5),
        (Matcher::new("6"), 6),
        (Matcher::new("six"), 6),
        (Matcher::new("7"), 7),
        (Matcher::new("seven"), 7),
        (Matcher::new("8"), 8),
        (Matcher::new("eight"), 8),
        (Matcher::new("9"), 9),
        (Matcher::new("nine"), 9),
    ];

    let mut first = None;
    let mut last = None;

    for c in line.chars() {
        for (matcher, value) in &mut matchers {
            if matcher.accept(c) {
                if first.is_none() {
                    first = Some(*value);
                }

                last = Some(*value);
            }
        }
    }

    match (first, last) {
        (Some(f), Some(l)) => f * 10 + l,
        _ => 0,
    }
}

struct Matcher {
    seq: Vec<char>,
    states: Vec<usize>,
}

impl Matcher {
    fn new(seq: &str) -> Self {
        Self {
            seq: seq.chars().collect(),
            states: Vec::new(),
        }
    }

    fn accept(&mut self, c: char) -> bool {
        // push the start of new matches
        if self.seq[0] == c {
            self.states.push(0)
        }

        // progress existing matches
        let mut deletes = Vec::new();
        for (i, state) in self.states.iter_mut().enumerate() {
            if self.seq[*state] == c {
                *state += 1
            } else {
                deletes.push(i)
            }
        }

        // check for completed matches
        let mut got_match = false;
        for (i, state) in self.states.iter().enumerate() {
            if *state == self.seq.len() {
                got_match = true;
                deletes.push(i)
            }
        }

        // delete failed matches
        deletes.sort();
        for (n, i) in deletes.iter().enumerate() {
            self.states.remove(i - n);
        }

        return got_match;
    }
}
