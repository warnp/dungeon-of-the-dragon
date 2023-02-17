use std::collections::BTreeMap;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use ggez::{event, graphics};
use ggez::{Context, GameResult};
use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::glam::Vec2;
use ggez::graphics::{Color, DrawMode, DrawParam, Image, Mesh, Rect, Text, TextFragment};
use crate::gui::graphical::sprite::{Layer, ObjectToSprite, Sprite};
use crate::services::messaging::{MessageContent, Messaging};

const SPRITE_SIZE: i32 = 32;

pub struct MainState {
    sprites_movables: Vec<(Image, DrawParam)>,
    sprites_background: Vec<(Image, DrawParam)>,
    sprites_ui: Vec<(Image, DrawParam)>,
    mouse: Mouse,
    messenger: Option<Arc<Mutex<Messaging>>>,
    sprites_textures: BTreeMap<u8, Image>,
}

impl Default for MainState {
    fn default() -> Self {
        MainState {
            sprites_movables: vec![],
            sprites_background: vec![],
            sprites_ui: vec![],
            mouse: Default::default(),
            messenger: None,
            sprites_textures: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct Mouse {
    pos_x: f32,
    pos_y: f32,
}

impl Mouse {
    pub fn set_pointer_position(&mut self, x: f32, y: f32) {
        self.pos_x = x;
        self.pos_y = y;
    }

    pub fn get_mesh(&self, ctx: &Context) -> Mesh {
        Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(self.pos_x, self.pos_y, 20., 20.), Color::RED).unwrap()
    }
}

impl MainState {
    fn new(ctx: &Context, messenger: Arc<Mutex<Messaging>>) -> GameResult<MainState> {
        let mouse = Mouse {
            pos_y: 0.,
            pos_x: 0.,
        };

        let mut textures = BTreeMap::new();
        textures.insert(0, Image::from_path(ctx, "/dungeon_ground.png").unwrap());
        textures.insert(10, Image::from_path(ctx, "/dungeon_ground.png").unwrap());
        textures.insert(11, Image::from_path(ctx, "/dungeon_ground.png").unwrap());
        textures.insert(200, Image::from_path(ctx, "/warrior.png").unwrap());
        textures.insert(201, Image::from_path(ctx, "/goblin.png").unwrap());


        let s = MainState {
            mouse,
            messenger: Some(messenger),
            sprites_textures: textures,
            ..Default::default()
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let point2 = ctx.mouse.position();
        {
            let messenger_clone = self.messenger.as_ref().unwrap().clone();
            let guard = {
                loop {
                    // println!("window l 85 toto");
                    if let Ok(messenger) = messenger_clone.try_lock() {
                        break messenger;
                    }
                }
            };

            let (_, sprite_sub_receiver) = guard.get_subscription("sprite").unwrap();

            if let Ok(sprites) = sprite_sub_receiver.try_recv() {
                let image_creation = |s: &Sprite| {
                    let param = DrawParam::new().dest(Vec2::new((s.pos_x * SPRITE_SIZE) as f32, (s.pos_y * SPRITE_SIZE) as f32));
                    (self.sprites_textures.get(&s.texture_id).unwrap().clone(), param)
                };

                let sprites: Vec<Sprite> = bincode::deserialize(sprites.content.as_slice()).unwrap();


                self.sprites_movables = sprites.iter()
                    .filter(|s| s.layer == Layer::MOVABLES)
                    .map(image_creation)
                    .collect::<Vec<(Image, DrawParam)>>();

                self.sprites_background = sprites.iter()
                    .filter(|s| s.layer == Layer::BACKGROUND)
                    .map(image_creation)
                    .collect::<Vec<(Image, DrawParam)>>();

                self.sprites_ui = sprites.iter()
                    .filter(|s| s.layer == Layer::UI)
                    .map(image_creation)
                    .collect::<Vec<(Image, DrawParam)>>();
            }
        }
        self.mouse.set_pointer_position(point2.x, point2.y);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let fps = ctx.time.fps();
        ctx.gfx.set_window_title(format!("fps: {}", fps).as_str());
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0., 0., 0., 1.0]),
        );

        for mesh in &self.sprites_background {
            canvas.draw(&mesh.0, mesh.1);
        }
        for mesh in &self.sprites_movables {
            canvas.draw(&mesh.0, mesh.1);
        }
        for mesh in &self.sprites_ui {
            canvas.draw(&mesh.0, mesh.1);
        }

        canvas.draw(&Text::new(format!("Bonjour {}", fps)), graphics::DrawParam::from([200.0, 0.0]).color(Color::WHITE).scale(Vec2::new(10., 10.)));

        canvas.draw(&self.mouse.get_mesh(&ctx), Vec2::new(0.0, 0.0));

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn init(messenger: Arc<Mutex<Messaging>>) -> GameResult {
    let mut cb = ggez::ContextBuilder::new("super simple", "ggez")
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .window_setup(WindowSetup::default().samples(NumSamples::Four));
    let (mut ctx, event_loop) = cb.build()?;


    let state = MainState::new(&ctx, messenger.clone())?;
    event::run(ctx, event_loop, state)
}