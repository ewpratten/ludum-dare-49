use std::collections::HashMap;

use chrono::Duration;
use tracing::info;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ProgressData {
    pub level_best_times: HashMap<usize, i64>,
}

impl ProgressData {
    pub fn get_level_best_time(&self, level: usize) -> Option<Duration> {
        let level_best_time = self.level_best_times.get(&level);
        match level_best_time {
            Some(time) => Some(Duration::seconds(*time)),
            None => None,
        }
    }

    pub fn maybe_write_new_time(&mut self, level: usize, time: &Duration) {
        let time_in_seconds = time.num_seconds();
        if let Some(best_time) = self.get_level_best_time(level) {
            if best_time.num_seconds() > time_in_seconds {
                self.level_best_times.insert(level, time_in_seconds);
            }
        } else {
            self.level_best_times.insert(level, time_in_seconds);
        }
    }

    pub fn load_from_file() -> Self {
        info!("Loading progress data from file");
        serde_json::from_str(
            &std::fs::read_to_string("./savegame.json")
                .unwrap_or("{\"level_best_times\":{}}".to_string()),
        )
        .unwrap_or(Self::default())
    }

    pub fn save(&self) {
        info!("Saving progress data to file");
        std::fs::write("./savegame.json", serde_json::to_string(self).unwrap()).unwrap()
    }
}
