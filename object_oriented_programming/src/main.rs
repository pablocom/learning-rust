use object_oriented_programming::design_patterns::state::{Player, State, StoppedState};

fn main() {
    state_pattern_playground()
}

fn state_pattern_playground() {
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
