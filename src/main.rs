use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut inputs: Vec<i64> = vec![];

    for line in stdin.lock().lines() {
        inputs.push(
            line.map_err(Box::<dyn std::error::Error>::from)
                .and_then(|x| x.parse().map_err(Box::<dyn std::error::Error>::from))?,
        )
    }

    println!("{:?}", expenses(&inputs));

    Ok(())
}

fn expenses(inputs: &[i64]) -> Option<i64> {
    return inputs
        .iter()
        .flat_map(move |&a| {
            inputs
                .iter()
                .flat_map(move |&b| inputs.iter().map(move |&c| [a, b, c]))
        })
        .filter(|[a, b, c]| a + b + c == 2020)
        .next()
        .map(|[a, b, c]| a * b * c);
}

#[cfg(test)]
mod tests {
    use crate::expenses;

    #[test]
    fn no_input_result_is_none() {
        assert_eq!(None, expenses(&[]))
    }

    #[test]
    fn no_year_that_adds_up_to_2020() {
        assert_eq!(None, expenses(&[1, 1]))
    }

    #[test]
    fn only_two_answers() {
        assert_eq!(Some(241_861_950), expenses(&[979, 366, 675]))
    }

    #[test]
    fn multiple_answers() {
        assert_eq!(
            Some(241_861_950),
            expenses(&[1, 2, 3, 4, 979, 366, 675, 5, 6])
        )
    }
}
