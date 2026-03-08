use stars_gapa::{event::Event, game::Game, input::Input};

#[test]
pub fn input_fuzzer() {
    let nb_thread = 16;
    let nb_event = 200000;

    let handles: Vec<_> = (0..nb_thread)
        .map(|_| {
            std::thread::spawn(move || {
                let mut game = Game::new();
                for _ in 0..nb_event {
                    game.interact(Input::random());
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("This thread has crashed");
    }
}

#[test]
pub fn event_fuzzer() {
    let nb_thread = 16;
    let nb_event = 200000;

    let handles: Vec<_> = (0..nb_thread)
        .map(|_| {
            std::thread::spawn(move || {
                let mut game = Game::new();
                for _ in 0..nb_event {
                    game.apply_event(Event::random());
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("This thread has crashed");
    }
}
