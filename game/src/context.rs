use std::cell::RefCell;

use crate::{GameConfig, utilities::non_ref_raylib::HackedRaylibHandle};


#[derive(Debug)]
pub struct GameContext {
    pub renderer: RefCell<HackedRaylibHandle>,
    pub config: GameConfig
}

// impl GameContext {
//     /// Construct a new game context.
//     pub fn new(raylib: RefCell<HackedRaylibHandle>) -> Self {
//         Self {
//             renderer: raylib
//         }
//     }
// }
