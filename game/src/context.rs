use std::{cell::RefCell, sync::mpsc::Sender};

use chrono::{DateTime, Duration, Utc};
use discord_sdk::activity::ActivityBuilder;

use crate::{progress::ProgressData, utilities::non_ref_raylib::HackedRaylibHandle, GameConfig};

#[derive(Debug)]
pub enum ControlFlag {
    Quit,
    SwitchLevel(usize),
    UpdateLevelStart(DateTime<Utc>),
    SaveProgress,
    MaybeUpdateHighScore(usize, Duration)
}

#[derive(Debug)]
pub struct GameContext {
    pub renderer: RefCell<HackedRaylibHandle>,
    pub config: GameConfig,
    pub player_progress: ProgressData,
    pub current_level: usize,
    pub level_start_time: DateTime<Utc>,
    pub discord_rpc_send: Sender<Option<ActivityBuilder>>,
    pub flag_send: Sender<Option<ControlFlag>>,
}
