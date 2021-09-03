use std::time::{Duration, Instant};
use rand::seq::SliceRandom;

/// A basic timer.
pub struct Timer {
    pub duration: Duration,
    //start: Option<Instant>,
    running: bool,
}

/// A Rubik's cube shuffle.
pub struct Shuffle {
    pub moves: u8,
    pub shuffle: Vec<String>,
    rng: rand::rngs::ThreadRng,
}

impl Timer {
    /// Creates a new timer.
    pub fn new() -> Self {
        Self { duration: Duration::ZERO, running: false }
    }
    
    /// Starts the timer.
    pub fn start(&mut self) {
        let current = Instant::now();
        self.running = true;
        while self.running {
            self.duration = current.elapsed();
        }
    }
    
    /// Stops the timer.
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    /// Resets the timer.
    pub fn reset(&mut self) {
        self.duration = Duration::ZERO;
        self.running = false;
    }
}

impl Shuffle {
    /// The 6 faces on a 3x3 Rubik's cube.
    const FACES: [&'static str; 6] = ["F", "R", "U", "B", "D", "L"];
    
    /// All the possible modifiers that can be added to a "move" in a shuffle: no modifier, a prime/counter-clockwise modifier, and a double modifier.
    const MODIFIERS: [&'static str; 3] = ["", "'", "2"];
    
    /// Create a new shuffle from a desired number of moves.
    pub fn new(moves: u8) -> Self {
        let mut rng = rand::thread_rng();
        let mut shuffle = Vec::new();
        for _ in 0..moves {
            if let Some(face) = Self::FACES.choose(&mut rng) {
                if let Some(modifier) = Self::MODIFIERS.choose(&mut rng) {
                   let mut face_string = String::from(*face);
                   face_string.push_str(modifier);
                   shuffle.push(face_string);
               }
            }
        }
        Self { moves, shuffle, rng }
    }
    
    /// Reshuffle.
    pub fn reshuffle(&mut self) {
        let mut shuffle = Vec::new();
        for _ in 0..self.moves {
            if let Some(face) = Self::FACES.choose(&mut self.rng) {
                if let Some(modifier) = Self::MODIFIERS.choose(&mut self.rng) {
                   let mut face_string = String::from(*face);
                   face_string.push_str(modifier);
                   shuffle.push(face_string);
               }
            }
        }
        self.shuffle = shuffle;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
