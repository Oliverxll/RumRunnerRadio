use std::{default, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum PlaybackState {
    Playing,
    #[default]
    Paused,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum RepeatMode {
    #[default]
    Off,
    One,
    All,
    Shuffle,
}

pub struct Track {
    pub id: String,
    pub title: String,
    pub duration: Duration,
}

pub struct Player {
    queue: Vec<Track>,
    current_index: Option<usize>,
    state: PlaybackState,
    position: Duration,
    volume: u8,
    muted: bool,
    repeat: RepeatMode,
}

impl Player {
    pub fn load_queue(&mut self, tracks: Vec<Track>, start_index: usize) {
        self.queue = tracks;

        self.current_index = if self.queue.is_empty() {
            None
        } else {
            Some(start_index.min(self.queue.len() - 1))
        };

        self.position = Duration::ZERO;
        self.state = PlaybackState::Paused;
    }

    pub fn play(&mut self) {
        if self.current_index.is_none() && !self.queue.is_empty() {
            self.current_index = Some(0);
            self.position = Duration::ZERO;
        }

        if self.current_index.is_some() {
            self.state = PlaybackState::Playing;
        }
    }

    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }

    pub fn toggle_play_pause(&mut self) {
        match self.state {
            PlaybackState::Playing => self.pause(),
            _ => self.play(),
        }
    }

    pub fn next(&mut self) {
        let Some(i) = self.current_index else { return };

        if self.queue.is_empty() {
            self.current_index = None;
            self.state = PlaybackState::Stopped;
            self.position = Duration::ZERO;
            return;
        }

        match self.repeat {
            RepeatMode::Off => {
                if i + 1 < self.queue.len() {
                    self.current_index = Some(i + 1);
                    self.position = Duration::ZERO;
                } else {
                    self.state = PlaybackState::Stopped;
                    self.position = Duration::ZERO;
                }
            }
            RepeatMode::One => {
                self.position = Duration::ZERO;
            }
            RepeatMode::All => {
                self.current_index = Some((i + 1) % self.queue.len());
                self.position = Duration::ZERO;
            }
            RepeatMode::Shuffle => {
                todo!()
            }
        }
    }

    pub fn prev(&mut self) {
        let Some(i) = self.current_index else { return };

        if self.position > Duration::from_secs(3) {
            self.position = Duration::ZERO;
            return;
        }

        if i > 0 {
            self.current_index = Some(i - 1);
            self.position = Duration::ZERO;
        } else {
            self.current_index = Some(self.queue.len() - 1);
            self.position = Duration::ZERO;
        }
    }

    pub fn seek_to(&mut self, pos: Duration) {
        let max = self
            .current_track()
            .map(|t| t.duration)
            .unwrap_or(Duration::ZERO);
        self.position = pos.min(max);
    }

    pub fn seek_by(&mut self, delta_specs: i64) {}

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(100);
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    pub fn set_repeat(&mut self, mode: RepeatMode) {
        self.repeat = mode;
    }

    pub fn current_track(&self) -> Option<&Track> {
        self.current_index.and_then(|i| self.queue.get(i))
    }

    pub fn state(&self) -> PlaybackState {
        self.state
    }

    pub fn position(&self) -> Duration {
        self.position
    }

    pub fn volume(&self) -> u8 {
        self.volume
    }

    pub fn muted(&self) -> bool {
        self.muted
    }
}
