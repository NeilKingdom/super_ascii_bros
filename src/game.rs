use crate::Window;
use crate::Actor;
use crate::ascii_bros::*;
use crate::actor::sprite::Sprite;
use crate::actor::sprite::tile::{Tile, PixBuf};

use std::borrow::BorrowMut;
use std::ffi::OsString;
use std::path::PathBuf;
use std::collections::HashMap;
use itertools::Itertools;
use std::{fmt,fs};
use std::rc::Rc;

// Handles game logic e.g. timers, physics, etc.
pub struct Game {
    pub tile_atlas: HashMap<Ident, Tile>,
    pub actor_list: Vec<Rc<Actor<f32>>>,
    pub next_tile_id: Ident,
}

impl Game {
    pub fn tile_atlas_contains(&self, pix_buf: &PixBuf) -> bool {
        for tile in self.tile_atlas.values() {
            if tile.pix_buf == *pix_buf { return true; }
        }
        false
    }

    pub fn on_start(&mut self) {

        /*** Load sprites and map to corresponding actors ***/

        let sprite_dir = &(env!("CARGO_MANIFEST_DIR").to_owned() + "/assets/sprites/");
        if let Ok(entries) = fs::read_dir(sprite_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    // TODO: Remove
                    if file_name.eq(&OsString::from("mario.txt")) { continue; }
                    let file_name_str = file_name.to_str().expect("file name into str failed");
                    let full_path = OsString::from(sprite_dir.to_owned() + file_name_str);

                    // TODO: Somehow include entity type information (maybe using dict?)
                    let sprite = Sprite::new(self, PathBuf::from(full_path), 255);
                    let actor = Rc::new(Actor::new(
                        EntityType::Mushroom,
                        5.0,
                        5.0,
                        sprite,
                        Some(Box::new(|actor: &mut Actor<f32>, args: &[f32]| {
                            actor.x_pos += args[0];
                            Ok(())
                        }))));

                    self.actor_list.push(actor);
                }
            }
        } else {
            println!("Error reading directory");
        }
    }

    pub fn on_update(&mut self, win: &mut Window, delta_time: &f32) {
        for actor in &self.actor_list {
            let actor_ref = Rc::clone(&actor);
            actor_ref.borrow_mut().call_on_update(&[0.1]).unwrap();
        }
        win.render_frame(self);
    }
}

// TODO: haven't implemented for sprite/actor list
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = f.debug_struct("Game")
            .field("next_tile_id", &self.next_tile_id)
            .finish();

        for key in self.tile_atlas.keys().sorted() {
            println!();
            if let Some(tile) = self.tile_atlas.get(key) {
                let pix_buf_chars: Vec<char> = tile.pix_buf.iter().map(|&pixel| pixel as char).collect();

                write!(f, "{:<2} [", key)?;
                for (i, c) in pix_buf_chars.iter().enumerate() {
                    write!(f, "{}", c)?;
                    if i < pix_buf_chars.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")?;
            }
        }

        result
    }
}
