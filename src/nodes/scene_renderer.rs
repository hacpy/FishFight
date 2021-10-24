use macroquad::{
    experimental::{
        collections::storage,
        scene::{self, RefMut},
    },
    prelude::*,
};

use crate::{GameWorld, Resources};

pub struct SceneRenderer;

impl SceneRenderer {
    pub fn new() -> SceneRenderer {
        SceneRenderer {}
    }
}

fn parallax(texture: Texture2D, depth: f32, camera_pos: Vec2) -> Rect {
    let w = texture.width();
    let h = texture.height();

    let dest_rect = Rect::new(0., 0., w, h);
    let parallax_w = w as f32 * 0.5;

    let mut dest_rect2 = Rect::new(
        -parallax_w,
        -parallax_w,
        w + parallax_w * 2.,
        h + parallax_w * 2.,
    );

    let parallax_x = camera_pos.x / dest_rect.w - 0.3;
    let parallax_y = camera_pos.y / dest_rect.h * 0.6 - 0.5;

    dest_rect2.x += parallax_w * parallax_x * depth;
    dest_rect2.y += parallax_w * parallax_y * depth;

    dest_rect2
}

impl scene::Node for SceneRenderer {
    fn draw(_: RefMut<Self>) {
        {
            let resources = storage::get::<Resources>();
            let pos = scene::camera_pos();

            clear_background(Color::from_rgba(126, 168, 166, 255));

            {
                let texture_entry = resources.textures.get("background_03").unwrap();
                let dest_rect = parallax(texture_entry.texture, 2.0, pos);
                draw_texture_ex(
                    texture_entry.texture,
                    dest_rect.x,
                    80.0 + dest_rect.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(dest_rect.w, dest_rect.h)),
                        ..Default::default()
                    },
                );
            }

            {
                let texture_entry = resources.textures.get("background_02").unwrap();
                let dest_rect = parallax(texture_entry.texture, 1.0, pos);
                draw_texture_ex(
                    texture_entry.texture,
                    dest_rect.x,
                    120.0 + dest_rect.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(dest_rect.w, dest_rect.h)),
                        ..Default::default()
                    },
                );
            }

            {
                let texture_entry = resources.textures.get("background_01").unwrap();
                let dest_rect = parallax(texture_entry.texture, 0.5, pos);
                draw_texture_ex(
                    texture_entry.texture,
                    dest_rect.x,
                    180.0 + dest_rect.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(dest_rect.w, dest_rect.h)),
                        ..Default::default()
                    },
                );
            }
        }

        let world = storage::get::<GameWorld>();
        world.map.draw(false, None);
    }
}
