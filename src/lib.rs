use std::cmp;

type Narray = Vec<Vec<usize>>;

trait Show {
    fn print_it(&self);
}

impl Show for Narray {
    fn print_it(&self) {
        print!("[ ");
        for (i, v_slice) in self.iter().enumerate() {
            if i == 0 {
                println!("{:?},", v_slice);
            } else {
                println!("  {:?},", v_slice);
            }
        }
        println!("]");
    }
}

/// Returns the Levenshtein distance of any two utf8 strings.
///
/// # Arguments
///
/// * `a` - String to compare
/// * `b` - String to compare
///
/// # Examples
/// basic usage:
/// ```
/// use leven_dist::lev_distance;
/// 
/// let dist = lev_distance("kitten", "sitting");
/// assert_eq!(dist, 3)
/// ``` 
pub fn lev_distance(a: &str, b: &str) -> usize {
    if a.is_empty() {
        return a.chars().count();
    }
    if b.is_empty() {
        return b.chars().count();
    }

    let b_size = b.chars().count() + 1;
    let a_size = a.chars().count() + 1;

    let mut matrix: Narray = vec![vec![0; b_size]; a_size];

    for (i, char_a) in a.chars().enumerate() {
        matrix[i][0] = i;
        for (j, char_b) in b.chars().enumerate() {
            matrix[0][j] = j;
            let subed = if char_a == char_b { 0 } else { 1 };
            let mini = cmp::min(
                cmp::min(
                    matrix[i + 1][j] + 1, // deletion
                    matrix[i][j + 1] + 1,
                ), // insertion
                matrix[i][j] + subed,
            ); // substitution
            matrix[i + 1][j + 1] = mini;
        }
    }
    matrix[a_size - 1][b_size - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let dist = lev_distance("Devin", "devin");
        assert_eq!(dist, 1);
        // println!("leven dist is: {}", dist);
    }
}
