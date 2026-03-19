use leptos::prelude::*;
use crate::state::{AppState, Screen};
use crate::particles::UnderwaterScene;

#[component]
pub fn WelcomeScreen() -> impl IntoView {
    let state = expect_context::<AppState>();

    let dive_in = move |_| {
        state.navigate(Screen::Hub);
    };

    view! {
        <div class="screen welcome-screen">
            <UnderwaterScene bubble_count=20 seaweed_count=8 fish_count=4 />
            <div class="welcome-content">
                <h1 class="welcome-title">"Happy Birthday!"</h1>
                <p class="welcome-subtitle">"A special underwater adventure awaits..."</p>
                <button class="btn-primary dive-btn" on:click=dive_in>
                    "Dive In!"
                </button>
            </div>
        </div>
    }
}
