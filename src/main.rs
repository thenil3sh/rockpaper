use slint;
use std::cell::RefCell; 
use std::rc::Rc;

use rand::{seq::SliceRandom, thread_rng};
slint::include_modules!();
fn main() {
    let window = AppWindow::new().unwrap();
    let matches_left = Rc::new(RefCell::new(0));
    let pc_score = Rc::new(RefCell::new(0));
    let yt_score = Rc::new(RefCell::new(0));
    let win_count = Rc::new(RefCell::new(0));

    let matches_clone = Rc::clone(&matches_left);
    let picked = window.as_weak(); // Rc clone of `pc_pick`
    let yr_clone = Rc::clone(&yt_score);
    let pc_clone = Rc::clone(&pc_score);
    let win_clone = Rc::clone(&win_count);
    window.on_picked(move |weapon| {
        let mut yr_score = yr_clone.borrow_mut();
        let mut pc_score = pc_clone.borrow_mut();
        let app = picked.upgrade().unwrap();
        let pc_pick = *[0, 1, 2].choose(&mut rand::thread_rng()).unwrap();
        app.set_pc_pick(pc_pick);
        app.set_yr_pick( weapon );
        app.set_winning(
        if pc_pick == 0 && weapon == 1 {
            *yr_score += 1;
            0
        } else if pc_pick == 1 && weapon == 2 {
            *yr_score += 1;
            0
        } else if pc_pick == 2 && weapon == 0 {
            *yr_score += 1;
            0
        } else if pc_pick == 1 && weapon == 0 {
            *pc_score += 1;
            1
        } else if pc_pick == 2 && weapon == 1 {
            *pc_score += 1;
            1
        } else if pc_pick == 0 && weapon == 2 {
            *pc_score += 1;
            1
        } else if pc_pick == weapon {
            *pc_score += 1;
            *yr_score += 1;
            match weapon {
                x => x + 2,
            }
        } else {
            panic!("Again whatt??");
        });
        app.set_pc_score(*pc_score);
        app.set_yr_score(*yr_score);
        *win_clone.borrow_mut() = if *yr_score > *pc_score {
            *yr_score
        } else {
            *pc_score
        };

        if *matches_clone.borrow() == *win_clone.borrow() {
            todo!("lmao ok");
        }

        println!("You picked {}!! i see ya :D and i picked {}", pc_pick, weapon);
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
