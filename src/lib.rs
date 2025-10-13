/// Find all possible ways to split a string into exactly N non-empty substrings.
/// Iterative version using an explicit stack.
pub fn split_into_n_substrings(s: &str, n: usize) -> Vec<Vec<&str>> {
    let mut result = Vec::new();

    // Stack holds: (start_position, current_split, remaining_parts)
    let mut stack: Vec<(usize, Vec<&str>, usize)> = vec![(0, Vec::new(), n)];

    while let Some((start, current_split, remaining_parts)) = stack.pop() {
        // Base case: if no more parts needed
        if remaining_parts == 0 {
            // Check if we've used the entire string
            if start == s.len() {
                result.push(current_split);
            }
            continue;
        }

        // Pruning: if remaining string can't be split into remaining_parts
        let chars_left = s.len() - start;
        if chars_left < remaining_parts {
            continue;
        }

        // Try all possible positions for the next cut
        let max_end = s.len() - (remaining_parts - 1);

        // Push states in reverse order so they're processed in correct order
        for end in ((start + 1)..=max_end).rev() {
            let substring = &s[start..end];
            let mut new_split = current_split.clone();
            new_split.push(substring);
            stack.push((end, new_split, remaining_parts - 1));
        }
        println!("{:?}", stack);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_one() {
        let s = "abcd";
        let n = 1;
        let result = split_into_n_substrings(s, n);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec!["abcd"]);
    }

    #[test]
    fn n_two() {
        let s = "abcd";
        let n = 2;
        let result = split_into_n_substrings(s, n);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], vec!["a", "bcd"]);
        assert_eq!(result[1], vec!["ab", "cd"]);
        assert_eq!(result[2], vec!["abc", "d"]);
    }

    #[test]
    fn n_impossible() {
        let s = "abcd";
        let n = 10;
        let result = split_into_n_substrings(s, n);
        assert_eq!(result.len(), 0);
    }
}
