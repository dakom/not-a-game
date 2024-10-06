use std::sync::{atomic::AtomicBool, LazyLock};

use awsm_web::{env::env_var, prelude::UnwrapExt};
use serde::Deserialize;

use crate::{dom::ui::game::GameUiPhase, enemy::data::EnemyKind};


cfg_if::cfg_if! {
    if #[cfg(feature = "dev")] {
        // for github pages etc. where website isn't at root
        pub const URI_ROOT:&'static str = "";
        pub const CONFIG: LazyLock<Config> = LazyLock::new(|| {
            Config {
                image_base: "http://127.0.0.1:9000/image".to_string(),
                audio_base: "http://127.0.0.1:9000/audio".to_string(),
                html_base: "http://127.0.0.1:9000/html".to_string(),
                max_bg_panes: Some(1),
                max_bg_layers: None,
                initial_drop_countdown: 100.0,
                live_drop_countdown_range: 100.0..200.0,
                //live_drop_countdown_range: 1000.0..5000.0,
                selected_enemy: Some(EnemyKind::Two), 
                can_debug_colliders: true,
                cell_duration: 50.0,
                initial_game_phase: Some(GameUiPhase::Welcome)
                //initial_game_phase: None 
            }
        });
    } else {
            // for github pages etc. where website isn't at root
            pub const URI_ROOT:&'static str = "not-a-game";
            pub const CONFIG: LazyLock<Config> = LazyLock::new(|| {
            Config {
                image_base: format!("/{}/media/image", URI_ROOT),
                audio_base: format!("/{}/media/audio", URI_ROOT),
                html_base: format!("/{}/media/html", URI_ROOT),
                initial_drop_countdown: 100.0,
                live_drop_countdown_range: 100.0..200.0,
                max_bg_panes: Some(1),
                max_bg_layers: None,
                selected_enemy: Some(EnemyKind::Two), 
                can_debug_colliders: false,
                cell_duration: 50.0,
                initial_game_phase: Some(GameUiPhase::Welcome)
            }
        });
    }
}

#[derive(Debug)]
pub struct Config {
    image_base: String,
    audio_base: String,
    html_base: String,
    pub initial_drop_countdown: f64,
    pub live_drop_countdown_range: std::ops::Range<f64>,
    pub max_bg_panes: Option<usize>,
    pub max_bg_layers: Option<usize>,
    pub selected_enemy: Option<EnemyKind>,
    pub can_debug_colliders: bool,
    pub cell_duration: f64,
    pub initial_game_phase: Option<GameUiPhase>,

}

impl Config {
    pub fn image_url(&self, path: &str) -> String {
        format!("{}/{}", self.image_base, path)
    }
    pub fn audio_url(&self, path: &str) -> String {
        format!("{}/{}", self.audio_base, path)
    }

    pub fn html_url(&self, path: &str) -> String {
        format!("{}/{}", self.html_base, path)
    }
}

fn check_env(name: &str) -> Option<String> {
    match env_var(name) {
        Ok(value) => if value.is_empty() { None } else { Some(value) },
        Err(_) => None
    }
}
