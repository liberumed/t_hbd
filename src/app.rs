use leptos::prelude::*;
use crate::state::{ActivityId, AppState, Screen};
use crate::screens::{WelcomeScreen, HubScreen, FinaleScreen};
use crate::activities::{
    PearlWisdomActivity, CurrentRiderActivity, CoralGardenActivity,
    DeepSeaLightsActivity, TreasureHuntActivity,
};
use crate::audio::AudioPlayer;

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

    let current = move || state.current_screen.get();

    view! {
        <div class="app">
            <AudioPlayer />
            {move || match current() {
                Screen::Welcome => view! { <WelcomeScreen /> }.into_any(),
                Screen::Hub => view! { <HubScreen /> }.into_any(),
                Screen::Activity(id) => match id {
                    ActivityId::PearlWisdom => view! { <PearlWisdomActivity /> }.into_any(),
                    ActivityId::CurrentRider => view! { <CurrentRiderActivity /> }.into_any(),
                    ActivityId::CoralGarden => view! { <CoralGardenActivity /> }.into_any(),
                    ActivityId::DeepSeaLights => view! { <DeepSeaLightsActivity /> }.into_any(),
                    ActivityId::TreasureHunt => view! { <TreasureHuntActivity /> }.into_any(),
                },
                Screen::Finale => view! { <FinaleScreen /> }.into_any(),
            }}
        </div>
    }
}
