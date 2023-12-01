
fn main() {
    let mut sum = 0;
    for l in std::io::stdin().lines() {
        sum += calibration_value(l.expect("could not get stdin line"));
    }
    println!("{}", sum);
}

fn calibration_value(line: String) -> u64 {
    let mut first = None;
    let mut last = None;

    for c in line.chars() {
        if let Some(digit) = c.to_digit(10) {
            if first.is_none() {
                first = Some(digit);
            }

            last = Some(digit);
        }
    }

    match (first, last) {
        (Some(f), Some(l)) => f as u64 * 10 + l as u64,
        _ => 0,
    }
}
