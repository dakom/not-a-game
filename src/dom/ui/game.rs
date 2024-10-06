mod game_over;
mod help;
mod footer;
mod welcome;
mod winner;

use std::collections::HashSet;

use game_over::GameOver;
use gloo_timers::future::TimeoutFuture;
use help::Help;
use wasm_bindgen_futures::spawn_local;
use welcome::Welcome;
use winner::Winner;

use crate::{dispatch_select_event, enemy::{data::{Enemy, EnemyKind}, events::EnemySelectEvent}, prelude::*};

#[derive(Clone)]
pub struct GameUi {
    pub world: Arc<World>,
    pub selected_kind: Mutable<Option<EnemyKind>>,
    pub destroyed_kinds: Mutable<HashSet<EnemyKind>>,
    pub phase: Mutable<Option<GameUiPhase>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameUiPhase {
    Welcome,
    Winner,
    Help,
    GameOver,
}

impl GameUi {
    pub fn new(world:Arc<World>) -> Arc<Self> {
        let _self = Arc::new(Self {
            world,
            selected_kind: Mutable::new(None),
            destroyed_kinds: Mutable::new(HashSet::new()),
            phase: Mutable::new(None),
        });

        _self
    }

    pub fn render(self: Arc<Self>) -> Dom {
        static CONTAINER :LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("position", "absolute")
                .style("width", "100vw")
                .style("height", "100vh")
                .style("overflow", "hidden")
            }
        });
        let state = self;
        html!("div", {
            .class(&*CONTAINER)
            .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
                match phase {
                    None => Some(state.render_footer()),
                    Some(phase) => {
                        match phase {
                            GameUiPhase::Welcome => {
                                Some(Welcome::new(state.world.clone(), state.phase.clone()).render())
                            },
                            GameUiPhase::Help=> {
                                Some(Help::new(state.world.clone(), state.phase.clone()).render())
                            },
                            GameUiPhase::GameOver => {
                                Some(GameOver::new().render())
                            },
                            GameUiPhase::Winner => {
                                Some(Winner::new().render())
                            },
                        }
                    }
                }
            })))
        })
    }

    pub fn select_enemy(&self, kind: EnemyKind) {
        // just dispatch the event, the system will handle it
        // and ultimately set the local mutable
        // perhaps we could make it a _little_ more responsive by setting it here too
        // but better for sanity checking to just let it flow from the system
        self.world.run(|mut events: ViewMut<EnemySelectEvent>, enemies: View<Enemy>| {
            dispatch_select_event!(enemies, &mut events, kind);
        });
    }
}