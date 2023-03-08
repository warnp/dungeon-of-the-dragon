use std::collections::{BTreeMap, HashMap};
use std::ops::Not;
use std::str::from_utf8;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use ggez::{event, GameError, graphics};
use ggez::{Context, GameResult};
use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::event::MouseButton;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect, Text};
use keyframe::{AnimationSequence, functions, keyframes};
use keyframe_derive::CanTween;
use crate::gui::graphical::sprite::{Layer, Sprite};
use crate::interact::actions::Actions;
use crate::inventory::item::{DamageTypeEnum, ItemAttackTypeEnum};
use crate::services::animator::Animator;
use crate::services::messaging::MessageContent;

const SPRITE_SIZE: i32 = 32;

pub struct MainState {
    sprites_movables: Vec<(Image, DrawParam)>,
    sprites_background: Vec<(Image, DrawParam)>,
    sprites_ui: Vec<(Image, DrawParam)>,
    particles: Vec<(Image, DrawParam, Instant, u8)>,
    animation_duration: u64,
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
    active_modal: Option<(f32, f32, String)>,
    gameplay_state: Option<Actions>,
    sprites_clicked: Vec<(f32, f32, Sprite)>,
    animator: Animator,
}

impl Default for MainState {
    fn default() -> Self {


        MainState {
            sprites_movables: vec![],
            sprites_background: vec![],
            sprites_ui: vec![],
            particles: vec![],
            animation_duration: 1,
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
            active_modal: None,
            gameplay_state: None,
            sprites_clicked: vec![],
            animator: Animator::new(),
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
        textures.insert(1, Image::from_path(ctx, "/selector.png").unwrap());
        textures.insert(2, Image::from_path(ctx, "/possible_area.png").unwrap());
        textures.insert(10, Image::from_path(ctx, "/dungeon_ground.png").unwrap());
        textures.insert(11, Image::from_path(ctx, "/door.png").unwrap());
        textures.insert(12, Image::from_path(ctx, "/door.png").unwrap());
        textures.insert(100, Image::from_path(ctx, "/particles.png").unwrap());
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
                        .scale(Vec2::new(5f32, 5f32)));

        options.iter()
            .enumerate()
            .for_each(|(i, el)| {
                self.menu_buttons.push(Rect::new(x + 10., (y + i as f32 * 20.) + 10.0, 3. * 32., 15.));

                canvas.draw(&Text::new(el),
                            graphics::DrawParam::from([x, y])
                                .color(Color::WHITE)
                                .scale(Vec2::new(1., 1.))
                                .dest(Vec2::new(x + 10., (y + i as f32 * 20.) + 10.)));
            });

        Ok(())
    }

    fn draw_modal(&mut self, canvas: &mut Canvas, x: f32, y: f32, content: &str) -> GameResult<()> {
        canvas.draw(self.sprites_textures.get(&(0 as u8))
                        .unwrap(),
                    DrawParam::new()
                        .dest(Vec2::new(x, y))
                        .scale(Vec2::new(7.5f32, 6.5f32)));

        canvas.draw(&Text::new(content),
                    graphics::DrawParam::from([x, y])
                        .color(Color::WHITE)
                        .scale(Vec2::new(1., 1.))
                        .dest(Vec2::new(x + 10., y + 10.)));
        Ok(())
    }

    fn mouse_hovering_characterisation(&mut self, x: f32, y: f32, sprites: Vec<Sprite>) {
        if let Some(gameplay_state) = self.gameplay_state.clone() {
            match gameplay_state {
                Actions::WATCH => self.watch_action(&x, &y, sprites),
                Actions::ATTACK => self.attack_action(&x, &y, sprites),
                _ => ()
            }
        }
    }

    fn set_gameplay_state(&mut self) {
        if let Ok(state_content) = self.receivers.get("gameplay_state").unwrap().try_recv() {
            self.gameplay_state = Some(bincode::deserialize(state_content.content.as_slice()).unwrap());
        }
    }

    fn attack_action(&mut self, x: &f32, y: &f32, sprites: Vec<Sprite>) {
        //Send click position info
        self.send_info_message(&x, &y);
    }

    fn send_info_message(&mut self, x: &f32, y: &f32) {
        self.senders.get("info").unwrap().send(MessageContent {
            topic: "info".to_string(),
            content: bincode::serialize(&((x / SPRITE_SIZE as f32).floor() as u16, (y / SPRITE_SIZE as f32).floor() as u16)).unwrap(),
        }).unwrap();
    }

    fn watch_action(&mut self, x: &f32, y: &f32, sprites: Vec<Sprite>) {
        //Send click position info
        self.send_info_message(&x, &y);
        self.sprites_clicked = sprites.iter()
            .map(|s| (x.clone(), y.clone(), s.clone()))
            .collect::<Vec<(f32, f32, Sprite)>>();
    }

    fn get_all_targetables_cell_to_sprites(&self) -> Vec<Sprite> {
        //Get all targetables cells
        let targetables_receiver = self.receivers.get("targetable").unwrap();
        let targetable_coordinates: Vec<Vec<bool>> = if let Ok(targetable) = targetables_receiver.try_recv() {
            bincode::deserialize(targetable.content.as_slice()).unwrap()
        } else {
            Vec::new()
        };
        targetable_coordinates.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate()
                    .filter(|(x, &cell)| cell)
                    .map(|(x, &cell)|
                        Sprite::new(2, x as i32, y as i32, Layer::UI)
                    )
                    .collect::<Vec<Sprite>>()
            })
            .collect::<Vec<Sprite>>()
    }

    fn wait_for_watch(&mut self) {
        let hovering_info =
            if let Ok(response) = self.receivers.get("info_response").unwrap().try_recv() {
                Some(format!("{}", from_utf8(response.content.as_slice()).unwrap()))
            } else {
                None
            };

        if let Some(info) = hovering_info {
            self.active_modal = {
                if self.sprites_clicked.is_empty().not() {
                    let first_element = self.sprites_clicked.first().unwrap();
                    Some((first_element.0.clone(), first_element.1.clone(), info.to_string()))
                } else {
                    None
                }
            };

            self.clear_after_turn();
        }
    }

    fn clear_after_turn(&mut self) {
        self.sprites_clicked.clear();
        self.sprites_ui.clear();
        self.gameplay_state = None;
    }

    fn wait_for_attack(&mut self) {
        if let Ok(response) = self.receivers.get("info_response").unwrap().try_recv() {
            if let Ok(ending_attack_turn) = from_utf8( response.content.as_slice()) {
                if ending_attack_turn == "end_attack" {
                    self.clear_after_turn();
                    return;
                }
            }
            if let Ok(target_position) = bincode::deserialize::<((u16, u16), DamageTypeEnum)>(response.content.as_slice()) {

                let sprite = Sprite::new(1, target_position.0.0 as i32, target_position.0.1 as i32, Layer::UI);
                self.sprites_ui.append(&mut vec![sprite.create_drawable(SPRITE_SIZE as f32, &self.sprites_textures)]);
                let attack_particle = Sprite::new(100, target_position.0.0 as i32, target_position.0.1 as i32, Layer::PARTICLE)
                    .create_drawable(SPRITE_SIZE as f32, &self.sprites_textures);

                let damage_type = match target_position.1 {
                    DamageTypeEnum::SLASHING => 1,
                    DamageTypeEnum::FIRE => 0,
                    _ => 0
                };

                self.particles.push((attack_particle.0, attack_particle.1, Instant::now(), damage_type));
            }
        } else {
            let mut targetable_cells = self.get_all_targetables_cell_to_sprites();
            self.sprites_ui.append(&mut targetable_cells.iter()
                .filter(|s| s.layer == Layer::UI)
                .map(|e| e.create_drawable(SPRITE_SIZE as f32, &self.sprites_textures))
                .collect::<Vec<(Image, DrawParam)>>());

            self.sprites.append(&mut targetable_cells);
        }
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> Result<(), GameError> {
        if button != MouseButton::Left {
            return Ok(());
        }

        //If some modal exist, we close it on click
        if let Some(a_m) = self.active_modal.clone() {
            self.senders.get("info").unwrap().send(MessageContent {
                topic: "info".to_string(),
                content: vec![],
            }).unwrap();

            self.active_modal = None;
            return Ok(());
        }

        let button_clicked = self.menu_buttons.iter()
            .filter(|b| b.x < x && b.x + b.w > x &&
                b.y < y && b.y + b.h > y)
            .map(|el| el.clone())
            .collect::<Vec<Rect>>();

        if button_clicked.len() > 0 {
            self.selected_menu_option = self.menu_buttons.iter()
                .position(|b| b.x < x && b.x + b.w > x &&
                    b.y < y && b.y + b.h > y);

            if let Some(menu_option) = self.selected_menu_option {
                self.senders.get("select_response").unwrap().send(MessageContent {
                    topic: "select_response".to_string(),
                    content: bincode::serialize(&menu_option).unwrap(),
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
        if !sprites_selected.is_empty() {
            self.mouse_hovering_characterisation(x, y, sprites_selected);
        }


        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let point2 = ctx.mouse.position();
        self.set_gameplay_state();

        if let Some(clear_container) = self.receivers.get("clear") {
            if let Ok(clear) = clear_container.try_recv() {
                self.stdout.clear();
            }
        }

        //Get stdout
        if let Some(stdout_container) = self.receivers.get("stdout") {
            if let Ok(text) = stdout_container.try_recv() {
                let out = format!("{}\n{}", self.stdout, from_utf8(text.content.as_slice()).unwrap());
                self.stdout = out;
            }
        }

        //Get menu
        if let Some(select_container) = self.receivers.get("select") {
            if let Ok(text) = select_container.try_recv() {
                self.current_menu = from_utf8(text.content.as_slice())
                    .unwrap()
                    .split(":")
                    .map(|el| el.to_string())
                    .collect();
            }
        }

        //Get sprites
        if let Some(receiver) = self.receivers.get("sprite") {
            if let Ok(sprites) = receiver.try_recv() {
                let sprites: Vec<Sprite> = bincode::deserialize(sprites.content.as_slice()).unwrap();

                self.sprites_movables = sprites.iter()
                    .filter(|s| s.layer == Layer::MOVABLES)
                    .map(|e| e.create_drawable(SPRITE_SIZE as f32, &self.sprites_textures))
                    .collect::<Vec<(Image, DrawParam)>>();

                self.sprites_background = sprites.iter()
                    .filter(|s| s.layer == Layer::BACKGROUND)
                    .map(|e| e.create_drawable(SPRITE_SIZE as f32, &self.sprites_textures))
                    .collect::<Vec<(Image, DrawParam)>>();

                self.sprites_ui = sprites.iter()
                    .filter(|s| s.layer == Layer::UI)
                    .map(|e| e.create_drawable(SPRITE_SIZE as f32, &self.sprites_textures))
                    .collect::<Vec<(Image, DrawParam)>>();

                self.sprites = sprites
            }
        }

        if let Some(state) = self.gameplay_state.clone() {
            match state {
                Actions::OPEN => {}
                Actions::ATTACK => self.wait_for_attack(),
                Actions::WALK_TO => {}
                Actions::WATCH => self.wait_for_watch(),
                Actions::USE => {}
                Actions::EQUIP => {}
            }
        }

        self.mouse.set_pointer_position(point2.x, point2.y);
        self.animator.advance(1., ctx.time.delta().as_secs_f64());

        self.particles.retain(|p: &(Image,DrawParam, Instant, u8)|  p.2.elapsed() < Duration::new(self.animation_duration as u64,0));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let fps = ctx.time.fps();
        ctx.gfx.set_window_title(format!("fps: {0:.0}", fps).as_str());
        let mut canvas = Canvas::from_frame(
            ctx,
            graphics::Color::from([0., 0., 0., 1.0]),
        );

        for mesh in &self.sprites_background {
            canvas.draw(&mesh.0, mesh.1);
        }
        for mesh in &self.sprites_movables {
            canvas.draw(&mesh.0, mesh.1);
        }
        for particle in &self.particles {
            let mut local_clone = particle.clone();
            canvas.draw(&particle.0, local_clone.1
                .src(self.animator.get_currenct_rect(local_clone.3 as usize)));
        }
        for mesh in &self.sprites_ui {
            canvas.draw(&mesh.0, mesh.1);
        }

        if self.current_menu.len() > 0 {
            let options = self.current_menu.clone();
            self.draw_menu(&mut canvas, 0., 200.0, options)?;
        }

        canvas.draw(&Text::new(self.stdout.clone()),
                    graphics::DrawParam::from(Vec2::new(200.0, 0.0)).color(Color::WHITE).scale(Vec2::new(1., 1.)));

        if let Some((x, y, content)) = self.active_modal.clone() {
            self.draw_modal(&mut canvas, x, y, content.as_str())?;
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