use crate::Window;
use crate::ascii_bros::*;
use crate::actor::*;
use crate::actor::sprite::tile::{Tile, PixBuf};
use crate::actor::sprite::Sprite;

use std::ffi::OsString;
use std::path::PathBuf;
use std::collections::HashMap;
use itertools::Itertools;
use std::{fmt,fs};

// Handles game logic e.g. timers, physics, etc.
pub struct Game {
    pub tile_atlas: HashMap<Ident, Tile>,
    pub actor_list: Vec<Actor>,
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

        let sprite_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sprites/");
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
                    let actor = Actor::new(ActorProps::new(5.0, 5.0, sprite), Box::new(MushroomActions));
                    self.actor_list.push(actor);
                }
            }
        } else {
            println!("Error reading directory");
        }
    }

    pub fn on_update(&mut self, win: &mut Window, delta_time: &u128) {
        for actor in &mut self.actor_list {
            actor.actions.update_pos(&mut actor.props, &delta_time);
            // TODO: invoke additional functions that are exclusive to specific Action types
            //match type_of(&actor.actions) {
            //    MushroomActions => {
            //    }
            //    _ => {}
            //}
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
