#![allow(dead_code)]
#![allow(warnings)]

pub mod prelude;
pub mod route;
pub mod config;
pub mod utils;
pub mod dom;
pub mod media;
pub mod renderer;
pub mod controller;
pub mod background;
pub mod camera;
pub mod tick;
pub mod enemy;
pub mod logging;
pub mod rand_helpers;
pub mod spritesheet;
pub mod projectiles;
pub mod delete;
pub mod layout;
pub mod bomber;
pub mod explosion;
pub mod collision;
pub mod animation;
pub mod game_over;
pub mod audio;


use std::{borrow::BorrowMut, sync::atomic::AtomicU64};

use audio::{audio_event_process_sys, AudioEventQueue, AudioPlayer};
use background::{systems::background_move_sys, data::{Background, BackgroundViewMut}};
use bomber::{data::Bomber, systems::bomber_drop_sys};
use camera::{Camera, systems::camera_update_ubo_sys, CameraViewMut};
use collision::{debug::CollisionDebugger, systems::{update_collider_sys, detect_geometric_collision_sys, pixel_collision_check_sys, pixel_collision_render_sys}, data::CollisionEventQueue};
use config::CONFIG;
use delete::systems::delete_sys;
use enemy::{animation::systems::enemy_animation_sys, controller::systems::{enemy_controller_physics_sys}, data::{Enemy}, destroy::enemy_destroy_event_sys, launcher::{data::LauncherSide, systems::launcher_animation_sys}, physics::systems::{enemy_position_sys}, select::enemy_select_event_sys, spawner::{EnemySpawner, actions::{spawn_enemies, spawn_launcher}}};
use explosion::{data::ExplosionSpawner, systems::explosion_spawn_sys, animation::explosion_animation_sys};
use game_over::systems::game_over_sys;
use gloo_timers::future::TimeoutFuture;
use layout::systems::flush_layout_sys;
use prelude::*;
use awsm_web::{webgl::ResizeStrategy, tick::{MainLoop, MainLoopOptions, Raf}};
use controller::{listeners::InputListeners, queue::InputQueue, systems::controller_process_queue_sys};
use gloo_events::EventListener;
use media::Media;
use projectiles::{data::ProjectileSpawner, systems::{projectile_spawn_sys, projectile_physics_sys}};
use renderer::{Renderer, RendererViewMut, systems::render_sys, shaders::Shaders, framebuffers::FrameBuffers};
use tick::{UpdateTick, BeginTick, UpdateTickViewMut, BeginTickViewMut, DrawTickViewMut, DrawTick, EndTickViewMut, EndTick, PauseTickView, PauseTick};
use wasm_bindgen_futures::spawn_local;
use dom::{theme, ui::{game::{GameUi, GameUiPhase}, UiPhase}, DomState, DomView};
use shipyard_scenegraph::init::init_scenegraph;
use web_sys::console::clear;

// async lib w/ wasm_bindgen(start) is waiting on https://github.com/thedodd/trunk/issues/575
pub fn main() {
    spawn_local(async {
        init().await;
        start().await;
    });
}

async fn init() {
    init_logger();
    theme::stylesheet::init();
}

async fn start() {
    let world = Arc::new(World::new());
    {
        let dom = DomState::new().await;
        let audio_player = AudioPlayer::new();
        let media = Media::load(&dom, &audio_player).await.unwrap_ext();
        let mut renderer = Renderer::new(&dom).unwrap_ext();

        dom.ui.phase.set(UiPhase::Initializing);
        let background = Background::new(&mut renderer, &media).unwrap_ext();
        let camera = Camera::new(&mut renderer).unwrap_ext();
        let enemy_spawner = EnemySpawner::new(&mut renderer, &media).unwrap_ext();
        let projectile_spawner = ProjectileSpawner::new(&mut renderer, &media).unwrap_ext();
        let explosion_spawner = ExplosionSpawner::new(&mut renderer, &media).unwrap_ext();
        let collision_debugger = CollisionDebugger::new(&mut renderer).unwrap_ext();

        world.add_unique_non_send_sync(media);
        world.add_unique_non_send_sync(renderer);
        world.add_unique_non_send_sync(dom);
        world.add_unique_non_send_sync(audio_player);
        world.add_unique_non_send_sync(Rand::new());
        world.add_unique(AudioEventQueue::new());
        world.add_unique(background);
        world.add_unique(enemy_spawner);
        world.add_unique(projectile_spawner);
        world.add_unique(explosion_spawner);
        world.add_unique_non_send_sync(CollisionEventQueue::new());
        world.add_unique(collision_debugger);
        world.add_unique(Bomber::new());
        world.add_unique(InputQueue::new());
        world.add_unique(camera);
        world.add_unique(BeginTick::default());
        world.add_unique(UpdateTick::default());
        world.add_unique(DrawTick::default());
        world.add_unique(EndTick::default());
        world.add_unique(PauseTick::Running);

    }

    Workload::new("controller")
        .with_system(controller_process_queue_sys)
        .with_system(audio_event_process_sys)
        .add_to_world(&world)
        .unwrap_ext();

    Workload::new("begin")
        .with_system(background_move_sys)
        .with_system(enemy_select_event_sys)
        .with_system(enemy_animation_sys)
        .with_system(launcher_animation_sys)
        .with_system(explosion_animation_sys)
        .add_to_world(&world)
        .unwrap_ext();

    Workload::new("update")
        .with_system(game_over_sys)
        .with_system(enemy_destroy_event_sys)
        .with_system(enemy_controller_physics_sys)
        .with_system(enemy_position_sys)
        .with_system(bomber_drop_sys)
        .with_system(explosion_spawn_sys)
        .with_system(projectile_spawn_sys)
        .with_system(projectile_physics_sys)
        .with_system(flush_layout_sys)
        .with_system(local_transform_sys)
        .with_system(world_transform_sys)
        .with_system(update_collider_sys)
        .with_system(detect_geometric_collision_sys)
        .with_system(pixel_collision_check_sys)
        .with_system(delete_sys)
        .add_to_world(&world)
        .unwrap_ext();

    Workload::new("draw")
        .with_system(camera_update_ubo_sys)
        .with_system(pixel_collision_render_sys)
        .with_system(render_sys)
        .add_to_world(&world)
        .unwrap_ext();

    Workload::new("end")
        .add_to_world(&world)
        .unwrap_ext();

    init_scenegraph::<Vec3, Quat, Mat4, f32>(&world);


    let on_resize = {
        let world = Arc::clone(&world);
        move |_: &web_sys::Event| {
            // This is a very heavy operation - at least in theory, for example we may want to recreate framebuffers
            // So only run it when the event is really triggered (i.e. startup, user resizes browser, etc.)
            world.run(|dom: DomView, mut renderer: RendererViewMut, mut camera: CameraViewMut, mut background: BackgroundViewMut| {
                let (width, height) = dom.window_size();
                renderer.resize(ResizeStrategy::All(width, height));
                renderer.resize_framebuffers();
                camera.resize(width as f64, height as f64);
                background.resize(width as f64, height as f64);


            });
        }
    };

    on_resize(&web_sys::Event::new("").unwrap_ext());


    let mut main_loop = MainLoop::new(
        MainLoopOptions::default(),
        {
            let world = Arc::clone(&world);
            move |time, delta| {

                world.run_workload("controller").unwrap_ext();
                if *world.borrow::<PauseTickView>().unwrap_ext() == PauseTick::Running {
                    *world.borrow::<BeginTickViewMut>().unwrap_ext() = BeginTick{time, delta};
                    world.run_workload("begin").unwrap_ext();
                }
            }
        },
        {
            let world = Arc::clone(&world);


            move |delta| {

                if *world.borrow::<PauseTickView>().unwrap_ext() == PauseTick::Running {
                    let viewport = world.borrow::<RendererViewMut>().map(|renderer| renderer.get_viewport());
                    if let Ok((_, _, viewport_width, viewport_height)) = viewport {
                        *world.borrow::<UpdateTickViewMut>().unwrap_ext() = UpdateTick {
                            delta,
                            viewport_width: viewport_width as f64,
                            viewport_height: viewport_height as f64,
                        };
                        world.run_workload("update").unwrap_ext();
                    }
                }
            }
        },
        {
            let world = Arc::clone(&world);
            move |interpolation| {
                if *world.borrow::<PauseTickView>().unwrap_ext() == PauseTick::Running {
                    let viewport = world.borrow::<RendererViewMut>().map(|renderer| renderer.get_viewport());
                    if let Ok((_, _, viewport_width, viewport_height)) = viewport {
                        *world.borrow::<DrawTickViewMut>().unwrap_ext() = DrawTick {
                            interpolation,
                            viewport_width: viewport_width as f64,
                            viewport_height: viewport_height as f64,
                        };
                        world.run_workload("draw").unwrap_ext();
                    }
                }
            }
        },
        {
            let world = Arc::clone(&world);
            move |fps, abort| {
                if *world.borrow::<PauseTickView>().unwrap_ext() == PauseTick::Running {
                    *world.borrow::<EndTickViewMut>().unwrap_ext() = EndTick {fps, abort};
                    world.run_workload("end").unwrap_ext();
                }
            }
        },
    );

    // accumulted amount of time lost due to visibility pause which fully stops the ticker. 
    // Manual pause doesn't affect the ticker (and in fact needs it to do things like handle input to unpause)
    let pause_ms_acc = Arc::new(AtomicU64::new(0));

    let tick = Raf::new({
        clone!(pause_ms_acc => move |mut ts| {
            ts = ts - pause_ms_acc.load(std::sync::atomic::Ordering::SeqCst) as f64;
            main_loop.tick(ts);
        })
    });

    let on_visibility = {
        let world = Arc::clone(&world);
        move |_: &web_sys::Event| {
            world.run(|dom: DomView, mut pause_tick: UniqueViewMut<PauseTick>| {
                let pause_tick = &mut *pause_tick;
                match dom.document.visibility_state() {
                    web_sys::VisibilityState::Hidden => {
                        *pause_tick = PauseTick::LostVisibility{
                            timestamp: dom.window.performance().unwrap_ext().now(),
                            previous: Box::new(pause_tick.clone())
                        }
                    },
                    web_sys::VisibilityState::Visible => {
                        if let PauseTick::LostVisibility{timestamp, previous} = pause_tick {
                            let timestamp = *timestamp;
                            let previous = previous.clone();
                            *pause_tick = *previous; 
                            let diff = dom.window.performance().unwrap_ext().now() - timestamp;
                            pause_ms_acc.fetch_add(diff.round() as u64, std::sync::atomic::Ordering::SeqCst);
                        }
                    },
                    _ => {
                        log::warn!("Unknown visibility state");
                    },
                }
            });
        }
    };


    // these just run forever
    std::mem::forget(Box::new(tick));
    std::mem::forget(Box::new(InputListeners::new(Arc::clone(&world))));
    EventListener::new(&world.borrow::<DomView>().unwrap_ext().window, "resize", on_resize).forget();
    EventListener::new(&world.borrow::<DomView>().unwrap_ext().window, "visibilitychange", on_visibility).forget();

    // Update the UI - and we're off!
    world.borrow::<DomView>().unwrap_ext().ui.phase.set(UiPhase::Playing(GameUi::new(Arc::clone(&world))));
    spawn_enemies(&world);
    spawn_launcher(&world, LauncherSide::Left);
    spawn_launcher(&world, LauncherSide::Right);

    if let Some(phase) = CONFIG.initial_game_phase {
        world.borrow::<DomView>().unwrap_ext().ui.game_ui_unchecked().phase.set_neq(Some(phase));
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook"))] {
        fn init_logger() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn init_logger() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}

