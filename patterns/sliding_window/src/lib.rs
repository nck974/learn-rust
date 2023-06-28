/// Calculate the maximum sum of a sliding window for each element
pub fn calculate_max_number(numbers: Vec<i32>, window_size: usize) -> Vec<i32> {
    let res: Vec<i32> = (0..numbers.len() - 1)
        .map(|i| {
            let start_index = i.saturating_sub(window_size - 1);
            let end_index = (i + window_size - 1).min(numbers.len() - 1);

            let max_sum = (start_index..=end_index)
                .map(|j| numbers[j])
                .collect::<Vec<_>>()
                .windows(window_size)
                .map(|window| window.iter().sum::<i32>())
                .max()
                .unwrap_or(0);

            max_sum
        })
        .collect();
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sliding_window() {
        let numbers: Vec<i32> = vec![4, 2, 1, 7, 8, 1, 2, 8, 1, 0];
        let window_size: usize = 3;
        assert_eq!(
            vec![7, 10, 16, 16, 16, 16, 11, 11, 11],
            calculate_max_number(numbers, window_size)
        );
    }
}
