use awsm_web::webgl::{WebGl2Renderer, VertexArray, NameOrLoc, AttributeOptions, DataType, TextureTarget, SimpleTextureOptions, PixelFormat, WebGlTextureSource, TextureWrapMode};
use web_sys::HtmlImageElement;

use crate::{prelude::*, renderer::{Renderer, shaders::ShaderProgram}, media::{Media, SpriteSheetMediaInfo}, spritesheet::{SpriteSheet, self}, config::CONFIG};

use super::{controller::data::{EnemyControllerOne, EnemyControllerTwo, EnemyControllerThree, EnemyControllerFour, EnemyController}, physics::data::EnemyDirection};

#[derive(Component)]
pub enum Enemy {
    One {
        phase: EnemyOnePhase,
        spritesheet: EnemySpriteSheetsOne,
        controller: EnemyControllerOne,
    },
    Two {
        phase: EnemyTwoPhase,
        spritesheet: EnemySpriteSheetsTwo,
        controller: EnemyControllerTwo,
    },
    Three {
        phase: EnemyThreePhase,
        spritesheet: EnemySpriteSheetsThree,
        controller: EnemyControllerThree,
    },
    Four {
        phase: EnemyFourPhase,
        spritesheet: EnemySpriteSheetsFour,
        controller: EnemyControllerFour,
    }
}

impl Enemy {
    pub fn kind(&self) -> EnemyKind {
        match self {
            Enemy::One { .. } => EnemyKind::One,
            Enemy::Two { .. } => EnemyKind::Two,
            Enemy::Three { .. } => EnemyKind::Three,
            Enemy::Four { .. } => EnemyKind::Four,
        }
    }

    pub fn name(&self) -> &'static str {
        match &self {
            Self::One { .. } => "one",
            Self::Two { .. } => "two",
            Self::Three { .. } => "three",
            Self::Four { .. } => "four",
        }
    }

    pub fn spritesheet(&self) -> &SpriteSheet {
        match self {
            Self::One { spritesheet, phase, .. } => {
                match phase {
                    EnemyOnePhase::Idle => &spritesheet.idle,
                    EnemyOnePhase::Walk => &spritesheet.walk,
                    EnemyOnePhase::Blast => &spritesheet.blast,
                    EnemyOnePhase::Hurt => &spritesheet.hurt,
                }
            }
            Self::Two { spritesheet, phase, .. } => {
                match phase {
                    EnemyTwoPhase::Idle => &spritesheet.idle,
                    EnemyTwoPhase::Walk => &spritesheet.walk,
                    EnemyTwoPhase::Hurt => &spritesheet.hurt,
                    EnemyTwoPhase::Shooting => {
                        &spritesheet.shooting
                    },
                }
            }
            Self::Three { spritesheet, phase, .. } => {
                match phase {
                    EnemyThreePhase::Idle => &spritesheet.idle,
                    EnemyThreePhase::Walk => &spritesheet.walk,
                    EnemyThreePhase::Hurt => &spritesheet.hurt,
                    EnemyThreePhase::Shoot => &spritesheet.shoot,
                }
            }
            Self::Four { spritesheet, phase, .. } => {
                match phase {
                    EnemyFourPhase::Idle => &spritesheet.idle,
                    EnemyFourPhase::Hurt => &spritesheet.hurt,
                    EnemyFourPhase::Shoot => &spritesheet.shoot,
                }
            }
        }
    }

    // returns the common controller trait for all enemies
    // more specific controllers need to be matched explicitly
    pub fn controller(&self) -> &dyn EnemyController {
        match self {
            Enemy::One { controller, .. } => controller,
            Enemy::Two { controller, .. } => controller, 
            Enemy::Three { controller, .. } => controller,
            Enemy::Four { controller, .. } => controller 
        }
    }
    // returns the common controller trait for all enemies
    // more specific controllers need to be matched explicitly
    pub fn controller_mut(&mut self) -> &mut dyn EnemyController {
        match self {
            Enemy::One { controller, .. } => controller,
            Enemy::Two { controller, .. } => controller, 
            Enemy::Three { controller, .. } => controller,
            Enemy::Four { controller, .. } => controller 
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum EnemyKind {
    One,
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyOnePhase{
    Idle,
    Walk,
    Blast,
    Hurt
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyTwoPhase{
    Idle,
    Walk,
    Hurt,
    Shooting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyThreePhase{
    Idle,
    Walk,
    Hurt,
    Shoot
}

impl EnemyThreePhase {
    pub fn next(self) -> Self {
        match self {
            Self::Idle => Self::Idle,
            Self::Walk => Self::Walk,
            Self::Shoot => Self::Idle,
            Self::Hurt => Self::Hurt,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyFourPhase{
    Idle,
    Hurt,
    Shoot
}

impl EnemyFourPhase {
    pub fn next(self) -> Self {
        match self {
            Self::Idle => Self::Idle,
            Self::Shoot => Self::Idle,
            Self::Hurt => Self::Hurt,
        }
    }
}


#[derive(Debug, Default)]
pub struct EnemySpriteSheets {
    pub one: Option<EnemySpriteSheetsOne>,
    pub two: Option<EnemySpriteSheetsTwo>,
    pub three: Option<EnemySpriteSheetsThree>,
    pub four: Option<EnemySpriteSheetsFour>,
}

impl EnemySpriteSheets {
    pub fn new(renderer: &mut Renderer, media: &Media) -> Result<Self> {
        let one = match &media.terrorists.one {
            None => None,
            Some(t) => {
                Some(EnemySpriteSheetsOne {
                    blast: SpriteSheet::new(renderer, &t.blast_img, &t.blast_info)?,
                    hurt: SpriteSheet::new(renderer, &t.hurt_img, &t.hurt_info)?,
                    idle: SpriteSheet::new(renderer, &t.idle_img, &t.idle_info)?,
                    walk: SpriteSheet::new(renderer, &t.walk_img, &t.walk_info)?,
                })
            }
        };

        let two = match &media.terrorists.two {
            None => None,
            Some(t) => {
                Some(EnemySpriteSheetsTwo{
                    hurt: SpriteSheet::new(renderer, &t.hurt_img, &t.hurt_info)?,
                    idle: SpriteSheet::new(renderer, &t.idle_img, &t.idle_info)?,
                    shooting: SpriteSheet::new(renderer, &t.shooting_img, &t.shooting_info)?,
                    walk: SpriteSheet::new(renderer, &t.walk_img, &t.walk_info)?,
                })
            }
        };

        let three = match &media.terrorists.three {
            None => None,
            Some(t) => {
                Some(EnemySpriteSheetsThree {
                    hurt: SpriteSheet::new(renderer, &t.hurt_img, &t.hurt_info)?,
                    idle: SpriteSheet::new(renderer, &t.idle_img, &t.idle_info)?,
                    shoot: SpriteSheet::new(renderer, &t.shoot_img, &t.shoot_info)?,
                    walk: SpriteSheet::new(renderer, &t.walk_img, &t.walk_info)?,
                })
            }
        };

        let four = match &media.terrorists.four {
            None => None,
            Some(t) => {
                Some(EnemySpriteSheetsFour {
                    hurt: SpriteSheet::new(renderer, &t.hurt_img, &t.hurt_info)?,
                    idle: SpriteSheet::new(renderer, &t.idle_img, &t.idle_info)?,
                    shoot: SpriteSheet::new(renderer, &t.shoot_img, &t.shoot_info)?,
                })
            }
        };

        Ok(Self {
            one,
            two,
            three,
            four
        })
    }
}

#[derive(Clone, Debug)]
pub struct EnemySpriteSheetsOne {
    pub blast: SpriteSheet,
    pub hurt: SpriteSheet,
    pub idle: SpriteSheet,
    pub walk: SpriteSheet,
}

#[derive(Clone, Debug)]
pub struct EnemySpriteSheetsTwo {
    pub hurt: SpriteSheet,
    pub idle: SpriteSheet,
    pub shooting: SpriteSheet,
    pub walk: SpriteSheet,
}

#[derive(Clone, Debug)]
pub struct EnemySpriteSheetsThree {
    pub hurt: SpriteSheet,
    pub idle: SpriteSheet,
    pub shoot: SpriteSheet,
    pub walk: SpriteSheet,
}

#[derive(Clone, Debug)]
pub struct EnemySpriteSheetsFour {
    pub hurt: SpriteSheet,
    pub idle: SpriteSheet,
    pub shoot: SpriteSheet,
}