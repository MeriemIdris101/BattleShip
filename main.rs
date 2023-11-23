use std::io;
use std::collections::HashSet;

struct Battleship {
    board_player: Vec<Vec<char>>,
    board_computer: Vec<Vec<char>>,
    ships_player: HashSet<(usize, usize)>,
    ships_computer: HashSet<(usize, usize)>,
}

impl Battleship {
    fn new() -> Self {
        let board_size = 8;
        let mut board_player = vec![vec![' '; board_size]; board_size];
        let mut board_computer = vec![vec![' '; board_size]; board_size]; 


        let ships_player = Battleship::ships_place(&mut board_player);
        let ships_computer = Battleship::ships_place(&mut board_computer);


        Battleship {
            board_player,
            board_computer,
            ships_player,
            ships_computer,
        }
    }

    fn ships_place(board: &mut Vec<Vec<char>>) -> HashSet<(usize, usize)> {
        let mut ships = HashSet::new();

        for _ in 0..3 {
            let mut rand_row;
            let mut rand_col;


            loop {
                rand_row = rand::random::<usize>() % board.len();
                rand_col = rand::random::<usize>() % board[0].len();
                if board[rand_row][rand_col] == ' ' {
                    break;
                }
            }


            ships.insert((rand_row, rand_col));
            board[rand_row][rand_col] = 'S';
        }


        ships
    }

    fn board_display(board: &[Vec<char>]) {
        for row in board {
            println!("{}", row.iter().collect::<String>());
        }
    }

    fn turn_of_player(&mut self) {
        println!("Your turn! Enter your coordinates (row and column separated by space):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let coordinates: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);

            if row < self.board_player.len() && col < self.board_player[0].len() {
                if self.ships_computer.contains(&(row, col)) {
                    println!("It's a Hit! Great job!");
                    self.ships_computer.remove(&(row, col));
                    self.board_player[row][col] = 'X';
                } else {
                    println!("Its a Miss...Better luck next time!");
                    self.board_player[row][col] = 'O';
                }
            } else {
                println!("Try again. Coordinates are invalid. ");
            }
        } else {
            println!("Input is Invalid. Enter row and column separated by space.");
        }
    }

    fn turn_of_computer(&mut self) {
        let mut rand_row;  
        let mut rand_col; 

        loop {
            rand_row = rand::random::<usize>() % self.board_player.len();
            rand_col = rand::random::<usize>() % self.board_player[0].len();
            if self.board_player[rand_row][rand_col] == ' ' {
                break;
            }
        }

        println!("Computer's turn: Row {}, Column {}", rand_row, rand_col);

        if self.ships_player.contains(&(rand_row, rand_col)) {
            println!("Ah! You have been hit!");
            self.ships_player.remove(&(rand_row, rand_col));
            self.board_computer[rand_row][rand_col] = 'X';
        } else {
            println!("Computer missed!");
            self.board_computer[rand_row][rand_col] = 'O';
        }
    }

    fn check_winner(&self) -> Option<&str> {
        if self.ships_computer.is_empty() {
            Some("Congrats! You sank all computer ships. Yay! You win!")
        } else if self.ships_player.is_empty() {
            Some("Oh no! Computer sank all your ships. You lose...!")
        } else {
            None
        }
    }
}

fn main() {
    let mut game = Battleship::new();

    loop {
        println!("\nYour Board:");
        Battleship::board_display(&game.board_player);

        println!("\nComputer's Board:");
        Battleship::board_display(&game.board_computer);

        game.turn_of_player();

        if let Some(result) = game.check_winner() {
            println!("{}", result);
            break;
        }

        game.turn_of_computer();

        if let Some(result) = game.check_winner() {
            println!("{}", result);
            break;
        }
    }
}

