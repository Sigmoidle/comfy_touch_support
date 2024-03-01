use crate::*;

pub struct AnimatedSprite {
    pub animations: HashMap<String, Animation>,
    pub state: AnimationState,

    pub z_index: i32,
    pub size: Vec2,
    pub color: Color,

    pub rotation_x: f32,

    pub flip_x: bool,
    pub flip_y: bool,

    pub blend_mode: BlendMode,
    pub offset: Vec2,

    pub on_finished: ContextFn,

    pub y_sort_offset: f32,
    pub despawn_on_finish: bool,
}

impl AnimatedSprite {
    pub fn play(&mut self, animation_name: &str) {
        if let Some(animation) = self.animations.get(animation_name) {
            if animation.name != self.state.animation_name {
                self.state = animation.to_state();
            }
        }
    }

    pub fn set_animations(&mut self, animations: Vec<Animation>) {
        self.state =
            animations.first().expect("animations can't be empty").to_state();

        self.animations.clear();

        for animation in animations.into_iter() {
            self.animations.insert(animation.name.clone(), animation);
        }
    }

    pub fn with_blend_mode(self, blend_mode: BlendMode) -> Self {
        Self { blend_mode, ..self }
    }
}

impl ToQuadDraw for AnimatedSprite {
    fn to_quad_draw(&self, transform: &Transform) -> QuadDraw {
        let (texture, source_rect) = self.state.current_rect();

        QuadDraw {
            transform: *transform,
            texture: texture_id(&texture),
            z_index: self.z_index,
            color: self.color,
            blend_mode: self.blend_mode,
            dest_size: self.size * transform.scale,
            source_rect,
            rotation_x: self.rotation_x,
            flip_x: self.flip_x,
            flip_y: self.flip_y,
            y_sort_offset: self.y_sort_offset,
        }
    }
}

pub struct AnimatedSpriteBuilder {
    pub animations: HashMap<String, Animation>,
    pub state: Option<AnimationState>,
    pub z_index: i32,
    pub size: Vec2,
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub rotation_x: f32,
    pub blend_mode: BlendMode,
    pub offset: Vec2,
    pub on_finished: Option<ContextFn>,
    pub y_sort_offset: f32,
    pub despawn_on_finish: bool,
}

impl AnimatedSpriteBuilder {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            state: None,
            z_index: 0,
            size: splat(1.0),
            color: WHITE,
            flip_x: false,
            flip_y: false,
            rotation_x: 0.0,
            blend_mode: BlendMode::None,
            offset: Vec2::ZERO,
            on_finished: None,
            y_sort_offset: 0.0,
            despawn_on_finish: true,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn rotation_x(mut self, rotation_x: f32) -> Self {
        self.rotation_x = rotation_x;
        self
    }

    pub fn flip_x(mut self, flip_x: bool) -> Self {
        self.flip_x = flip_x;
        self
    }

    pub fn flip_y(mut self, flip_y: bool) -> Self {
        self.flip_y = flip_y;
        self
    }

    pub fn blend_mode(mut self, blend_mode: BlendMode) -> Self {
        self.blend_mode = blend_mode;
        self
    }

    pub fn y_sort_offset(mut self, y_sort_offset: f32) -> Self {
        self.y_sort_offset = y_sort_offset;
        self
    }

    pub fn on_finished(mut self, on_finished: ContextFn) -> Self {
        self.on_finished = Some(on_finished);
        self
    }

    pub fn with_animations(mut self, animations: Vec<Animation>) -> Self {
        assert!(
            self.state.is_none(),
            "with_animations can only be used on a new AnimatedSpriteBuilder"
        );

        self.state = Some(
            animations.first().expect("animations can't be empty").to_state(),
        );

        for animation in animations.into_iter() {
            self.animations.insert(animation.name.clone(), animation);
        }

        self
    }

    pub fn despawn_on_finish(mut self, despawn_on_finish: bool) -> Self {
        self.despawn_on_finish = despawn_on_finish;
        self
    }

    pub fn add_anim(mut self, animation: Animation) -> Self {
        if self.state.is_none() {
            self.state = Some(animation.to_state());
        }

        self.animations.insert(animation.name.to_string(), animation);

        self
    }

    pub fn add_animation(
        mut self,
        name: &str,
        frame_time: f32,
        looping: bool,
        source: AnimationSource,
    ) -> AnimatedSpriteBuilder {
        let animation =
            Animation { name: name.to_string(), frame_time, looping, source };

        if self.state.is_none() {
            self.state = Some(animation.to_state());
        }


        self.animations.insert(name.to_string(), animation);

        self
    }

    pub fn with_timer(mut self, timer: f32) -> Self {
        let state = self
            .state
            .as_mut()
            .expect("with_timer() can be only used after adding an animation");

        state.timer = timer;

        self
    }

    pub fn build(self) -> AnimatedSprite {
        AnimatedSprite {
            animations: self.animations,
            state: self
                .state
                .expect("AnimatedSpriteBuilder's `state` must be set."),
            z_index: self.z_index,
            size: self.size,
            color: self.color,
            flip_x: self.flip_x,
            flip_y: self.flip_y,
            rotation_x: self.rotation_x,
            blend_mode: self.blend_mode,
            offset: self.offset,
            on_finished: self.on_finished.unwrap_or_else(|| Box::new(|_| {})),
            y_sort_offset: self.y_sort_offset,
            despawn_on_finish: self.despawn_on_finish,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Animation {
    // TODO: we need a better way of identifying animations when doing .play()
    // to avoid excessive string allocations
    pub name: String,
    pub source: AnimationSource,
    pub looping: bool,
    pub frame_time: f32,
}

impl Animation {
    pub fn to_state(&self) -> AnimationState {
        AnimationState {
            animation_name: self.name.clone(),
            source: self.source.clone(),
            interval: self.frame_time,
            looping: self.looping,
            timer: 0.0,
            current_frame: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AnimationSource {
    Files {
        prefix: Cow<'static, str>,
        frames: i32,
    },
    Atlas {
        name: Cow<'static, str>,
        offset: IVec2,
        step: IVec2,
        size: IVec2,
        frames: i32,
    },
    Spritesheet {
        name: Cow<'static, str>,
        spritesheet: Spritesheet,
    },
}

impl AnimationSource {
    pub fn frames(&self) -> i32 {
        match self {
            AnimationSource::Files { frames, .. } => *frames,
            AnimationSource::Atlas { frames, .. } => *frames,
            AnimationSource::Spritesheet { spritesheet, .. } => {
                (spritesheet.rows * spritesheet.columns) as i32
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnimationState {
    pub animation_name: String,
    pub source: AnimationSource,
    pub interval: f32,
    pub looping: bool,
    pub timer: f32,
    pub current_frame: i32,
}

impl AnimationState {
    pub fn new(
        animation_name: String,
        source: AnimationSource,
        time: f32,
        looping: bool,
    ) -> Self {
        Self {
            animation_name,
            looping,
            interval: time / source.frames() as f32,
            timer: 0.0,
            current_frame: 0,
            source,
        }
    }

    pub fn with_timer(self, timer: f32) -> Self {
        Self { timer, ..self }
    }

    pub fn progress(&self) -> f32 {
        self.timer / (self.interval * self.source.frames() as f32)
    }

    pub fn update_and_finished(&mut self, delta: f32) -> bool {
        let mut should_despawn = false;

        self.timer += delta;

        let idx = (self.timer / self.interval) as i32;
        let frames = self.source.frames();

        if idx >= frames && !self.looping {
            should_despawn = true;
        }

        if self.looping {
            self.current_frame = idx % frames;
        } else if idx >= frames {
            self.current_frame = frames - 1;
        } else {
            self.current_frame = idx;
        }

        should_despawn
    }

    pub fn current_rect(&self) -> (Cow<'static, str>, Option<IRect>) {
        match self.source {
            AnimationSource::Files { ref prefix, .. } => {
                (
                    Into::<Cow<'static, str>>::into(format!(
                        "{}{}",
                        prefix, self.current_frame
                    )),
                    None,
                )
            }
            AnimationSource::Atlas { ref name, offset, step, size, .. } => {
                (
                    name.clone(),
                    Some(IRect::new(offset + step * self.current_frame, size)),
                )
            }
            AnimationSource::Spritesheet { ref name, spritesheet } => {
                // let image_size = Assets::image_size(texture_id(name))
                //     .unwrap_or_else(|| {
                //         error!("failed to get size for {name}");
                //         uvec2(64, 64)
                //     })
                //     .as_ivec2();

                let image_size = match Assets::image_size(texture_id(name)) {
                    ImageSizeResult::Loaded(size) => size,
                    // TODO: this is probably not the best way to handle this
                    ImageSizeResult::LoadingInProgress => uvec2(64, 64),
                    _ => {
                        error!("NO SIZE FOR TEXTURE {:?}", name);
                        uvec2(64, 64)
                    }
                }
                .as_ivec2();


                let size = ivec2(
                    image_size.x / spritesheet.columns as i32,
                    image_size.y / spritesheet.rows as i32,
                );

                let row = self.current_frame / spritesheet.columns as i32;
                let col = self.current_frame % spritesheet.columns as i32;

                let offset = ivec2(col, row) * size;

                let rect = IRect::new(offset, size);


                (name.clone(), Some(rect))
            }
        }
    }
}
