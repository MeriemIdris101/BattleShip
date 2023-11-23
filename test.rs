use battleship::Battleship;

#[test]
fn test_new_game() {
    let game = Battleship::new();

    assert_eq!(game.player_ships.len(), 3);
    assert_eq!(game.computer_ships.len(), 3);
}

#[test]
fn test_player_turn_hit() {
    let mut game = Battleship::new();
    let coordinates = game.computer_ships.iter().next().cloned().unwrap();

    game.board_player[coordinates.0][coordinates.1] = 'S';

    game.turn_of_player();

    assert!(game.computer_ships.is_empty());
    assert_eq!(game.player_board[coordinates.0][coordinates.1], 'X');
}

#[test]
fn test_player_turn_miss() {
    let mut game = Battleship::new();
    let coordinates = (
        rand::random::<usize>() % game.computer_board.len(),
        rand::random::<usize>() % game.computer_board[0].len(),
    );

    game.board_player[coordinates.0][coordinates.1] = ' ';

    game.turn_of_player();

    assert!(game.computer_ships.contains(&coordinates));
    assert_eq!(game.player_board[coordinates.0][coordinates.1], 'O');
}

