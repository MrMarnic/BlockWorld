use crate::block::location::BlockLocation;
use crate::block::block_registry::BlockRegistry;
use std::sync::{Mutex, Arc};
use std::path::Path;
use nalgebra_glm::{vec3, TVec3};
use crate::world::world::World;
use binary_rw::{BinaryReader, BinaryWriter};
use crate::block::block_sides::BlockSides;
use parking_lot::RwLock;
use rust_game_library_wgpu::objects::camera::Camera;
use rust_game_library_wgpu::engine::game_engine::GameEngine;
use rust_game_library_wgpu::objects::color::Color;
use rust_game_library_wgpu::objects::vertex_buffer::{VertexBuffer, OnlyCoordsVertexBuffer};
use rust_game_library_wgpu::objects::aabb::AABB;
use winit::event::VirtualKeyCode;
use crate::data::load::Load;
use crate::data::save::Save;
use wgpu::{Device, Queue};
use crate::block::block_ray::BlockRay;
use crate::block::line_mesh::LineMesh;
use crate::block::line_mesh_renderer::LineMeshRenderer;
use crate::gui::pause_gui::PauseGui;
use crate::gui::inventory::hot_bar_inv::HotBarInventory;
use crate::gui::debug_gui::DebugGui;
use rust_game_library_wgpu::gui::gui_handler::GuiHandler;
use binary_rw::filestream::{Filestream, OpenType};

pub struct Player{
    pub camera: Camera,
    pub selected_block: BlockLocation,
    pub is_creative: bool,
    pub movement_speed: f64,
    pub fall_speed: f64,
    pub jump_speed: f64,
    pub is_sneaking: bool,
    pub is_jumping: bool,
    pub jump_height: f64,
    pub is_on_ground: bool,
    pub start_tick: i128,
    gravity_started: bool,
    vertical_momentum: f64,
    pub is_spectator: bool,
    pub hot_bar_inv: HotBarInventory,
    pub just_build: bool,
    block_registry: Arc<Mutex<BlockRegistry>>,
    block_sides: Arc<BlockSides>
}

impl Player {

    pub unsafe fn new(camera: Camera, engine:&GameEngine,block_registry:Arc<Mutex<BlockRegistry>>,block_sides:Arc<BlockSides>) -> Player{

        let mut pos = camera.transform.pos.clone();

        if Path::new(&format!("{}\\{}",engine.working_dir,"world\\player.dat")).exists() {
            let mut p = Player {camera,selected_block: BlockLocation::null(),is_creative: false, movement_speed: 4.0,fall_speed: 9.81 * 2.0,jump_speed: 7.0, is_sneaking: false, is_jumping: false, jump_height: 0.0, is_on_ground: true,
                start_tick: 0, gravity_started: false,vertical_momentum: 0.0, is_spectator: false,
                hot_bar_inv: HotBarInventory::new(engine,engine.resource_loader.get_texture("slot.png".to_string()),engine.resource_loader.get_texture("slot_selector.png".to_string()),block_registry.clone()),
                just_build: false,
                block_registry: block_registry.clone(),
                block_sides: block_sides.clone()
            };

            p.load(format!("{}\\{}",engine.working_dir,"world\\player.dat"));

            return p;
        }else {
            return Player {camera,selected_block: BlockLocation::null(),is_creative: false, movement_speed: 4.0,fall_speed: 9.81 * 2.0,jump_speed: 7.0, is_sneaking: false, is_jumping: false, jump_height: 0.0, is_on_ground: true,
                start_tick: 0, gravity_started: false,vertical_momentum: 0.0, is_spectator: false,
                hot_bar_inv: HotBarInventory::new(engine,engine.resource_loader.get_texture("slot.png".to_string()),engine.resource_loader.get_texture("slot_selector.png".to_string()),block_registry.clone()),
                just_build: false,
                block_registry: block_registry.clone(),
                block_sides: block_sides.clone()
            };
        }
    }


    pub fn create_block_ray(&self) -> BlockRay {
        return BlockRay::new(self.camera.transform.pos.clone(),vec3(self.camera.target.x,-self.camera.target.y,-self.camera.target.z),3.0);
    }

    pub fn get_block_pos(&self) -> BlockLocation {
        return BlockLocation::new_from(self.camera.transform.pos.x.round() as i32,(self.camera.transform.pos.y.round()) as i32,self.camera.transform.pos.z.ceil() as i32);
    }

    pub fn update(&mut self, engine:&GameEngine) {
        self.camera.transform.interpolate(engine);
        self.camera.update();
    }

    pub fn process_input(&mut self, engine:&mut GameEngine, world:&mut World,device:&Device,gui_handler:&mut GuiHandler,queue:&Queue,camera_2d:&Camera) {
        self.hot_bar_inv.process_input(engine);

        if engine.input_handler.is_key_clicked(VirtualKeyCode::C) {
            if self.is_creative {
                self.disable_creative(engine.current_ticks);
            }else {
                self.set_creative();
            }
        }

        if engine.input_handler.is_key_clicked(VirtualKeyCode::P) {
            if self.is_spectator {
                self.is_spectator = false;
            }else {
                self.is_spectator = true;
                self.set_creative();
            }
        }

        if engine.input_handler.is_key_clicked(VirtualKeyCode::Escape) {
            engine.tick_handler_paused = true;
            let pause_gui = PauseGui::new(engine,device,queue,camera_2d);
            engine.guis_to_open.push(Box::new(pause_gui));
        }

        if engine.input_handler.is_key_clicked(VirtualKeyCode::F2) {
            if !gui_handler.is_open("debug_gui".to_string()) {
                gui_handler.open_gui(Box::new(DebugGui::new(engine,device,queue,camera_2d,world)));
            }
        }

        self.handle_cursor_move(engine);
        self.handle_mouse_click(engine,world,self.block_registry.clone(),self.block_sides.clone(),device);
    }

    pub fn handle_tick(&mut self, engine:&mut GameEngine, world:&mut World,device:&Device) {

        self.camera.transform.handle_tick();
        self.handle_mouse_click_tick(engine,world,device);

        let delta_time = 0.02;

        let mut move_vec = vec3(0.0,0.0,0.0);

        if engine.input_handler.is_key_pressed(VirtualKeyCode::LShift) {
            if self.is_creative || self.is_spectator {
                move_vec.y -= self.movement_speed * delta_time;
            }else {
                self.activate_sneak();
            }
        }

        if engine.input_handler.is_key_pressed(VirtualKeyCode::W) || engine.input_handler.is_key_clicked(VirtualKeyCode::W) {
            let distance_x = self.camera.direction.x * self.movement_speed * delta_time;
            let distance_z = self.camera.direction.z * self.movement_speed * delta_time;
            move_vec.x += distance_x;
            move_vec.z -= distance_z;
        }
        if engine.input_handler.is_key_pressed(VirtualKeyCode::S) {
            let distance_x = self.camera.direction.x * self.movement_speed * delta_time;
            let distance_z = self.camera.direction.z * self.movement_speed * delta_time;
            move_vec.x -= distance_x;
            move_vec.z += distance_z;
        }
        if engine.input_handler.is_key_pressed(VirtualKeyCode::A) {
            let distance_x = self.camera.direction.z * self.movement_speed * delta_time;
            let distance_z = self.camera.direction.x * self.movement_speed * delta_time;
            move_vec.x -= distance_x;
            move_vec.z -= distance_z;
        }
        if engine.input_handler.is_key_pressed(VirtualKeyCode::D) {
            let distance_x = self.camera.direction.z * self.movement_speed * delta_time;
            let distance_z = self.camera.direction.x * self.movement_speed * delta_time;
            move_vec.x += distance_x;
            move_vec.z += distance_z;
        }

        if engine.input_handler.is_key_pressed(VirtualKeyCode::Space) {
            let distance_y = self.movement_speed * delta_time;
            if self.is_creative {
                move_vec.y += distance_y;
            }else {
                self.jump();
            }
        }

        if self.is_sneaking {
            self.disable_sneak();
        }

        if self.vertical_momentum != 0.0 {
            self.is_on_ground = false;
        }

        self.camera.transform.translate(vec3(move_vec.x,0.0,0.0));
        self.check_collision(self.camera.transform.new_transform.as_ref().unwrap().pos.clone(), world, vec3(move_vec.x, 0.0, 0.0));

        if !self.is_creative && !self.is_spectator {
            self.vertical_momentum -= self.fall_speed * delta_time;
        }

        self.camera.transform.translate(vec3(0.0,self.vertical_momentum * delta_time + move_vec.y,0.0));
        self.check_collision(self.camera.transform.new_transform.as_ref().unwrap().pos.clone(), world, vec3(0.0, self.vertical_momentum * delta_time + move_vec.y, 0.0));

        self.camera.transform.translate(vec3(0.0,0.0,move_vec.z));
        self.check_collision(self.camera.transform.new_transform.as_ref().unwrap().pos.clone(), world, vec3(0.0, 0.0, move_vec.z));

        if self.vertical_momentum <= 0.0 {
            self.is_jumping = false;
        }

        if !self.is_on_ground && !self.is_creative && !self.is_spectator {
            if !self.gravity_started {
                self.gravity_started = true;
                self.start_tick = engine.current_ticks;
            }
        }else {
            if self.gravity_started {
                self.gravity_started = false;
            }
        }


        if self.is_jumping {
            if move_vec.y < 0.0 {
                self.is_jumping = false;
                self.start_tick = engine.current_ticks;
            }
            self.jump_height += move_vec.y;
        }

        self.camera.update();
    }


    pub fn render_selected_block(&self,engine:&mut GameEngine, line_mesh_renderer:&LineMeshRenderer, world: &World, line_mesh: &mut LineMesh,queue:&Queue) {
        if !world.is_air(&self.selected_block) {
            line_mesh_renderer.render_line_mesh_instant(line_mesh,&mut engine.offset_handler.camera_offset,&self.camera,queue,&self.selected_block.create_world_vec());
            line_mesh.should_render = true;
        } else {
            line_mesh.should_render = false;
        }

    }

    pub fn jump(&mut self) {
        if !self.is_jumping && self.is_on_ground{
            self.is_jumping = true;
            self.vertical_momentum = self.jump_speed;
        }
    }


    pub fn check_collision(&mut self, new_pos:TVec3<f64>, world: &World, mut move_vec:TVec3<f64>) -> TVec3<f64>{
        if self.is_spectator {
            return move_vec;
        }

        let aabb = self.create_aabb(&new_pos);

        for b in self.get_block_pos().up().get_side_neighbours_and_self() {
            let b_aabb = b.get_abb();
            if b_aabb.collision_test(&aabb) {
                if !world.is_air(&b) {
                    if move_vec.y > 0.0 {
                        self.camera.transform.set_y(b_aabb.min.y - 0.25);
                        move_vec.y = 0.0;
                        self.is_jumping = false;
                        self.jump_height = 0.0;
                        self.vertical_momentum = 0.0;
                    }else if move_vec.y < 0.0 {
                        //self.camera.transform.pos.y = b_aabb.max.y + 1.5;
                        self.camera.transform.set_y(b_aabb.max.y + 1.5);
                        self.is_on_ground = true;
                        move_vec.y = 0.0;
                        self.vertical_momentum = 0.0;
                    }

                    if move_vec.x > 0.0 {
                        self.camera.transform.set_x(b_aabb.min.x - 0.35);
                        move_vec.x = 0.0;
                    } else if move_vec.x < 0.0 {
                        self.camera.transform.set_x(b_aabb.max.x + 0.35);
                        move_vec.x = 0.0;
                    }

                    if move_vec.z > 0.0 {
                        self.camera.transform.set_z(b_aabb.min.z - 0.35);
                        move_vec.z = 0.0;
                    } else if move_vec.z < 0.0 {
                        self.camera.transform.set_z(b_aabb.max.z + 0.35);
                        move_vec.z = 0.0;
                    }
                }
            }
        }

        for b in self.get_block_pos().get_side_neighbours_and_self() {
            let b_aabb = b.get_abb();
            if b_aabb.collision_test(&aabb) {
                if !world.is_air(&b) {
                    if move_vec.y > 0.0 {
                        self.camera.transform.set_y(b_aabb.min.y - 0.25);
                        move_vec.y = 0.0;
                        self.is_jumping = false;
                        self.jump_height = 0.0;
                        self.vertical_momentum = 0.0;
                    }else if move_vec.y < 0.0 {
                        //self.camera.transform.pos.y = b_aabb.max.y + 1.5;
                        self.camera.transform.set_y(b_aabb.max.y + 1.5);
                        self.is_on_ground = true;
                        move_vec.y = 0.0;
                        self.vertical_momentum = 0.0;
                    }

                    if move_vec.x > 0.0 {
                        self.camera.transform.set_x(b_aabb.min.x - 0.35);
                        move_vec.x = 0.0;
                    } else if move_vec.x < 0.0 {
                        self.camera.transform.set_x(b_aabb.max.x + 0.35);
                        move_vec.x = 0.0;
                    }

                    if move_vec.z > 0.0 {
                        self.camera.transform.set_z(b_aabb.min.z - 0.35);
                        move_vec.z = 0.0;
                    } else if move_vec.z < 0.0 {
                        self.camera.transform.set_z(b_aabb.max.z + 0.35);
                        move_vec.z = 0.0;
                    }
                }
            }
        }

        for b in self.get_block_pos().down().get_side_neighbours_and_self() {
            let b_aabb = b.get_abb();
            if b_aabb.collision_test(&aabb) {
                if !world.is_air(&b) {
                    if move_vec.y > 0.0 {
                        self.camera.transform.set_y(b_aabb.min.y - 0.25);
                        move_vec.y = 0.0;
                        self.is_jumping = false;
                        self.jump_height = 0.0;
                        self.vertical_momentum = 0.0;
                    }else if move_vec.y < 0.0 {
                        //self.camera.transform.pos.y = b_aabb.max.y + 1.5;
                        self.camera.transform.set_y(b_aabb.max.y + 1.5);
                        self.is_on_ground = true;
                        move_vec.y = 0.0;
                        self.vertical_momentum = 0.0;
                    }

                    if move_vec.x > 0.0 {
                        self.camera.transform.set_x(b_aabb.min.x - 0.35);
                        move_vec.x = 0.0;
                    } else if move_vec.x < 0.0 {
                        self.camera.transform.set_x(b_aabb.max.x + 0.35);
                        move_vec.x = 0.0;
                    }

                    if move_vec.z > 0.0 {
                        self.camera.transform.set_z(b_aabb.min.z - 0.35);
                        move_vec.z = 0.0;
                    } else if move_vec.z < 0.0 {
                        self.camera.transform.set_z(b_aabb.max.z + 0.35);
                        move_vec.z = 0.0;
                    }
                }
            }
        }


        for b in self.get_block_pos().down().down().get_side_neighbours_and_self() {
            let b_aabb = b.get_abb();
            if b_aabb.collision_test(&aabb) {
                if !world.is_air(&b) {
                    if move_vec.y > 0.0 {
                        self.camera.transform.set_y(b_aabb.min.y - 0.25);
                        move_vec.y = 0.0;
                        self.is_jumping = false;
                        self.jump_height = 0.0;
                        self.vertical_momentum = 0.0;
                    }else if move_vec.y < 0.0 {
                        //self.camera.transform.pos.y = b_aabb.max.y + 1.5;
                        self.camera.transform.set_y(b_aabb.max.y + 1.5);
                        self.is_on_ground = true;
                        move_vec.y = 0.0;
                        self.vertical_momentum = 0.0;
                    }

                    if move_vec.x > 0.0 {
                        self.camera.transform.set_x(b_aabb.min.x - 0.35);
                        move_vec.x = 0.0;
                    } else if move_vec.x < 0.0 {
                        self.camera.transform.set_x(b_aabb.max.x + 0.35);
                        move_vec.x = 0.0;
                    }

                    if move_vec.z > 0.0 {
                        self.camera.transform.set_z(b_aabb.min.z - 0.35);
                        move_vec.z = 0.0;
                    } else if move_vec.z < 0.0 {
                        self.camera.transform.set_z(b_aabb.max.z + 0.35);
                        move_vec.z = 0.0;
                    }
                }
            }
        }

        return move_vec;
    }

    pub fn handle_cursor_move(&mut self, engine:&GameEngine) {
        self.camera.transform.yaw += engine.input_handler.delta_x  * 0.4;
        self.camera.transform.pitch += engine.input_handler.delta_y  * 0.4;


        if self.camera.transform.pitch >= 89.0 {
            self.camera.transform.pitch = 89.0;
        }
        if self.camera.transform.pitch <= -89.0 {
            self.camera.transform.pitch = -89.0;
        }

        if self.camera.transform.yaw > 360.0 && engine.input_handler.delta_x > 0.0 {
            self.camera.transform.yaw = self.camera.transform.yaw - 360.0;
        }else if self.camera.transform.yaw < 0.0 && engine.input_handler.delta_x < 0.0 {
            self.camera.transform.yaw = 360.0 - self.camera.transform.yaw;
        }


        self.camera.direction.x = self.camera.transform.sin(self.camera.transform.yaw);
        self.camera.direction.y = self.camera.transform.sin(self.camera.transform.pitch);
        self.camera.direction.z = self.camera.transform.cos(self.camera.transform.yaw);
        self.camera.target.x = self.camera.direction.x;
        self.camera.target.y = self.camera.transform.tan(self.camera.transform.pitch);
        self.camera.target.z = self.camera.direction.z;
        let norm: TVec3<f64> = nalgebra_glm::normalize(&self.camera.target) as TVec3<f64>;
        self.camera.target = norm;
        self.camera.direction.normalize();
        self.camera.update();
    }

    pub fn handle_mouse_click(&mut self,engine:&mut GameEngine,world:&mut World,block_registry:Arc<Mutex<BlockRegistry>>,block_sides:Arc<BlockSides>,device:&Device) {
        unsafe {
            if engine.input_handler.is_mouse_clicked(0) {
                if world.get_type(&self.selected_block).unwrap().id.eq(&"grass") {
                    //engine.audio_handler.get_source("grass1.ogg".to_string()).play();
                }
                if world.get_type(&self.selected_block).unwrap().id.eq(&"glass") {
                    //engine.audio_handler.get_source("glass.mp3".to_string()).play();
                    //engine.audio_handler.get_source("".to_string());
                }
                world.set_block_at(block_registry.lock().unwrap().blocks["air"].clone(),&self.selected_block,block_sides.clone(),device);
            } else if engine.input_handler.is_mouse_clicked(1) {
                if !world.is_air(&self.selected_block) {
                    let ray = self.create_block_ray();
                    let loc = ray.start_air(&world);
                    if !self.is_stuck_in_loc(world,&loc) {
                        if self.hot_bar_inv.get_selected_block().id.eq(&"grass") {
                            //engine.audio_handler.get_source("grass1.ogg".to_string()).play();
                        }
                        world.set_block_at(self.hot_bar_inv.get_selected_block(),&loc,block_sides.clone(),device);
                        self.just_build = true;

                    }
                }
            }
        }
    }

    pub fn handle_mouse_click_tick(&mut self,engine:&mut GameEngine,world:&mut World,device:&Device) {
        unsafe {
            if engine.current_ticks%12 == 0 {
                if !self.just_build {
                    if engine.input_handler.is_mouse_pressed(1) {
                        if !world.is_air(&self.selected_block) {
                            let ray = self.create_block_ray();
                            let loc = ray.start_air(world);
                            if !self.is_stuck_in_loc(world,&loc) {
                                world.set_block_at(self.hot_bar_inv.get_selected_block(),&loc,self.block_sides.clone(),device);
                            }
                        }

                    }
                }
                self.just_build = false;
            }
        }
    }

    pub fn is_stuck_in_loc(&self,world:&World,loc:&BlockLocation) -> bool{

        let aabb = &self.create_aabb(&self.camera.transform.new_transform.as_ref().unwrap().pos.clone());
        let mut is_stuck = false;

        if loc.get_abb().collision_test(aabb) {
            is_stuck = true;
        }

        return is_stuck;
    }

    pub fn set_creative(&mut self) {
        self.is_creative = true;
        self.gravity_started = false;
        self.start_tick = 0;
        self.is_jumping = false;
        self.vertical_momentum = 0.0;
    }

    pub fn disable_creative(&mut self,tick: i128) {
        self.is_creative = false;
        self.start_tick = tick;
    }

    fn activate_sneak(&mut self) {
        self.is_sneaking = true;
        self.movement_speed = 1.0;
    }

    fn disable_sneak(&mut self) {
        self.is_sneaking = false;
        self.movement_speed = 5.0;
    }

    pub fn create_aabb(&self, pos: &TVec3<f64>) -> AABB {
        return AABB::new(vec3(pos.x-0.35,pos.y-1.5,pos.z-0.35),vec3(pos.x+0.35,pos.y+0.25,pos.z+0.35));
    }
}

impl Load for Player {
    fn load(&mut self, file_path: String){
        let mut file_stream = Filestream::new(file_path.as_str(),OpenType::Open).unwrap();
        let mut binary = BinaryReader::new(&mut file_stream);
        let x = binary.read_f64().unwrap();
        let y = binary.read_f64().unwrap();
        let z = binary.read_f64().unwrap();

        let pos = vec3(x,y,z);

        self.camera.transform.set_translation(pos.clone());
    }
}

impl Save for Player {
    fn save(&self, file_path: String) {
        let mut file_stream = Filestream::new(file_path.as_str(),OpenType::OpenAndCreate).unwrap();
        let mut binary = BinaryWriter::new(&mut file_stream);
        binary.write_f64(self.camera.transform.pos.x);
        binary.write_f64(self.camera.transform.pos.y);
        binary.write_f64(self.camera.transform.pos.z);
    }
}