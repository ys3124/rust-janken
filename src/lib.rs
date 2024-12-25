#[derive(Debug, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
pub enum JankenResult {
    Draw,
    Player1Win,
    Player2Win,
}

pub struct Janken {
    player1_hand: Hand,
    player2_hand: Hand,
}

impl Janken {
    pub fn new(player1_hand: Hand, player2_hand: Hand) -> Self {
        Janken {
            player1_hand,
            player2_hand,
        }
    }
    pub fn get_result(&self) -> JankenResult {
        match self.player1_hand {
            Hand::Rock => match self.player2_hand {
                Hand::Rock => JankenResult::Draw,
                Hand::Paper => JankenResult::Player2Win,
                Hand::Scissors => JankenResult::Player1Win,
            },
            Hand::Paper => match self.player2_hand {
                Hand::Rock => JankenResult::Player1Win,
                Hand::Paper => JankenResult::Draw,
                Hand::Scissors => JankenResult::Player2Win,
            },
            Hand::Scissors => match self.player2_hand {
                Hand::Rock => JankenResult::Player2Win,
                Hand::Paper => JankenResult::Player1Win,
                Hand::Scissors => JankenResult::Draw,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_janken_test() {
        let janken = Janken::new(Hand::Paper, Hand::Rock);
        assert_eq!(janken.player1_hand, Hand::Paper);
        assert_eq!(janken.player2_hand, Hand::Rock);
    }

    #[test]
    fn player1_win_test() {
        let janken = Janken::new(Hand::Paper, Hand::Rock);
        assert_eq!(janken.get_result(), JankenResult::Player1Win);
    }

    #[test]
    fn draw_test() {
        let janken = Janken::new(Hand::Rock, Hand::Rock);
        assert_eq!(janken.get_result(), JankenResult::Draw);
    }
}
