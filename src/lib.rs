/// Calculates the winner of a round.
/// 
/// # Examples
/// ```
/// let winner = rpsgame_playground::get_winner();
/// assert_eq!(winner, true);
/// ```
pub fn get_winner() -> bool {
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Test the game logic.
    #[test]
    fn t_get_winner() {
        assert_eq!(get_winner(), true);
    }
}