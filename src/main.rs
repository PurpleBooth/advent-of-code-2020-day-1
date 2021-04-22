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
        .map(|item| (item, &inputs))
        .map(|(target, list)| {
            (
                target,
                list.iter()
                    .find(|list_item| (*list_item) + (*target) == 2020),
            )
        })
        .filter_map(|item| match item {
            (left, Some(right)) => Some(left * right),
            _ => None,
        })
        .collect::<Vec<_>>()
        .get(0)
        .cloned();
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
        assert_eq!(Some(1_020_100), expenses(&[1010, 1010]))
    }

    #[test]
    fn multiple_answers() {
        assert_eq!(Some(1_020_100), expenses(&[1, 2, 3, 4, 1010, 1010, 5, 6]))
    }
}
