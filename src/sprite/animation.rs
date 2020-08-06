use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::resources::{Resources};

#[derive(Debug, Serialize, Deserialize)]
pub struct Animation {
    name: String,
    frames: Vec<(f32, f32)>,
    #[serde(default)]
    framerate: f32,
    #[serde(default)]
    current_frame: usize,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            frames: vec![(0.0, 0.0)],
            framerate: 0.1,
            current_frame: 0,
        }
    }
}

impl Animation {
    pub fn new(name: String, frames: Vec<(f32, f32)>) -> Animation {
        Animation {
            name,
            frames,
            ..Default::default()
        }
    }

    pub fn from_json(res: &Resources, path: String) -> Result<Animation, failure::Error> {
        let json = res.load_from_json(&path)?;
        let animation: Animation = serde_json::from_value(json)?;

        Ok(animation)
    }

    pub fn set_current_frame(&mut self, frame: usize) {
        let frame_bounds = self.frames.len() - 1;

        if frame <= frame_bounds {
            self.current_frame = frame;
        } else {
            self.current_frame = 0;
        }
    }

    pub fn set_framerate(&mut self, framerate: f32) {
        self.framerate = framerate;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Animations {
    animations: HashMap<String, Animation>,
    is_playing: bool,
    current: Option<String>,
    accumulated: f32,
}

impl Animations {
    pub fn new(animations: Vec<Animation>) -> Animations {
        let mut animation_map = HashMap::new();

        for animation in animations {
            animation_map.insert(
                animation.name.to_string(),
                animation
            );
        }

        Animations {
            animations: animation_map,
            is_playing: false,
            current: None,
            accumulated: 0.0,
        }
    }

    pub fn from_json(res: &Resources, path: String) -> Result<Animations, failure::Error> {
        let json = res.load_from_json(&path)?;
        let animations: HashMap<String, Animation> = serde_json::from_value(json)?;

        Ok(Animations {
            animations,
            is_playing: false,
            current: None,
            accumulated: 0.0,
        })
    }

    pub fn add(&mut self, animation: Animation) {
        let name = animation.name.to_string();
        self.animations.insert(name, animation);
    }

    pub fn remove(&mut self, key: &str) {
        self.animations.remove(key);
    }

    pub fn clear(&mut self) {
        self.animations.clear();
    }

    pub fn set_current(&mut self, key: &str) {
        self.current = Some(key.to_string());
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn play(&mut self, key: &str) {
        self.is_playing = true;
        self.current = Some(key.to_string());
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn resume(&mut self) {
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.accumulated = 0.0;

        match &self.current {
            Some(current) => {
                let animation = self.animations.get_mut(current).unwrap();

                animation.set_current_frame(0);
            },
            None => {},
        }
    }

    pub fn get_frame(&self) -> (f32, f32) {
        match &self.current {
            Some(current) => {
                let animation = self.animations.get(current).unwrap();
                let i = animation.current_frame;

                animation.frames[i]
            },
            None => (0.0, 0.0),
        }
    }

    pub fn set_frame(&mut self, frame: usize) {
        match &self.current {
            Some(current) => {
                let animation = self.animations.get_mut(current).unwrap();
                animation.set_current_frame(frame);
            },
            None => {},
        }
    }

    pub fn set_framerate(&mut self, framerate: f32) {
        match &self.current {
            Some(current) => {
                let animation = self.animations.get_mut(current).unwrap();
                animation.set_framerate(framerate);
            },
            None => {},
        }
    }

    pub fn update_frame(&mut self) {
        if self.is_playing {
            match &self.current {
                Some(current) => {
                    let animation = self.animations.get_mut(current).unwrap();
                    animation.set_current_frame(animation.current_frame + 1);
                },
                None => {},
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        let is_playing = self.is_playing;
        let current = self.current.as_ref();

        if is_playing {
            let animation = match current {
                Some(curr) => self.animations.get_mut(curr).unwrap(),
                None => { return },
            };

            self.accumulated += dt;
            while self.accumulated > animation.framerate {
                self.accumulated -= animation.framerate;
                animation.set_current_frame(animation.current_frame + 1);
            }
        }
    }
}

