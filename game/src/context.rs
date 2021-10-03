use std::{cell::RefCell, collections::HashMap, sync::mpsc::Sender};

use chrono::{DateTime, Duration, Utc};
use discord_sdk::activity::ActivityBuilder;
use raylib::audio::Sound;

use crate::{
    progress::ProgressData,
    utilities::{audio_player::AudioPlayer, non_ref_raylib::HackedRaylibHandle},
    GameConfig,
};

#[derive(Debug)]
pub enum ControlFlag {
    Quit,
    SwitchLevel(usize),
    UpdateLevelStart(DateTime<Utc>),
    SaveProgress,
    MaybeUpdateHighScore(usize, Duration),
    SoundTrigger(String),
}

#[derive(Debug)]
pub struct GameContext {
    pub renderer: RefCell<HackedRaylibHandle>,
    pub audio: AudioPlayer,
    pub sounds: HashMap<String, Sound>,
    pub config: GameConfig,
    pub player_progress: ProgressData,
    pub current_level: usize,
    pub total_levels: usize,
    pub level_start_time: DateTime<Utc>,
    pub discord_rpc_send: Sender<Option<ActivityBuilder>>,
    pub flag_send: Sender<Option<ControlFlag>>,
}
