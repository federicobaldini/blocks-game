use rand::prelude::*;
use std::io;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[derive(Copy, Clone)]
struct Cell {
  active: bool,
  color: u8,
}

impl Cell {
  pub fn new(active: bool, color: u8) -> Cell {
    Cell { active, color }
  }

  pub fn active(&self) -> bool {
    self.active
  }

  pub fn set_active(&mut self, value: bool) {
    self.active = value;
  }

  pub fn color(&self) -> u8 {
    self.color
  }

  pub fn set_color(&mut self, value: u8) {
    self.color = value;
  }
}

#[derive(Clone)]
struct Piece {
  blocks: Vec<Vec<bool>>,
  color: u8,
}

impl Piece {
  fn new(blocks: Vec<Vec<bool>>, color: u8) -> Piece {
    Piece { blocks, color }
  }

  fn rotate(&mut self, board: &[[Cell; WIDTH]; HEIGHT], x: usize, y: usize) {
    let mut new_blocks: Vec<Vec<bool>> = vec![vec![false; self.blocks.len()]; self.blocks[0].len()];
    let mut collision_detected: bool = false;
    for i in 0..self.blocks.len() {
      for j in 0..self.blocks[0].len() {
        new_blocks[j][self.blocks.len() - i - 1] = self.blocks[i][j];
      }
    }
    for i in x..(x + new_blocks[0].len()) {
      for j in y..(y + new_blocks.len()) {
        if i < WIDTH && j < HEIGHT {
          if board[j][i].active() {
            collision_detected = true;
            break;
          }
        }
      }
    }
    if !collision_detected {
      self.blocks = new_blocks;
    }
  }

  pub fn blocks(&self) -> &Vec<Vec<bool>> {
    &self.blocks
  }
}

fn random_piece() -> Piece {
  let pieces: [Piece; 28] = [
    /*
     [#][#][#][#]
     [ ][ ][ ][ ]
     [ ][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(vec![vec![true, true, true, true]], 1),
    // Added one more to ensure equiprobability
    Piece::new(vec![vec![true, true, true, true]], 1),
    /*
     [#][ ][ ][ ]
     [#][ ][ ][ ]
     [#][ ][ ][ ]
     [#][ ][ ][ ]
    */
    Piece::new(vec![vec![true], vec![true], vec![true], vec![true]], 1),
    // Added one more to ensure equiprobability
    Piece::new(vec![vec![true], vec![true], vec![true], vec![true]], 1),
    /*
     [ ][#][ ][ ]
     [#][#][#][ ]
     [ ][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(vec![vec![false, true, false], vec![true, true, true]], 2),
    /*
     [#][ ][ ][ ]
     [#][#][ ][ ]
     [#][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(
      vec![vec![true, false], vec![true, true], vec![true, false]],
      2,
    ),
    /*
     [#][#][#][ ]
     [ ][#][ ][ ]
     [ ][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(vec![vec![true, true, true], vec![false, true, false]], 2),
    /*
     [ ][#][ ][ ]
     [#][#][ ][ ]
     [ ][#][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(
      vec![vec![false, true], vec![true, true], vec![false, true]],
      2,
    ),
    /*
     [#][#][ ][ ]
     [#][#][ ][ ]
     [ ][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(vec![vec![true, true], vec![true, true]], 3),
    // Added three more to ensure equiprobability
    Piece::new(vec![vec![true, true], vec![true, true]], 3),
    Piece::new(vec![vec![true, true], vec![true, true]], 3),
    Piece::new(vec![vec![true, true], vec![true, true]], 3),
    /*
     [#][#][ ][ ]
     [ ][#][#][ ]
     [ ][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(vec![vec![true, true, false], vec![false, true, true]], 4),
    // Added one more to ensure equiprobability
    Piece::new(vec![vec![true, true, false], vec![false, true, true]], 4),
    /*
     [ ][#][ ][ ]
     [#][#][ ][ ]
     [#][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(
      vec![vec![false, true], vec![true, true], vec![true, false]],
      4,
    ),
    // Added one more to ensure equiprobability
    Piece::new(
      vec![vec![false, true], vec![true, true], vec![true, false]],
      4,
    ),
    /*
     [ ][#][#][ ]
     [#][#][ ][ ]
     [ ][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(vec![vec![false, true, true], vec![true, true, false]], 5),
    // Added one more to ensure equiprobability
    Piece::new(vec![vec![false, true, true], vec![true, true, false]], 5),
    /*
     [#][ ][ ][ ]
     [#][#][ ][ ]
     [ ][#][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(
      vec![vec![true, false], vec![true, true], vec![false, true]],
      5,
    ),
    // Added one more to ensure equiprobability
    Piece::new(
      vec![vec![true, false], vec![true, true], vec![false, true]],
      5,
    ),
    /*
     [ ][ ][#][ ]
     [#][#][#][ ]
     [ ][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(vec![vec![false, false, true], vec![true, true, true]], 6),
    /*
     [#][ ][ ][ ]
     [#][ ][ ][ ]
     [#][#][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(
      vec![vec![true, false], vec![true, false], vec![true, true]],
      6,
    ),
    /*
     [#][#][#][ ]
     [#][ ][ ][ ]
     [ ][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(vec![vec![true, true, true], vec![true, false, false]], 6),
    /*
     [#][#][ ][ ]
     [ ][#][ ][ ]
     [ ][#][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(
      vec![vec![true, true], vec![false, true], vec![false, true]],
      6,
    ),
    /*
     [#][ ][ ][ ]
     [#][#][#][ ]
     [ ][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(vec![vec![true, false, false], vec![true, true, true]], 7),
    /*
     [#][#][ ][ ]
     [#][ ][ ][ ]
     [#][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(
      vec![vec![true, true], vec![true, false], vec![true, false]],
      7,
    ),
    /*
     [#][#][#][ ]
     [ ][ ][#][ ]
     [ ][ ][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(vec![vec![true, true, true], vec![false, false, true]], 7),
    /*
     [ ][#][ ][ ]
     [ ][#][ ][ ]
     [#][#][ ][ ]
     [ ][ ][ ][ ]
    */
    Piece::new(
      vec![vec![false, true], vec![false, true], vec![true, true]],
      7,
    ),
  ];
  pieces.choose(&mut thread_rng()).unwrap().clone()
}

// returns the number of completed lines
fn check_line_complete(board: &mut [[Cell; WIDTH]; HEIGHT]) -> u32 {
  let mut active_cells_by_row_count: usize = 0;
  let mut lines_completed_count: u32 = 0;
  let mut row_to_check: usize = HEIGHT - 1;

  while row_to_check > 0 {
    for column in 0..WIDTH {
      if board[row_to_check][column].active() {
        active_cells_by_row_count += 1
      }
    }
    if active_cells_by_row_count == WIDTH {
      for row in (1..=row_to_check).rev() {
        for column in 0..WIDTH {
          board[row][column].set_active(board[row - 1][column].active());
          board[row][column].set_color(board[row - 1][column].color());
        }
      }
      for column in 0..WIDTH {
        board[0][column].set_active(false);
      }
      lines_completed_count += 1;
      row_to_check = HEIGHT - 1;
    } else {
      row_to_check -= 1
    }
    active_cells_by_row_count = 0;
  }

  lines_completed_count
}

fn clear_piece(board: &mut [[Cell; WIDTH]; HEIGHT], piece: &Piece, x: usize, y: usize) {
  for (i, row) in piece.blocks().iter().enumerate() {
    for (j, cell) in row.iter().enumerate() {
      if *cell {
        let x_actual_position: usize = x + j;
        let y_actual_position: usize = y + i;
        if y_actual_position < HEIGHT {
          board[y_actual_position][x_actual_position].set_active(false);
        }
      }
    }
  }
}

fn print_board(board: &[[Cell; WIDTH]; HEIGHT], score: u32) {
  print!("\x1B[2J\x1B[1;1H"); // clear screen
  println!("{:spaces$}Score: {}", "", score, spaces = (WIDTH * 2) + 6);
  for row in board.iter() {
    // print left side
    print!("||");
    for cell in row.iter() {
      if cell.active() {
        // print piece cell
        print!("\x1B[{}m  \x1B[0m", 40 + cell.color());
      } else {
        print!("  ");
      }
    }
    // print right side
    print!("||");
    println!();
  }
  // print container bottom
  for _ in 0..(WIDTH + 2) {
    print!("‾‾");
  }
  println!();
}

fn get_input() -> String {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  input.trim().to_string()
}

fn start_game() {
  let mut board: [[Cell; WIDTH]; HEIGHT] = [[Cell::new(false, 0); WIDTH]; HEIGHT];
  let mut score: u32 = 0;

  loop {
    let mut piece: Piece = random_piece();
    let mut x: usize = 4;
    let mut y: usize = 0;
    let mut x_previous_position: usize = 0;
    let mut y_previous_position: usize = 0;
    let mut game_over: bool = false;
    let mut move_to_bottom: bool = false;

    while !game_over {
      let mut input: String = String::new();

      // Print the game board and wait for user input
      print_board(&board, score);

      if !move_to_bottom {
        input = get_input();
      }

      // clear the piece from its previous position when moving up
      clear_piece(&mut board, &piece, x_previous_position, y_previous_position);

      // Handle user input to move or rotate the piece
      match input.as_str() {
        "a" => x = x.saturating_sub(1),
        "d" => {
          x = (x + 1).min(WIDTH - piece.blocks()[0].len());
        }
        "w" => {
          piece.rotate(&board, x, y);
          x = (x).min(WIDTH - piece.blocks()[0].len());
        }
        "s" => {
          move_to_bottom = true;
        }
        _ => {}
      }

      let mut piece_removed: bool = false;

      // update x_previous_position and y_previous_position after the movement or rotation
      x_previous_position = x;
      y_previous_position = y;

      // Update the game board with the current piece
      for (i, row) in piece.blocks().iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
          if *cell {
            let x_actual_position: usize = x + j;
            let y_actual_position: usize = y + i;

            if y_actual_position == HEIGHT - 1 {
              piece_removed = true;
            }

            if y_actual_position + 1 < HEIGHT
              && board[y_actual_position + 1][x_actual_position].active()
            {
              if y == 0 {
                game_over = true;
              }
              piece_removed = true;
            }

            if y_actual_position < HEIGHT && x_actual_position < WIDTH {
              board[y_actual_position][x_actual_position].set_active(true);
              board[y_actual_position][x_actual_position].set_color(piece.color);
            }
          }
        }
      }

      // Break out of the loop if the piece is removed (i.e. has reached the bottom of the game board)
      if piece_removed {
        // Increase the score if a lines are removed
        let lines_completed: u32 = check_line_complete(&mut board);
        if lines_completed > 0 {
          match lines_completed {
            1 => score += 40,
            2 => score += 100,
            3 => score += 300,
            _ => score += 1200,
          }
        }
        break;
      }

      // Move the piece down one row
      y += 1;
    }

    if game_over {
      print_board(&board, score);
      println!("Game Over!");
      break;
    }
  }
}

fn main() {
  start_game();
}
