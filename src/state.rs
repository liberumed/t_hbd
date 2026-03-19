use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const STORAGE_KEY: &str = "t_hbd_state";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActivityId {
    PearlWisdom,
    CurrentRider,
    CoralGarden,
    DeepSeaLights,
    TreasureHunt,
}

impl ActivityId {
    pub fn all() -> [ActivityId; 5] {
        [
            Self::PearlWisdom,
            Self::CurrentRider,
            Self::CoralGarden,
            Self::DeepSeaLights,
            Self::TreasureHunt,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::PearlWisdom => "Pearls of Wisdom",
            Self::CurrentRider => "Ride the Current",
            Self::CoralGarden => "Coral Garden",
            Self::DeepSeaLights => "Deep Sea Lights",
            Self::TreasureHunt => "The Treasure Chest",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Screen {
    Welcome,
    Hub,
    Activity(ActivityId),
    Finale,
}

#[derive(Serialize, Deserialize)]
struct SavedState {
    completed: HashSet<ActivityId>,
    current_screen: Screen,
    audio_enabled: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct AppState {
    pub current_screen: RwSignal<Screen>,
    pub completed: RwSignal<HashSet<ActivityId>>,
    pub audio_enabled: RwSignal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        let saved = load_state();

        let state = Self {
            current_screen: RwSignal::new(saved.current_screen),
            completed: RwSignal::new(saved.completed),
            audio_enabled: RwSignal::new(saved.audio_enabled),
        };

        Effect::new(move |_| {
            let saved = SavedState {
                completed: state.completed.get(),
                current_screen: state.current_screen.get(),
                audio_enabled: state.audio_enabled.get(),
            };
            save_state(&saved);
        });

        state
    }

    pub fn navigate(&self, screen: Screen) {
        self.current_screen.set(screen);
    }

    pub fn complete_activity(&self, id: ActivityId) {
        self.completed.update(|set| {
            set.insert(id);
        });
    }

    pub fn all_completed(&self) -> bool {
        self.completed.get().len() == ActivityId::all().len()
    }

    pub fn is_completed(&self, id: &ActivityId) -> bool {
        self.completed.get().contains(id)
    }

    pub fn reset(&self) {
        self.completed.set(HashSet::new());
        self.current_screen.set(Screen::Welcome);
        self.audio_enabled.set(true);
        clear_storage();
    }
}

fn load_state() -> SavedState {
    let window = web_sys::window().expect("no window");
    let storage = window
        .local_storage()
        .ok()
        .flatten();

    if let Some(storage) = storage {
        if let Ok(Some(json)) = storage.get_item(STORAGE_KEY) {
            if let Ok(saved) = serde_json::from_str::<SavedState>(&json) {
                return saved;
            }
        }
    }

    SavedState {
        completed: HashSet::new(),
        current_screen: Screen::Welcome,
        audio_enabled: true,
    }
}

fn save_state(state: &SavedState) {
    let window = web_sys::window().expect("no window");
    if let Some(storage) = window.local_storage().ok().flatten() {
        if let Ok(json) = serde_json::to_string(state) {
            let _ = storage.set_item(STORAGE_KEY, &json);
        }
    }
}

fn clear_storage() {
    let window = web_sys::window().expect("no window");
    if let Some(storage) = window.local_storage().ok().flatten() {
        let _ = storage.remove_item(STORAGE_KEY);
    }
}
