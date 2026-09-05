mod cube;
mod framebuffer;
mod material;
mod ray;
mod renderer;
mod vec3;

use cube::Cube;
use framebuffer::Framebuffer;
use material::Material;
use vec3::Vec3;

const IMAGE_WIDTH: usize = 480;
const IMAGE_HEIGHT: usize = 360;

fn main() {
    let (mut window, thread) = raylib::init()
        .size(1000, 780)
        .title("cubo primer ejercicio")
        .resizable()
        .build();
    window.set_target_fps(60);
    let cube = Cube::new(
        Vec3::new(0.0, 0.0, -4.2),
        1.25,
        Vec3::new(0.48, -0.58, 0.0),
        Material::new(0.16, 0.78, 0.42, 64.0),
    );
    let mut framebuffer = Framebuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    renderer::render(&mut framebuffer, &cube);
    while !window.window_should_close() {
        framebuffer.draw(&mut window, &thread);
    }
}
