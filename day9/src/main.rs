use itertools::Itertools;


fn first_hist_entry(vec: &[i64]) -> i64 
{
    if vec.iter().sum::<i64>() == 0 {
        return 0;
    }
    let a = vec.iter().tuple_windows().map(|(a,b)| b-a).collect::<Vec<i64>>();
    vec.first().unwrap() - first_hist_entry(&a)
}
fn last_hist_entry(vec: &[i64]) -> i64 
{
    if vec.iter().sum::<i64>() == 0 {
        return 0;
    }
    let a = vec.iter().tuple_windows().map(|(a,b)| b-a).collect::<Vec<i64>>();
    vec.last().unwrap() + last_hist_entry(&a)
}
fn calculate_prediction(input: &str) -> i64 {
    input.lines().map(|line| {
        line.split_whitespace().map(
            |num_str| {num_str.parse::<i64>().unwrap()})
            .collect::<Vec<i64>>()
    }).map(|vec| {
        last_hist_entry(&vec)
    })
    .sum::<i64>()
}
fn calculate_prediction2(input: &str) -> i64 {
    input.lines().map(|line| {
        line.split_whitespace().map(
            |num_str| {num_str.parse::<i64>().unwrap()})
            .collect::<Vec<i64>>()
    }).map(|vec| {
        first_hist_entry(&vec)
    })
    .sum::<i64>()
}

fn main() {
    let file_content = std::fs::read_to_string("data.txt").expect("Data file should be present in the file system");
    let result = calculate_prediction(&file_content);
    let result2 = calculate_prediction2(&file_content);
    println!("{}",result);
    println!("{}",result2);

}

#[cfg(test)]
mod test {
    use crate::{calculate_prediction, calculate_prediction2};

    #[test]
    fn test_example () {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let answer = calculate_prediction(&input);
        assert_eq!(114, answer);
    }
    #[test]
    fn test_example2 () {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let answer = calculate_prediction2(&input);
        assert_eq!(2, answer);
    }

}
