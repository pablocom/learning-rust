use object_oriented_programming::design_patterns::player::Player;
use object_oriented_programming::design_patterns::state::{State, StoppedState};

fn main() {
    let mut player = Player::default();
    let mut state: Box<dyn State> = Box::new(StoppedState);

    for action in ["Play", "Play", "Next", "Play", "Stop"] {
        state = match action {
            "Play" => state.play(&mut player),
            "Stop" => state.stop(&mut player),
            "Prev" => state.prev(&mut player),
            "Next" => state.next(&mut player),
            _ => unreachable!(),
        };

        println!("{}", state.render(&player));
    }
}
