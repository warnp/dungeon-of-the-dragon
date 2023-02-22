use std::collections::{BTreeMap, HashMap};
use std::str::from_utf8;
use std::sync::mpsc::{Receiver, Sender};
use ggez::{event, GameError, graphics};
use ggez::{Context, GameResult};
use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::event::MouseButton;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect, Text};
use crate::gui::graphical::sprite::{Layer, Sprite};
use crate::services::messaging::MessageContent;

const SPRITE_SIZE: i32 = 32;

pub struct MainState {
    sprites_movables: Vec<(Image, DrawParam)>,
    sprites_background: Vec<(Image, DrawParam)>,
    sprites_ui: Vec<(Image, DrawParam)>,
    mouse: Mouse,
    receivers: HashMap<String, Receiver<MessageContent>>,
    senders: HashMap<String, Sender<MessageContent>>,
    sprites_textures: BTreeMap<u8, Image>,
    stdout: String,
    current_menu: Vec<String>,
    sprites: Vec<Sprite>,
    menu_to_show: Vec<((f32, f32), Vec<String>)>,
    menu_buttons: Vec<Rect>,
    selected_menu_option: Option<usize>,
}

impl Default for MainState {
    fn default() -> Self {
        MainState {
            sprites_movables: vec![],
            sprites_background: vec![],
            sprites_ui: vec![],
            mouse: Default::default(),
            receivers: HashMap::new(),
            senders: HashMap::new(),
            sprites_textures: Default::default(),
            stdout: String::new(),
            current_menu: vec![],
            sprites: vec![],
            menu_to_show: vec![],
            menu_buttons: vec![],
            selected_menu_option: None,
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
    fn new(ctx: &Context, receivers: HashMap<String, Receiver<MessageContent>>, senders: HashMap<String, Sender<MessageContent>>) -> GameResult<MainState> {
        let mouse = Mouse {
            pos_y: 0.,
            pos_x: 0.,
        };

        let mut textures = BTreeMap::new();
        textures.insert(0, Image::from_path(ctx, "/menu_background.png").unwrap());
        textures.insert(10, Image::from_path(ctx, "/dungeon_ground.png").unwrap());
        textures.insert(11, Image::from_path(ctx, "/dungeon_ground.png").unwrap());
        textures.insert(12, Image::from_path(ctx, "/dungeon_ground.png").unwrap());
        textures.insert(200, Image::from_path(ctx, "/warrior.png").unwrap());
        textures.insert(201, Image::from_path(ctx, "/goblin.png").unwrap());


        let s = MainState {
            mouse,
            receivers,
            senders,
            sprites_textures: textures,
            ..Default::default()
        };
        Ok(s)
    }

    fn draw_menu(&mut self, canvas: &mut Canvas, x: f32, y: f32, options: Vec<String>) -> GameResult<()> {
        canvas.draw(self.sprites_textures.get(&(0 as u8))
                        .unwrap(),
                    DrawParam::new()
                        .dest(Vec2::new(x, y))
                        .scale(Vec2::new(3f32, 5f32)));

        options.iter()
            .enumerate()
            .for_each(|(i, el)| {
                self.menu_buttons.push(Rect::new(x, y + i as f32 * 20., 3. * 32., 15.));

                canvas.draw(&Text::new(el),
                            graphics::DrawParam::from([x, y])
                                .color(Color::WHITE)
                                .scale(Vec2::new(1., 1.))
                                .dest(Vec2::new(x, y + i as f32 * 20.)));
            });

        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> Result<(), GameError> {
        if button != MouseButton::Left {
            return Ok(());
        }

        let button_clicked = self.menu_buttons.iter()
            .filter(|b| b.x < x && b.x + b.w > x &&
                b.x < x && b.x + b.w > x)
            .map(|el| el.clone())
            .collect::<Vec<Rect>>();

        if button_clicked.len() > 0 {
            self.selected_menu_option = self.menu_buttons.iter()
                .position(|b| b.x < x && b.x + b.w > x &&
                    b.x < x && b.x + b.w > x);
            if let Some(menu_option) = self.selected_menu_option {
                self.senders.get("select_response").unwrap().send( MessageContent {
                    topic: "select_response".to_string(),
                    content: menu_option.to_be_bytes().to_vec(),
                }).unwrap();
            }
            return Ok(());
        }

        let sprites_selected = self.sprites.iter()
            .filter(|s| s.pos_y * SPRITE_SIZE < y as i32 && s.pos_y * SPRITE_SIZE + SPRITE_SIZE > y as i32 &&
                s.pos_x * SPRITE_SIZE < x as i32 && s.pos_x * SPRITE_SIZE + SPRITE_SIZE > x as i32)
            .map(|e| e.clone())
            .collect::<Vec<Sprite>>();

        //We check if user has clicked on something interactable and if interactions are availables
        if sprites_selected.len() > 0 {
            todo!()
        }


        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let point2 = ctx.mouse.position();

        //Get stdout
        if let Some(stdout_container) = self.receivers.get("stdout") {
            if let Ok(text) = stdout_container.try_recv() {
                self.stdout = format!("{}\n{}", self.stdout, from_utf8(text.content.as_slice()).unwrap());
            }
        }

        //Get menu
        if let Some(select_container) = self.receivers.get("select") {
            if let Ok(text) = select_container.try_recv() {
                self.current_menu = from_utf8(text.content.as_slice())
                    .unwrap()
                    .split("\n")
                    .map(|el| el.to_string())
                    .collect();
            }
        }

        //Get sprites
        if let Some(receiver) = self.receivers.get("sprite") {
            if let Ok(sprites) = receiver.try_recv() {
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

                // self.sprites_ui = sprites.iter()
                //     .filter(|s| s.layer == Layer::UI)
                //     .map(image_creation)
                //     .collect::<Vec<(Image, DrawParam)>>();

                self.sprites = sprites
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

        canvas.draw(&Text::new(format!("{}", self.stdout)),
                    graphics::DrawParam::from([200.0, 0.0]).color(Color::WHITE).scale(Vec2::new(1., 1.)));

        if self.current_menu.len() > 0 {
            let options = self.current_menu.clone();
            self.draw_menu(&mut canvas, 0., 200.0, options)?;
        }
        canvas.draw(&self.mouse.get_mesh(&ctx), Vec2::new(0.0, 0.0));

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn init(receivers: HashMap<String, Receiver<MessageContent>>, senders: HashMap<String, Sender<MessageContent>>) -> GameResult {
    let cb = ggez::ContextBuilder::new("super simple", "ggez")
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .window_setup(WindowSetup::default().samples(NumSamples::Four));
    let (mut ctx, event_loop) = cb.build()?;


    let state = MainState::new(&ctx, receivers, senders)?;
    event::run(ctx, event_loop, state)
}