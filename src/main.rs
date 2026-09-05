mod cube;
mod framebuffer;
mod material;
mod ray;
mod renderer;
mod texture;
mod vec3;
use cube::Cube;
use framebuffer::Framebuffer;
use material::Material;
use raylib::prelude::KeyboardKey;
use vec3::Vec3;
const IMAGE_WIDTH: usize = 480;
const IMAGE_HEIGHT: usize = 360;
fn main() {
    let (mut window, thread) = raylib::init()
        .size(1000, 780)
        .title("Cubo raytracing: texturas y sombras")
        .resizable()
        .build();
    window.set_target_fps(60);
    let texture = texture::Texture::from_png("assets/ceramica_geometrica.png")
        .expect("No se pudo cargar la textura");
    let mut cube = Cube::new(
        Vec3::new(0.0, 0.0, -4.2),
        1.25,
        Vec3::new(0.48, -0.58, 0.0),
        Material::textured(texture, 0.14, 0.82, 0.48, 72.0),
    );
    let mut fb = Framebuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    while !window.window_should_close() {
        let dt = window.get_frame_time();
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            cube.rotation.y -= dt * 1.5
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            cube.rotation.y += dt * 1.5
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            cube.rotation.x -= dt * 1.5
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            cube.rotation.x += dt * 1.5
        }
        renderer::render(&mut fb, &cube);
        fb.draw(&mut window, &thread);
    }
}
