use rand::prelude::*;
use std::io;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[derive(Copy, Clone)]
struct Cell {
  active: bool,
  color: char,
}

impl Cell {
  pub fn new(active: bool, color: char) -> Cell {
    Cell { active, color }
  }

  pub fn active(&self) -> bool {
    self.active
  }

  pub fn set_active(&mut self, value: bool) {
    self.active = value;
  }

  pub fn color(&self) -> char {
    self.color
  }

  pub fn set_color(&mut self, value: char) {
    self.color = value;
  }
}

#[derive(Clone)]
struct Piece {
  blocks: Vec<Vec<bool>>,
  color: char,
}

impl Piece {
  fn new(blocks: Vec<Vec<bool>>, color: char) -> Piece {
    Piece { blocks, color }
  }

  fn rotate(&mut self) {
    let mut new_blocks: Vec<Vec<bool>> = vec![vec![false; self.blocks.len()]; self.blocks[0].len()];
    for i in 0..self.blocks.len() {
      for j in 0..self.blocks[0].len() {
        new_blocks[j][self.blocks.len() - i - 1] = self.blocks[i][j];
      }
    }
    self.blocks = new_blocks;
  }

  pub fn blocks(&self) -> &Vec<Vec<bool>> {
    &self.blocks
  }
}

fn random_piece() -> Piece {
  let pieces: [Piece; 7] = [
    // ####
    Piece::new(vec![vec![true, true, true, true]], '1'),
    //  #
    // ###
    Piece::new(vec![vec![false, true, false], vec![true, true, true]], '2'),
    // ##
    // ##
    Piece::new(vec![vec![true, true], vec![true, true]], '3'),
    // ##
    //  ##
    Piece::new(vec![vec![true, true, false], vec![false, true, true]], '4'),
    //  ##
    // ##
    Piece::new(vec![vec![false, true, true], vec![true, true, false]], '5'),
    //   #
    // ###
    Piece::new(vec![vec![false, false, true], vec![true, true, true]], '6'),
    // #
    // ###
    Piece::new(vec![vec![true, false, false], vec![true, true, true]], '7'),
  ];
  pieces.choose(&mut thread_rng()).unwrap().clone()
}

fn check_line_complete(board: &mut [[Cell; WIDTH]; HEIGHT], y: usize) -> bool {
  for cell in board[y].iter() {
    if !cell.active() {
      return false;
    }
  }
  for i in (1..=y).rev() {
    for j in 0..WIDTH {
      board[i][j].set_active(board[i - 1][j].active());
    }
  }
  for j in 0..WIDTH {
    board[0][j].set_active(false);
  }
  true
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
  println!("Score: {}", score);
  for row in board.iter() {
    for cell in row.iter() {
      if cell.active() {
        print!("\x1b[3{}m#\x1b[0m", cell.color());
      } else {
        print!(".");
      }
    }
    println!();
  }
}

fn get_input() -> String {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  input.trim().to_string()
}

fn start_game() {
  let mut board: [[Cell; WIDTH]; HEIGHT] = [[Cell::new(false, '0'); WIDTH]; HEIGHT];
  let mut score: u32 = 0;

  loop {
    let mut piece: Piece = random_piece();
    let mut x: usize = 3;
    let mut y: usize = 0;
    let mut x_previous_position: usize = 0;
    let mut y_previous_position: usize = 0;
    let mut game_over: bool = false;

    while !game_over {
      // Print the game board and wait for user input
      print_board(&board, score);
      let input: String = get_input();

      // Handle user input to move or rotate the piece
      match input.as_str() {
        "a" => {
          clear_piece(&mut board, &piece, x_previous_position, y_previous_position);
          x = x.saturating_sub(1)
        }
        "d" => {
          clear_piece(&mut board, &piece, x_previous_position, y_previous_position);
          x = (x + 1).min(WIDTH - piece.blocks()[0].len());
        }
        "w" => {
          clear_piece(&mut board, &piece, x_previous_position, y_previous_position);
          piece.rotate();
        }
        _ => {}
      }

      let mut piece_removed: bool = false;
      let mut line_removed: bool = false;

      if y > 0 {
        // also clear the piece from its previous position when moving up
        clear_piece(&mut board, &piece, x, y - 1);
      }

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

            board[y_actual_position][x_actual_position].set_active(true);
            board[y_actual_position][x_actual_position].set_color(piece.color);

            if check_line_complete(&mut board, y_actual_position) {
              line_removed = true;
            }
          }
        }
      }

      // Increase the score if a line is removed
      if line_removed {
        score += 1000;
      }

      // Break out of the loop if the piece is removed (i.e. has reached the bottom of the game board)
      if piece_removed {
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
