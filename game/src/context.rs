use std::{cell::RefCell, sync::mpsc::Sender};

use chrono::{DateTime, Utc};
use discord_sdk::activity::ActivityBuilder;

use crate::{utilities::non_ref_raylib::HackedRaylibHandle, GameConfig};

#[derive(Debug)]
pub enum ControlFlag {
    Quit,
    SwitchLevel(usize),
    UpdateLevelStart(DateTime<Utc>)
}

#[derive(Debug)]
pub struct GameContext {
    pub renderer: RefCell<HackedRaylibHandle>,
    pub config: GameConfig,
    pub current_level: usize,
    pub level_start_time: DateTime<Utc>,
    pub discord_rpc_send: Sender<Option<ActivityBuilder>>,
    pub flag_send: Sender<Option<ControlFlag>>,
}
