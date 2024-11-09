use slint;
use std::cell::RefCell; 
use std::rc::Rc;

use rand::{seq::SliceRandom, thread_rng};
slint::include_modules!();
fn main() {
    let window = AppWindow::new().unwrap();
    let pc_pick = Rc::new(RefCell::new(1));
    let matches_left = Rc::new(RefCell::new(0));

    let picked = window.as_weak();
    let pc_clone = Rc::clone(&pc_pick); // Rc clone of `pc_pick`

    window.on_picked(move |weapon| {
        let mut pc_pick = pc_clone.borrow_mut();
        *pc_pick = 2;

        println!("You picked {weapon}!! i see ya :D");
    });


    let matched = window.as_weak();
    let match_clone = Rc::clone(&matches_left);
    window.on_match_selected(move |matches| {
        let app = matched.upgrade().unwrap();
        *match_clone.borrow_mut() = matches;
        app.set_next_round_opacity(0.0);
        app.set_start_passive(true);
        println!("match {matches}");
    });

    window.run().unwrap();
    println!("Hello, world!");
}
