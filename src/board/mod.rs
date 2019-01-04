pub enum BoardSize {
    Small,
    Big,
}

use std::fmt;
impl fmt::Display for BoardSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                BoardSize::Small => "Small",
                BoardSize::Big => "Big",
            }
        )
    }
}

use super::game::Players;
pub struct Board {
    pub board: [Option<Players>; 9],
    matches: Matches,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            board: [None; 9],
            matches: Default::default(),
        }
    }
}

impl Board {
    pub fn clear(&mut self) {
        self.board = [None; 9];
        self.matches = Default::default();
    }
    pub fn set_coordinate(
        &mut self,
        x: usize,
        y: usize,
        player: Players,
    ) -> Result<(), &'static str> {
        if x < 3 && y < 3 {
            let cell = &mut self.board[3 * y + x];
            if cell.is_some() {
                Err("Already occupied")
            } else {
                *cell = Some(player);
                Ok(())
            }
        } else {
            Err("Index out of range.")
        }
    }

    pub fn get_coordinate(&mut self, x: usize, y: usize) -> Result<Option<Players>, &'static str> {
        if x < 3 && y < 3 {
            Ok(self.board[3 * y + x])
        } else {
            Err("Index out of range.")
        }
    }

    pub fn has_match_line(&self) -> Option<Players> {
        match self.matches.has_match_line() {
            Some(i) => self.board[i],
            None => None,
        }
    }

    pub fn print_matches(&self, num_of_spaces: usize) {
        self.matches.print_matches(num_of_spaces);
    }

    pub fn is_full(&self) -> bool {
        for i in self.board.iter() {
            if i.is_none() {
                return false;
            }
        }
        true
    }

    pub fn update_matches(&mut self) {
        if self.board[0] != None && self.board[1] == self.board[0] && self.board[2] == self.board[0]
        {
            self.matches.rows.0 = true;
        }
        if self.board[3] != None && self.board[4] == self.board[3] && self.board[5] == self.board[3]
        {
            self.matches.rows.1 = true;
        }
        if self.board[6] != None && self.board[7] == self.board[6] && self.board[8] == self.board[6]
        {
            self.matches.rows.2 = true;
        }
        if self.board[0] != None && self.board[3] == self.board[0] && self.board[6] == self.board[0]
        {
            self.matches.cols.0 = true;
        }
        if self.board[1] != None && self.board[4] == self.board[1] && self.board[7] == self.board[1]
        {
            self.matches.cols.1 = true;
        }
        if self.board[2] != None && self.board[5] == self.board[2] && self.board[8] == self.board[2]
        {
            self.matches.cols.2 = true;
        }
        if self.board[0] != None && self.board[4] == self.board[0] && self.board[8] == self.board[0]
        {
            self.matches.diags.0 = true;
        }
        if self.board[2] != None && self.board[4] == self.board[2] && self.board[6] == self.board[2]
        {
            self.matches.diags.1 = true;
        }
    }
}

struct Matches {
    rows: (bool, bool, bool),
    cols: (bool, bool, bool),
    diags: (bool, bool),
}

impl Default for Matches {
    fn default() -> Matches {
        Matches {
            rows: (false, false, false),
            cols: (false, false, false),
            diags: (false, false),
        }
    }
}

impl Matches {
    fn has_match_line(&self) -> Option<usize> {
        if self.rows.0 {
            Some(0)
        } else if self.rows.1 {
            Some(3)
        } else if self.rows.2 {
            Some(6)
        } else if self.cols.0 {
            Some(0)
        } else if self.cols.1 {
            Some(1)
        } else if self.cols.2 {
            Some(2)
        } else if self.diags.0 {
            Some(0)
        } else if self.diags.1 {
            Some(2)
        } else {
            None
        }
    }

    fn print_matches(&self, num_of_spaces: usize) {
        let spaces = " ".repeat(num_of_spaces);
        println!("Matches:");
        if self.rows.0 {
            println!("{}row 0", spaces)
        };
        if self.rows.1 {
            println!("{}row 1", spaces)
        };
        if self.rows.2 {
            println!("{}row 2", spaces)
        };
        if self.cols.0 {
            println!("{}column 0", spaces)
        };
        if self.cols.1 {
            println!("{}column 1", spaces)
        };
        if self.cols.2 {
            println!("{}column 2", spaces)
        };
        if self.diags.0 {
            println!("{}Top-left-bottom-right diagonal", spaces)
        };
        if self.diags.1 {
            println!("{}Bottom-left-top-right diagonal", spaces)
        };
    }
}

#[cfg(test)]
mod tests;
