//! Lunar venture game

use macroquad::{prelude::*, window};
use std::f32::consts::PI;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 500.0;
const SKY_BOX_SIZE: f32 = 100.0;
const MOUSE_SENS: f32 = 2.0;
const WHEEL_SENS: f32 = 0.05;
const CAMERA_DIST_MIN: f32 = 5.0;
const CAMERA_DIST_MAX: f32 = 45.0;
const CAMERA_X_ANGLE_MAX: f32 = PI / 2.0 - 0.001;
const BH_FRG_SHADER: &str = include_str!("blackhole.frg");
const BH_VRX_SHADER: &str = include_str!("blackhole.vrx");
const ROTATION_SPEED: f32 = -2.0 * PI / 120.0;

#[macroquad::main("LunarVenture")]
async fn main() {
    window::request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let skybox_texture: Texture2D = load_texture("assets/skybox.png").await.unwrap();
    let bh_material = load_material(
        ShaderSource::Glsl { vertex: BH_VRX_SHADER, fragment: BH_FRG_SHADER },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("iBhPos", UniformType::Float2),
                UniformDesc::new("iBhDist", UniformType::Float1),
            ],
            ..MaterialParams::default()
        },
    )
    .unwrap();
    bh_material.set_uniform("iResolution", (SCREEN_WIDTH, SCREEN_HEIGHT));
    bh_material.set_uniform("iBhPos", (0f32, 0f32, 0f32));

    let mut camera = Camera3D { up: Vec3::X, target: Vec3::ZERO, ..Camera3D::default() };
    let mut camera_a: f32 = 0.0;
    let mut camera_b: f32 = 0.0;
    let mut camera_dist: f32 = 10.0;

    loop {
        let mouse_delta = mouse_delta_position() * MOUSE_SENS;
        camera_a += get_frame_time().mul_add(ROTATION_SPEED, mouse_delta.x);
        camera_b = (camera_b - mouse_delta.y).clamp(-CAMERA_X_ANGLE_MAX, CAMERA_X_ANGLE_MAX);
        let (_, wheel_delta) = mouse_wheel();
        camera_dist =
            wheel_delta.mul_add(WHEEL_SENS, camera_dist).clamp(CAMERA_DIST_MIN, CAMERA_DIST_MAX);
        camera.position = vec3(
            camera_b.sin() * camera_dist,
            camera_a.cos() * camera_b.cos() * camera_dist,
            camera_a.sin() * camera_b.cos() * camera_dist,
        );

        set_camera(&camera);

        clear_background(GRAY);
        draw_affine_parallelepiped(
            vec3(-SKY_BOX_SIZE / 2.0, -SKY_BOX_SIZE / 2.0, -SKY_BOX_SIZE / 2.0),
            Vec3::X * SKY_BOX_SIZE,
            Vec3::Y * SKY_BOX_SIZE,
            Vec3::Z * SKY_BOX_SIZE,
            Some(&skybox_texture),
            WHITE,
        );

        if let Some(bh_screen_pos) = world_to_screen(Vec3::ZERO, &camera) {
            bh_material.set_uniform("iBhPos", (bh_screen_pos.x, bh_screen_pos.y));
            bh_material.set_uniform("iBhDist", camera_dist);
        }

        gl_use_material(&bh_material);
        draw_rectangle(-1.0, -1.0, 2.0, 2.0, WHITE);
        gl_use_default_material();
        next_frame().await;
    }
}

fn world_to_screen(pos: Vec3, camera: &Camera3D) -> Option<Vec2> {
    let view = Mat4::look_at_rh(camera.position, camera.target, camera.up);
    let clip = view * pos.extend(1.0);
    if clip.w <= 0.0 {
        return None;
    }
    let ndc = clip.truncate() / clip.w;
    Some(vec2((ndc.x + 1.0) * 0.5, (1.0 - ndc.y) * 0.5))
}
