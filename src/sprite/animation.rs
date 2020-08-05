use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::resources::{Resources};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationData {
    name: String,
    frames: Vec<(f32, f32)>,
    framerate: f32,
}

impl Default for AnimationData {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            frames: vec![(0.0, 0.0)],
            framerate: 0.1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Animation {
    data: AnimationData,
    current_frame: usize,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            data: AnimationData { ..Default::default() },
            current_frame: 0,
        }
    }
}

impl Animation {
    pub fn new(name: String, frames: Vec<(f32, f32)>) -> Animation {
        Animation {
            data: AnimationData {
                name,
                frames,
                ..Default::default()
            },
            current_frame: 0,
        }
    }

    pub fn from_json(res: &Resources, path: String) -> Result<Animation, failure::Error> {
        let json = res.load_from_json(&path)?;
        let animation: AnimationData = serde_json::from_value(json)?;

        Ok(Animation {
            data: animation,
            current_frame: 0,
        })
    }

    pub fn set_current_frame(&mut self, frame: usize) {
        let frame_bounds = self.data.frames.len() - 1;

        if frame <= frame_bounds {
            self.current_frame = frame;
        } else {
            self.current_frame = 0;
        }
    }

    pub fn set_framerate(&mut self, framerate: f32) {
        self.data.framerate = framerate;
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
        let mut anims = HashMap::new();

        for Animation { data, current_frame } in animations {
            anims.insert(
                data.name.to_string(),
                Animation {
                    data,
                    current_frame,
                }
            );
        }

        Animations {
            animations: anims,
            is_playing: false,
            current: None,
            accumulated: 0.0,
        }
    }

    pub fn from_json(res: &Resources, path: String) -> Result<Animations, failure::Error> {
        let json = res.load_from_json(&path)?;
        let animations = serde_json::from_value(json)?;

        Ok(Animations {
            animations,
            is_playing: false,
            current: None,
            accumulated: 0.0,
        })
    }

    pub fn add(&mut self, animation: Animation) {
        let name = animation.data.name.to_string();
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

    pub fn play(&mut self, key: &str) {
        self.is_playing = true;
        self.current = Some(key.to_string());
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
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

                animation.data.frames[i]
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
            while self.accumulated > animation.data.framerate {
                self.accumulated -= animation.data.framerate;
                animation.set_current_frame(animation.current_frame + 1);
            }
        }
    }
}

