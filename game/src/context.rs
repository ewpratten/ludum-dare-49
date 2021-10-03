use std::{cell::RefCell, sync::mpsc::Sender};

use discord_sdk::activity::ActivityBuilder;

use crate::{GameConfig, utilities::non_ref_raylib::HackedRaylibHandle};


#[derive(Debug)]
pub enum ControlFlag {
    Quit
}

#[derive(Debug)]
pub struct GameContext {
    pub renderer: RefCell<HackedRaylibHandle>,
    pub config: GameConfig,
    pub discord_rpc_send: Sender<Option<ActivityBuilder>>,
    pub flag_send: Sender<Option<ControlFlag>>
}

// impl GameContext {
//     /// Construct a new game context.
//     pub fn new(raylib: RefCell<HackedRaylibHandle>) -> Self {
//         Self {
//             renderer: raylib
//         }
//     }
// }
