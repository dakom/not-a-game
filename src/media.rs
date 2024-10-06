use std::collections::HashMap;

use anyhow::Context;
use awsm_web::loaders::{self, audio};
use futures::{stream::FuturesUnordered, StreamExt};
use js_sys::{Array, ArrayBuffer};
use serde::Deserialize;
use web_sys::{AudioBuffer, HtmlImageElement};

use crate::{
    audio::AudioPlayer,
    collision,
    config::CONFIG,
    dom::{ui::UiPhase, DomState},
    prelude::*,
};

pub type MediaViewMut<'a> = NonSendSync<UniqueViewMut<'a, Media>>;
pub type MediaView<'a> = NonSendSync<UniqueView<'a, Media>>;

#[derive(Component, Unique)]
pub struct Media {
    pub bg: Vec<Vec<HtmlImageElement>>,
    pub terrorists: TerroristMedia,
    pub objects: ObjectMedia,
    pub audio: AudioMedia,
}

pub struct AudioMedia {
    pub collision_die: AudioBuffer,
    pub collision_impact: AudioBuffer,
    pub move_duck: AudioBuffer,
    pub move_jump: AudioBuffer,
    pub weapon_bullet: AudioBuffer,
    pub weapon_explode: AudioBuffer,
    pub weapon_launcher: AudioBuffer,
    pub weapon_rpg: AudioBuffer,
}

#[derive(Debug, Default)]
pub struct TerroristMedia {
    pub one: Option<TerroristMediaOne>,
    pub two: Option<TerroristMediaTwo>,
    pub three: Option<TerroristMediaThree>,
    pub four: Option<TerroristMediaFour>,
}

#[derive(Debug)]
pub struct ObjectMedia {
    pub launcher_img: HtmlImageElement,
    pub launcher_info: SpriteSheetMediaInfo,
    pub explosion_img: HtmlImageElement,
    pub explosion_info: SpriteSheetMediaInfo,
    pub rocket_bad_img: HtmlImageElement,
    pub bullet_img: HtmlImageElement,
    pub rocket_good_img: HtmlImageElement,
}

#[derive(Debug)]
pub struct TerroristMediaOne {
    pub blast_img: HtmlImageElement,
    pub hurt_img: HtmlImageElement,
    pub idle_img: HtmlImageElement,
    pub walk_img: HtmlImageElement,
    pub blast_info: SpriteSheetMediaInfo,
    pub hurt_info: SpriteSheetMediaInfo,
    pub idle_info: SpriteSheetMediaInfo,
    pub walk_info: SpriteSheetMediaInfo,
}

#[derive(Debug)]
pub struct TerroristMediaTwo {
    pub hurt_img: HtmlImageElement,
    pub idle_img: HtmlImageElement,
    pub shooting_img: HtmlImageElement,
    pub walk_img: HtmlImageElement,
    pub hurt_info: SpriteSheetMediaInfo,
    pub idle_info: SpriteSheetMediaInfo,
    pub shooting_info: SpriteSheetMediaInfo,
    pub walk_info: SpriteSheetMediaInfo,
}

#[derive(Debug)]
pub struct TerroristMediaThree {
    pub hurt_img: HtmlImageElement,
    pub idle_img: HtmlImageElement,
    pub shoot_img: HtmlImageElement,
    pub walk_img: HtmlImageElement,
    pub hurt_info: SpriteSheetMediaInfo,
    pub idle_info: SpriteSheetMediaInfo,
    pub shoot_info: SpriteSheetMediaInfo,
    pub walk_info: SpriteSheetMediaInfo,
}

#[derive(Debug)]
pub struct TerroristMediaFour {
    pub hurt_img: HtmlImageElement,
    pub idle_img: HtmlImageElement,
    pub shoot_img: HtmlImageElement,
    pub hurt_info: SpriteSheetMediaInfo,
    pub idle_info: SpriteSheetMediaInfo,
    pub shoot_info: SpriteSheetMediaInfo,
}

#[derive(Debug, Deserialize)]
pub struct SpriteSheetMediaInfo {
    pub cell_duration: Option<f64>,
    pub anchor_x: Option<f32>,
    #[serde(rename = "SubTexture")]
    pub sub_textures: Vec<SpriteSheetSubtextureMediaInfo>,
}
#[derive(Debug, Deserialize)]
pub struct SpriteSheetSubtextureMediaInfo {
    pub name: String,
    pub x: String,
    pub y: String,
    pub width: String,
    pub height: String,
}

impl Media {
    pub async fn load(dom: &DomState, audio_player: &AudioPlayer) -> Result<Self> {
        let config = &*CONFIG;

        let mut urls = vec![
            RawMedia::new_audio("collision_die", config.audio_url("collision-die.wav")),
            RawMedia::new_audio("collision_impact", config.audio_url("collision-impact.wav")),
            RawMedia::new_audio("move_duck", config.audio_url("move-duck.wav")),
            RawMedia::new_audio("move_jump", config.audio_url("move-jump.wav")),
            RawMedia::new_audio("weapon_bullet", config.audio_url("weapon-bullet.wav")),
            RawMedia::new_audio("weapon_explode", config.audio_url("weapon-explode.wav")),
            RawMedia::new_audio("weapon_launcher", config.audio_url("weapon-launcher.wav")),
            RawMedia::new_audio("weapon_rpg", config.audio_url("weapon-rpg.wav")),
        ];

        for i in 1..=config.max_bg_panes.unwrap_or(4) {
            for j in 1..=config.max_bg_layers.unwrap_or(7) {
                let name = match j {
                    1 => "background",
                    2 => "hills",
                    3 => "clouds",
                    4 => "ruin",
                    5 => "ground",
                    6 => "houses",
                    7 => "details",
                    _ => unimplemented!("no such bg layer"),
                };
                urls.push(RawMedia::new_image(
                    format!("bg-{i}-{j}-{name}"),
                    config.image_url(&format!("bg/{i}/layers/l{j}_{name}.png")),
                ));
            }
        }

        for i in 1..=4 {
            for j in 1..=4 {
                let name = match (i, j) {
                    (1, 1) => "blast",
                    (1, 2) => "hurt",
                    (1, 3) => "idle",
                    (1, 4) => "walk",
                    (2, 1) => "hurt",
                    (2, 2) => "idle",
                    (2, 3) => "shooting",
                    (2, 4) => "walk",
                    (3, 1) => "hurt",
                    (3, 2) => "idle",
                    (3, 3) => "shoot",
                    (3, 4) => "walk",
                    (4, 1) => "hurt",
                    (4, 2) => "idle",
                    (4, 3) => "shoot",
                    (4, 4) => "",
                    _ => unreachable!("internal loading error for invalid terrorist index"),
                };

                if name != "" {
                    urls.push(RawMedia::new_image(
                        format!("terrorist-image-{i}-{name}"),
                        config.image_url(&format!("terrorists/{i}/{name}.png")),
                    ));
                    urls.push(RawMedia::new_sprite_sheet_info(
                        format!("terrorist-json-{i}-{name}"),
                        config.image_url(&format!("terrorists/{i}/{name}.json")),
                    ));
                }
            }
        }

        urls.push(RawMedia::new_image(
            "object-launcher-img",
            config.image_url(&format!("object/launcher.png")),
        ));
        urls.push(RawMedia::new_sprite_sheet_info(
            "object-launcher-info",
            config.image_url(&format!("object/launcher.json")),
        ));
        urls.push(RawMedia::new_image(
            "object-explosion-img",
            config.image_url(&format!("object/explosion.png")),
        ));
        urls.push(RawMedia::new_sprite_sheet_info(
            "object-explosion-info",
            config.image_url(&format!("object/explosion.json")),
        ));

        urls.push(RawMedia::new_image(
            "object-rocket-bad-img",
            config.image_url(&format!("object/rocket-bad.png")),
        ));
        urls.push(RawMedia::new_image(
            "object-bullet-img",
            config.image_url(&format!("object/bullet.png")),
        ));
        urls.push(RawMedia::new_image(
            "object-rocket-good-img",
            config.image_url(&format!("object/rocket-good.png")),
        ));

        let audio_ctx = audio_player.mixer.clone_audio_ctx();

        let mut futures = urls
            .into_iter()
            .map(|data| {
                let audio_ctx = audio_ctx.clone();
                async move {
                    dom.ui.phase.set(UiPhase::Loading(Some(data.key.clone())));
                    match data.kind {
                        RawMediaKind::ArrayBuffer => {
                            let result = loaders::fetch::fetch_url(&data.url).await?;
                            let buffer = result.array_buffer().await?;
                            anyhow::Ok((data.key, RawMediaResult::ArrayBuffer(buffer)))
                        }
                        RawMediaKind::AudioBuffer => {
                            let result = loaders::fetch::fetch_url(&data.url).await?;
                            let buffer = result.array_buffer().await?;
                            let audio = loaders::audio::audio_buffer(&buffer, &audio_ctx).await?;
                            anyhow::Ok((data.key, RawMediaResult::AudioBuffer(audio)))
                        }
                        RawMediaKind::Image => {
                            let img = loaders::image::load(data.url.clone()).await?;
                            anyhow::Ok((data.key, RawMediaResult::Image(img)))
                        }
                        RawMediaKind::SpriteSheetInfo => {
                            let result = loaders::fetch::fetch_url(&data.url).await?;
                            let info: SpriteSheetMediaInfo = result.json_from_str().await?;
                            anyhow::Ok((data.key, RawMediaResult::SpriteSheetInfo(info)))
                        }
                    }
                }
            })
            .collect::<FuturesUnordered<_>>();

        let mut results = HashMap::new();

        while let Some(res) = futures.next().await {
            match res {
                Ok((name, buffer)) => {
                    results.insert(name, buffer);
                }
                Err(e) => return Err(e.into()),
            }
        }

        let mut bg = Vec::new();

        for i in 1..=config.max_bg_panes.unwrap_or(4) {
            let mut bg_pane = Vec::new();
            for j in 1..=config.max_bg_layers.unwrap_or(7) {
                let name = match j {
                    1 => "background",
                    2 => "hills",
                    3 => "clouds",
                    4 => "ruin",
                    5 => "ground",
                    6 => "houses",
                    7 => "details",
                    _ => unimplemented!("no such bg layer"),
                };
                bg_pane.push(
                    results
                        .remove(&format!("bg-{i}-{j}-{name}"))
                        .unwrap_ext()
                        .unwrap_image(),
                );
            }
            bg.push(bg_pane);
        }

        let mut terrorists = TerroristMedia::default();

        for i in 1..=4 {
            let mut images = Vec::new();
            let mut infos = Vec::new();
            for j in 1..=4 {
                let name = match (i, j) {
                    (1, 1) => "blast",
                    (1, 2) => "hurt",
                    (1, 3) => "idle",
                    (1, 4) => "walk",
                    (2, 1) => "hurt",
                    (2, 2) => "idle",
                    (2, 3) => "shooting",
                    (2, 4) => "walk",
                    (3, 1) => "hurt",
                    (3, 2) => "idle",
                    (3, 3) => "shoot",
                    (3, 4) => "walk",
                    (4, 1) => "hurt",
                    (4, 2) => "idle",
                    (4, 3) => "shoot",
                    (4, 4) => "",
                    _ => unreachable!("internal loading error for invalid terrorist index"),
                };

                if name != "" {
                    let image = results
                        .remove(&format!("terrorist-image-{i}-{name}"))
                        .unwrap_ext()
                        .unwrap_image();
                    let info = results
                        .remove(&format!("terrorist-json-{i}-{name}"))
                        .unwrap_ext()
                        .unwrap_sprite_sheet_info();

                    images.push(image);
                    infos.push(info);
                }
            }

            // pop the images and assign in reverse-order
            match i {
                1 => {
                    terrorists.one = Some(TerroristMediaOne {
                        walk_img: images.pop().unwrap_ext(),
                        walk_info: infos.pop().unwrap_ext(),
                        idle_img: images.pop().unwrap_ext(),
                        idle_info: infos.pop().unwrap_ext(),
                        hurt_img: images.pop().unwrap_ext(),
                        hurt_info: infos.pop().unwrap_ext(),
                        blast_img: images.pop().unwrap_ext(),
                        blast_info: infos.pop().unwrap_ext(),
                    })
                }
                2 => {
                    terrorists.two = Some(TerroristMediaTwo {
                        walk_img: images.pop().unwrap_ext(),
                        walk_info: infos.pop().unwrap_ext(),
                        shooting_img: images.pop().unwrap_ext(),
                        shooting_info: infos.pop().unwrap_ext(),
                        idle_img: images.pop().unwrap_ext(),
                        idle_info: infos.pop().unwrap_ext(),
                        hurt_img: images.pop().unwrap_ext(),
                        hurt_info: infos.pop().unwrap_ext(),
                    })
                }
                3 => {
                    terrorists.three = Some(TerroristMediaThree {
                        walk_img: images.pop().unwrap_ext(),
                        walk_info: infos.pop().unwrap_ext(),
                        shoot_img: images.pop().unwrap_ext(),
                        shoot_info: infos.pop().unwrap_ext(),
                        idle_img: images.pop().unwrap_ext(),
                        idle_info: infos.pop().unwrap_ext(),
                        hurt_img: images.pop().unwrap_ext(),
                        hurt_info: infos.pop().unwrap_ext(),
                    })
                }
                4 => {
                    terrorists.four = Some(TerroristMediaFour {
                        shoot_img: images.pop().unwrap_ext(),
                        shoot_info: infos.pop().unwrap_ext(),
                        idle_img: images.pop().unwrap_ext(),
                        idle_info: infos.pop().unwrap_ext(),
                        hurt_img: images.pop().unwrap_ext(),
                        hurt_info: infos.pop().unwrap_ext(),
                    })
                }
                _ => unimplemented!("could not create terrorist struct from media"),
            }
        }

        let launcher_img = results
            .remove("object-launcher-img")
            .unwrap_ext()
            .unwrap_image();
        let launcher_info = results
            .remove("object-launcher-info")
            .unwrap_ext()
            .unwrap_sprite_sheet_info();
        let explosion_img = results
            .remove("object-explosion-img")
            .unwrap_ext()
            .unwrap_image();
        let explosion_info = results
            .remove("object-explosion-info")
            .unwrap_ext()
            .unwrap_sprite_sheet_info();

        let rocket_bad_img = results
            .remove("object-rocket-bad-img")
            .unwrap_ext()
            .unwrap_image();
        let rocket_good_img = results
            .remove("object-rocket-good-img")
            .unwrap_ext()
            .unwrap_image();
        let bullet_img = results
            .remove("object-bullet-img")
            .unwrap_ext()
            .unwrap_image();

        let objects = ObjectMedia {
            launcher_img,
            launcher_info,
            explosion_img,
            explosion_info,
            rocket_bad_img,
            bullet_img,
            rocket_good_img,
        };

        let audio = AudioMedia {
            collision_die: results
                .remove("collision_die")
                .unwrap_ext()
                .unwrap_audio_buffer(),
            collision_impact: results
                .remove("collision_impact")
                .unwrap_ext()
                .unwrap_audio_buffer(),
            move_duck: results
                .remove("move_duck")
                .unwrap_ext()
                .unwrap_audio_buffer(),
            move_jump: results
                .remove("move_jump")
                .unwrap_ext()
                .unwrap_audio_buffer(),
            weapon_bullet: results
                .remove("weapon_bullet")
                .unwrap_ext()
                .unwrap_audio_buffer(),
            weapon_explode: results
                .remove("weapon_explode")
                .unwrap_ext()
                .unwrap_audio_buffer(),
            weapon_launcher: results
                .remove("weapon_launcher")
                .unwrap_ext()
                .unwrap_audio_buffer(),
            weapon_rpg: results
                .remove("weapon_rpg")
                .unwrap_ext()
                .unwrap_audio_buffer(),
        };

        Ok(Self {
            bg,
            terrorists,
            objects,
            audio,
        })
    }
}

struct RawMedia {
    url: String,
    kind: RawMediaKind,
    key: String,
}

impl RawMedia {
    fn new_buffer(key: impl ToString, url: String) -> Self {
        Self {
            key: key.to_string(),
            kind: RawMediaKind::ArrayBuffer,
            url,
        }
    }

    fn new_audio(key: impl ToString, url: String) -> Self {
        Self {
            key: key.to_string(),
            kind: RawMediaKind::AudioBuffer,
            url,
        }
    }

    fn new_image(key: impl ToString, url: String) -> Self {
        Self {
            key: key.to_string(),
            kind: RawMediaKind::Image,
            url,
        }
    }

    fn new_sprite_sheet_info(key: impl ToString, url: String) -> Self {
        Self {
            key: key.to_string(),
            kind: RawMediaKind::SpriteSheetInfo,
            url,
        }
    }
}

enum RawMediaKind {
    ArrayBuffer,
    AudioBuffer,
    Image,
    SpriteSheetInfo,
}

enum RawMediaResult {
    ArrayBuffer(ArrayBuffer),
    AudioBuffer(AudioBuffer),
    Image(HtmlImageElement),
    SpriteSheetInfo(SpriteSheetMediaInfo),
}

impl RawMediaResult {
    fn unwrap_image(self) -> HtmlImageElement {
        match self {
            RawMediaResult::Image(img) => img,
            _ => unreachable!("expected image"),
        }
    }

    fn unwrap_array_buffer(self) -> ArrayBuffer {
        match self {
            RawMediaResult::ArrayBuffer(buffer) => buffer,
            _ => unreachable!("expected array buffer"),
        }
    }

    fn unwrap_audio_buffer(self) -> AudioBuffer {
        match self {
            RawMediaResult::AudioBuffer(buffer) => buffer,
            _ => unreachable!("expected audio buffer"),
        }
    }

    fn unwrap_sprite_sheet_info(self) -> SpriteSheetMediaInfo {
        match self {
            RawMediaResult::SpriteSheetInfo(info) => info,
            _ => unreachable!("expected sprite sheet info"),
        }
    }
}
