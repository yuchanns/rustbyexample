const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];
pub(crate) const BOARD: [[usize; 3]; 3] = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];

pub fn calculate_winner(squares: [&str; 9]) -> &str {
    let mut winner = "";
    for [a, b, c] in LINES.iter() {
        if squares[*a] != "" && squares[*a] == squares[*b] && squares[*a] == squares[*c] {
            winner = squares[*a];
            break;
        }
    }
    winner
}
