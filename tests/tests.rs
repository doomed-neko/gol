use gol::Game;

#[test]
fn test_game_creation() {
    let grid = vec![false; 12];
    let game = Game::new(grid.clone(), 4, 3);

    assert_eq!(game.cols, 4);
    assert_eq!(game.rows, 3);
    assert_eq!(game.grid.len(), 12);
    assert_eq!(game.grid, grid);
}

#[test]
fn test_index_from_cords_standard() {
    let game = Game::new(vec![false; 12], 4, 3);

    // Standard within-bounds mappings
    assert_eq!(game.index_from_cords(0, 0), 0);
    assert_eq!(game.index_from_cords(3, 0), 3);
    assert_eq!(game.index_from_cords(0, 1), 4);
    assert_eq!(game.index_from_cords(2, 1), 6);
    assert_eq!(game.index_from_cords(3, 2), 11);
}

#[test]
fn test_index_from_cords_wrapping() {
    let game = Game::new(vec![false; 12], 4, 3);

    // Wrapping negative coordinates (left and top borders)
    assert_eq!(game.index_from_cords(-1, 0), 3);
    assert_eq!(game.index_from_cords(0, -1), 8);
    assert_eq!(game.index_from_cords(-1, -1), 11);

    // Wrapping overflow coordinates (right and bottom borders)
    assert_eq!(game.index_from_cords(4, 0), 0);
    assert_eq!(game.index_from_cords(0, 3), 0);
    assert_eq!(game.index_from_cords(4, 3), 0);
}

#[test]
fn test_cords_from_index() {
    let game = Game::new(vec![false; 12], 4, 3);

    assert_eq!(game.cords_from_index(0), (0, 0));
    assert_eq!(game.cords_from_index(3), (3, 0));
    assert_eq!(game.cords_from_index(4), (0, 1));
    assert_eq!(game.cords_from_index(6), (2, 1));
    assert_eq!(game.cords_from_index(11), (3, 2));
}

#[test]
fn test_next_cell_state_game_of_life_rules() {
    // 3x3 grid setup with a vertical line of 3 alive cells (blinker pattern)
    // . 1 .
    // . 1 .
    // . 1 .
    let initial_grid = vec![false, true, false, false, true, false, false, true, false];
    let game = Game::new(initial_grid, 3, 3);

    // Center cell (1, 1) index 4: has 2 neighbors -> stays alive
    assert!(game.next_cell_state(4));

    // Top-center cell (1, 0) index 1: center cell + wraps to bottom/top
    // Check dead cells that should become alive due to 3 neighbors
    assert!(game.next_cell_state(3)); // (0, 1) should become alive
    assert!(game.next_cell_state(5)); // (2, 1) should become alive
}

#[test]
fn test_next_gen_blinker_oscillator() {
    // Vertical blinker on a 3x3 grid
    // F T F
    // F T F
    // F T F
    let vertical = vec![false, true, false, false, true, false, false, true, false];

    // Expected horizontal blinker after 1 generation
    // F F F
    // T T T
    // F F F
    let horizontal = vec![false, false, false, true, true, true, false, false, false];

    let mut game = Game::new(vertical.clone(), 3, 3);

    // Step to generation 1
    game.next_gen();
    assert_eq!(game.grid, horizontal);

    // Step back to generation 2 (should return to initial vertical state)
    game.next_gen();
    assert_eq!(game.grid, vertical);
}

#[test]
fn test_next_gen_still_life_block() {
    // 2x2 block (still life pattern that shouldn't change)
    let block = vec![true, true, false, true, true, false, false, false, false];

    let mut game = Game::new(block.clone(), 3, 3);
    game.next_gen();

    assert_eq!(game.grid, block);
}
