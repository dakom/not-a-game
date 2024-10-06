use nalgebra_glm::Vec2;

use crate::{
    animation::data::Animation,
    collision::data::Collider,
    config::CONFIG,
    dom::{ui::UiPhase, DomView},
    enemy::{
        controller::data::{
            EnemyControllerFour, EnemyControllerOne, EnemyControllerThree, EnemyControllerTwo,
        },
        data::{Enemy, EnemyFourPhase, EnemyKind, EnemyOnePhase, EnemyThreePhase, EnemyTwoPhase},
        effects::data::EnemyEffect,
        events::EnemySelectEvent,
        launcher::data::{EnemyLauncher, LauncherSide},
        physics::data::EnemyDirection,
    },
    layout::data::{LayoutAnchor, LayoutPosition},
    media::MediaView,
    prelude::*,
    renderer::RendererViewMut,
    tick::{BeginTickView, UpdateTickView},
};

use super::data::{EnemySpawner, EnemySpawnerViewMut};

pub fn spawn_enemies(world: &World) {
    world.run(
        |mut spawner: EnemySpawnerViewMut,
         mut sg_storages: SceneGraphStoragesMut,
         // mut entities: EntitiesViewMut,
         mut enemies: ViewMut<Enemy>,
         mut animations: ViewMut<Animation>,
         // to allow more args
         mut enemy_group: (
            ViewMut<LayoutPosition>,
            ViewMut<LayoutAnchor>,
            ViewMut<EnemySelectEvent>,
            ViewMut<EnemyEffect>,
        ),
         mut colliders: ViewMut<Collider>,
         mut renderer: RendererViewMut,
         media: MediaView| {
            let (mut enemy_positions, mut enemy_anchors, mut enemy_select, mut enemy_effects) =
                enemy_group;

            for spawn_index in 0..4 {
                let entity = sg_storages.spawn_child_trs(None, None, None, None);

                let enemy = match spawn_index {
                    0 => Enemy::One {
                        phase: EnemyOnePhase::Idle,
                        spritesheet: spawner.sprite_sheets.one.clone().unwrap(),
                        controller: EnemyControllerOne::new(EnemyDirection::Right),
                    },
                    1 => Enemy::Two {
                        phase: EnemyTwoPhase::Idle,
                        spritesheet: spawner.sprite_sheets.two.clone().unwrap(),
                        controller: EnemyControllerTwo::new(EnemyDirection::Left),
                    },
                    2 => Enemy::Three {
                        phase: EnemyThreePhase::Idle,
                        spritesheet: spawner.sprite_sheets.three.clone().unwrap(),
                        controller: EnemyControllerThree::new(EnemyDirection::Right),
                    },
                    3 => Enemy::Four {
                        phase: EnemyFourPhase::Idle,
                        spritesheet: spawner.sprite_sheets.four.clone().unwrap(),
                        controller: EnemyControllerFour::new(EnemyDirection::Left),
                    },
                    _ => unreachable!(),
                };

                let enemy_kind = enemy.kind();
                /*
                let position = match enemy_kind {
                    EnemyKind::One => {
                        Vec3::new(0.35, -1.0, 0.0)
                    },
                    EnemyKind::Two => {
                        Vec3::new(0.5, -1.0, 0.0)
                    },
                    EnemyKind::Three => {
                        Vec3::new(0.65, 0.0, 0.0)
                    },
                    EnemyKind::Four => {
                        // this will be repositioned later based on LauncherSide
                        Vec3::new(0.0, 0.0, 0.0)
                    },
                }; */
                let position = match enemy_kind {
                    EnemyKind::One => Vec3::new(0.35, 0.0, 0.0),
                    EnemyKind::Two => Vec3::new(0.5, 0.0, 0.0),
                    EnemyKind::Three => Vec3::new(0.65, 0.0, 0.0),
                    EnemyKind::Four => {
                        // this will be repositioned later based on LauncherSide
                        Vec3::new(0.0, 0.0, 0.0)
                    }
                };

                (
                    &mut animations,
                    &mut enemies,
                    &mut enemy_positions,
                    &mut enemy_anchors,
                    &mut colliders,
                    &mut enemy_effects,
                )
                    .add_component_unchecked(
                        entity,
                        (
                            Animation::new(enemy.spritesheet()),
                            enemy,
                            LayoutPosition::new(position),
                            LayoutAnchor::new(Vec3::zeros()),
                            Collider::default(),
                            EnemyEffect::new(),
                        ),
                    );

                if enemy_kind == CONFIG.selected_enemy.unwrap_or(EnemyKind::One) {
                    enemy_select.add_component_unchecked(entity, EnemySelectEvent {});
                }
            }
        },
    )
}

pub fn spawn_launcher(world: &World, side: LauncherSide) {
    world.run(
        |mut spawner: EnemySpawnerViewMut,
         mut sg_storages: SceneGraphStoragesMut,
         // mut entities: EntitiesViewMut,
         mut launchers: ViewMut<EnemyLauncher>,
         mut animations: ViewMut<Animation>,
         mut renderer: RendererViewMut,
         mut positions: ViewMut<LayoutPosition>,
         media: MediaView| {
            let scale = match side {
                LauncherSide::Left => Vec3::new(1.0, 1.0, 1.0),
                LauncherSide::Right => Vec3::new(-1.0, 1.0, 1.0),
            };

            let entity = sg_storages.spawn_child_trs(None, None, None, Some(scale));

            (&mut animations, &mut launchers, &mut positions).add_component_unchecked(
                entity,
                (
                    Animation::new(&spawner.launcher_sprite_sheet),
                    EnemyLauncher::new(side, spawner.launcher_sprite_sheet.clone()),
                    LayoutPosition::new(match side {
                        LauncherSide::Left => Vec3::zeros(),
                        LauncherSide::Right => Vec3::new(1.0, 0.0, 0.0),
                    }),
                ),
            );
        },
    )
}
