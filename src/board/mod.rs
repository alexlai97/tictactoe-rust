pub enum BoardSize {
    Small,
    Big,
}

use std::fmt;
impl fmt::Display for BoardSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match &self {
                BoardSize::Small => "Small",
                BoardSize::Big => "Big",
            }
        )
    }
}
