use std::char;
use std::collections::HashSet;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = vec![];
    let mines: HashSet<(usize, usize)> = build_mines(minefield);
    for (r, &row) in minefield.iter().enumerate() {
        let mut s = String::new();
        for (c, val) in row.char_indices() {
            if val == '*' {
                s.push('*');
            } else {
                s.push(count(&mines, r as i32, c as i32));
            }
        }
        println!("row, {:?}, s={:?}", row, s);
        result.push(s);
    }
    result
}

fn build_mines(minefield: &[&str]) -> HashSet<(usize, usize)> {
    let mut mines: HashSet<(usize, usize)> = HashSet::new();
    for (r, &row) in minefield.iter().enumerate() {
        for (c, val) in row.char_indices() {
            if val == '*' {
                mines.insert((r, c));
            }
        }
    }
    mines
}

fn is_mine(mines: &HashSet<(usize, usize)>, r: usize, c: usize) -> bool {
    mines.contains(&(r, c))
}

fn count(mines: &HashSet<(usize, usize)>, r: i32, c: i32) -> char {
    let mut result = 0;
    for i in [-1, 0, 1] {
        for j in [-1, 0, 1] {
            if i == 0 && j == 0 {
                continue;
            }
            if is_mine(mines, (r + i) as usize, (c + j) as usize) {
                result += 1;
            }
        }
    }
    if result == 0 {
        ' '
    } else {
        char::from_digit(result, 10).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn run_test(board: &[&str]) {
        let clean_board: Vec<String> = board
            .iter()
            .map(|&s| {
                s.chars()
                    .map(|c| if c == '*' { '*' } else { '.' })
                    .collect()
            })
            .collect();
        let cleaned_str = clean_board.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        println!("cleaned_str =\n {:?}", &cleaned_str);
        assert_eq!(board, annotate(&cleaned_str));
    }

    #[test]
    fn test_simple() {
        // run_test(&[
        //         "",
        // ]);
        #[rustfmt::skip]
        let board = [
        "1*1",
        "111",
        "   "
        ];
        run_test(&board);
    }
}
