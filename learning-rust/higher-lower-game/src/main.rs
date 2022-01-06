use rand::Rng;
use std::cmp::Ordering;

struct Player {
    name: String,
    guess: i8,
}

impl Player {
    fn new() -> Player {
        let mut name = String::new();
        println!("Enter player's name: ");
        let _b = std::io::stdin().read_line(&mut name).unwrap();
        println!("Hello, {}", name);
        Player {
            name: name.trim().to_string(),
            guess: 0,
        }
    }

    fn get_input(&mut self) {
        let mut guess = String::new();
        println!("Enter {}'s guess: ", self.name);
        let _g = std::io::stdin().read_line(&mut guess).unwrap();
        self.guess = guess.trim().parse::<i8>().unwrap();
    }
}

struct Game {
    player1: Player,
    player2: Player,
    random_value: i8,
}

impl Game {
    fn new() -> Game {
        Game {
            player1: Player::new(),
            player2: Player::new(),
            random_value: rand::thread_rng().gen_range(1..101),
        }
    }
    fn gameloop(&mut self) {
        let mut done = false;
        while !done {
            self.player1.get_input();
            self.player2.get_input();

            let p1_diff = (self.player1.guess - self.random_value).abs();
            let p2_diff = (self.player2.guess - self.random_value).abs();

            match p1_diff.cmp(&p2_diff) {
                Ordering::Greater => println!("{} wins!", self.player2.name),
                Ordering::Less => println!("{} wins!", self.player1.name),
                Ordering::Equal => println!("It's a tie!"),
            }

            println!("The true value was {}!", self.random_value);

            let mut play_again = String::new();
            println!("Would you like to play again? (y/n): ");
            let _a = std::io::stdin().read_line(&mut play_again).unwrap();

            if play_again.trim().eq("n") {
                done = true;
            }
        }
        println!("Thanks for playing!")
    }
}

fn main() {
    let mut game = Game::new();
    game.gameloop();
}
