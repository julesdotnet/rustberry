use std::sync::{Arc, Mutex, OnceLock};

pub struct ApplicationState {
    pub bpm: i32,
    pub song_playing: bool,
    pub show_pattern_editor: bool,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self { bpm: 130, song_playing: false, show_pattern_editor: false }
    }
}

static APP_STATE: OnceLock<Arc<Mutex<ApplicationState>>> = OnceLock::new();

pub fn app_state() -> &'static Arc<Mutex<ApplicationState>> {
    APP_STATE.get_or_init(|| Arc::new(Mutex::new(ApplicationState::default())))
}
