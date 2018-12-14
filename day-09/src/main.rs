fn calculate(players: usize, marbles: usize) -> u64 {
    let mut scores = vec![0_u64; players as usize];
    let mut marble = 0;
    let mut idx = 0_usize;
    let mut board = Vec::<u64>::with_capacity(marbles as usize);
    board.push(0);

    'out: loop {
        for player in 1..=players {
            marble += 1;
            if board.len() < 2 {
                board.push(marble);
                idx = 1;
                continue;
            }
            if marble % 23 == 0 {
                scores[player - 1] += marble;
                if idx > 6 {
                    idx -= 7;
                } else {
                    let mut n: i64 = idx as i64 - 7;
                    n = board.len() as i64 + n;
                    idx = n as usize;
                }
                scores[player - 1] += board[idx];
                board.remove(idx);
                continue;
            }

            // we've reached the end of the board
            if idx == (board.len() - 1) {
                idx = 1;
                board.insert(idx, marble);
                continue;
            }

            // we're neither at the start or end.
            idx += 2;
            board.insert(idx, marble);

            // no more marbles
            if marble >= marbles as u64 {
                break 'out;
            }
        }
    }
    *scores.iter().max().unwrap()
}

fn main() {
    println!("part 1 {}", calculate(459, 71790));
    // constantly shifting numbers up and down in a large vec
    // is very slow, must be a better way of doing this!
    // linked list comes to mind, but in rust?
    println!("part 2 {}", calculate(459, 7179000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(calculate(9, 23), 32);
        assert_eq!(calculate(10, 1618), 8317);
        assert_eq!(calculate(17, 1104), 2764);
        assert_eq!(calculate(21, 6111), 54718);
        assert_eq!(calculate(30, 5807), 37305);
    }

}
