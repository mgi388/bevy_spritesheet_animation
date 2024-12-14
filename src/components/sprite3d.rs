use bevy::{
    asset::Handle,
    color::Color,
    ecs::prelude::*,
    math::Vec2,
    prelude::*,
    render::view::Visibility,
    sprite::{Anchor, TextureAtlas},
    transform::components::Transform,
};

/// Specifies the rendering properties of a 3D sprite.
///
/// This contains similar fields as Bevy's [Sprite](bevy::sprite::Sprite).
///
/// # Note
///
/// The geometry and material required for rendering a 3D sprite will be automatically added by the library in an internal system.
///
/// The library requires the sprite's texture to be loaded before setting everything up.
/// If the texture has already been loaded (for example, in a loading stage), the sprite will appear on the next update.
/// Otherwise, the actual rendering will be delayed and the sprite will not be visible during a few frames.
#[derive(Component, Default, Debug, Reflect)]
#[require(Transform, Visibility)]
#[reflect(Component, Debug)]
pub struct Sprite3d {
    /// The image used to render the sprite
    pub image: Handle<Image>,

    /// The (optional) texture atlas used to render the sprite
    pub texture_atlas: Option<TextureAtlas>,

    /// A color to tint the sprite with.
    ///
    /// The default color is white, which does not tint the sprite.
    pub color: Color,

    /// Flips the sprite horizontally.
    pub flip_x: bool,

    /// Flips the sprite vertically.
    pub flip_y: bool,

    /// The size of the sprite.
    ///
    /// If undefined, the dimensions of the sprite's image will be used.
    pub custom_size: Option<Vec2>,

    /// The position of the sprite's origin
    pub anchor: Anchor,
}

impl Sprite3d {
    pub fn from_image(image: Handle<Image>) -> Self {
        Self {
            image,
            ..Default::default()
        }
    }

    pub fn from_atlas_image(image: Handle<Image>, atlas: TextureAtlas) -> Self {
        Self {
            image,
            texture_atlas: Some(atlas),
            ..Default::default()
        }
    }

    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }

    pub fn with_flip(mut self, x: bool, y: bool) -> Self {
        self.flip_x = x;
        self.flip_y = y;
        self
    }

    pub fn with_custom_size(mut self, size: impl Into<Vec2>) -> Self {
        self.custom_size = Some(size.into());
        self
    }

    pub fn with_anchor(mut self, anchor: impl Into<Anchor>) -> Self {
        self.anchor = anchor.into();
        self
    }
}
