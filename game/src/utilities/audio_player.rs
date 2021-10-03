use raylib::audio::RaylibAudio;

/// A thin wrapper around `raylib::core::audio::RaylibAudio` that keeps track of the volume of its audio channels.
pub struct AudioPlayer {
    backend: RaylibAudio,

    // Volume
    pub master_volume: f32,
}

impl AudioPlayer {
    /// Construct an AudioPlayer around a RaylibAudio
    pub fn new(backend: RaylibAudio) -> Self {
        Self {
            backend,
            master_volume: 1.0,
        }
    }

    /// Set the master volume for all tracks. `0.0` to `1.0`
    pub fn set_master_volume(&mut self, volume: f32) {
        // The volume must be 0-1
        let volume = volume.clamp(0.0, 1.0);

        // Set the volume
        self.master_volume = volume;
        self.backend.set_master_volume(volume);
    }

    /// Get the master volume
    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }
}

impl std::ops::Deref for AudioPlayer {
    type Target = RaylibAudio;
    fn deref(&self) -> &Self::Target {
        &self.backend
    }
}


impl std::ops::DerefMut for AudioPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.backend
    }
}
