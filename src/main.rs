use slint;
use std::cell::RefCell; 
use std::rc::Rc;

use rand::{seq::SliceRandom, thread_rng};
slint::include_modules!();
fn main() {
    let window = AppWindow::new().unwrap();
    let pc_pick = Rc::new(RefCell::new(1));

    let picked = window.as_weak();
    let pc_clone = Rc::clone(&pc_pick); // Rc clone of `pc_pick`

    window.on_picked(move |weapon| {
        let mut pc_pick = pc_clone.borrow_mut();
        *pc_pick = 2;
    });

    window.run().unwrap();
    println!("Hello, world!");
}
