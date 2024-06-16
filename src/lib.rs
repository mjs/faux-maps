use noise::{utils::*, Clamp, Fbm, Seedable};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

const SEA_LEVEL: f64 = -0.04;
const PERSISTENCE: f64 = 0.4;
const SCALING: f64 = 800.0;

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    seed: u32,
) -> Result<(), JsValue> {
    let data = generate_map(width, height, seed);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn generate_map(width: u32, height: u32, seed: u32) -> Vec<u8> {
    let width: usize = width.try_into().unwrap();
    let height: usize = height.try_into().unwrap();
    let out_scale = width as f64 / SCALING;

    let mut land_base = Fbm::new().set_seed(seed);
    land_base.persistence = PERSISTENCE;

    // Avoid shadows on seas by flattening these to sea leel
    let sea_clamped = Clamp::new(&land_base).set_lower_bound(SEA_LEVEL);

    let map = PlaneMapBuilder::new(&sea_clamped)
        .set_size(width, height)
        .set_x_bounds(-out_scale, out_scale)
        .set_y_bounds(-out_scale, out_scale)
        .build();

    let grad = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.00, [0, 0, 160, 255]) // deep sea
        .add_gradient_point(SEA_LEVEL, [0, 0, 164, 255]) // shallow sea
        .add_gradient_point(-0.02, [192, 192, 128, 255]) // beach
        .add_gradient_point(0.00, [0, 128, 0, 255]) //low forest
        .add_gradient_point(0.15, [0, 192, 0, 255]) //high forest
        .add_gradient_point(0.18, [172, 172, 172, 255]) //rock
        .add_gradient_point(0.28, [192, 192, 192, 255]) // high rock
        .add_gradient_point(0.29, [252, 252, 252, 255]) //snow
        .add_gradient_point(1.00, [255, 255, 255, 255]);

    let mut renderer = ImageRenderer::new();
    renderer.enable_light();
    let image = renderer
        .set_gradient(grad)
        .set_light_elevation(30.0)
        .set_light_azimuth(270.0)
        .set_light_contrast(20.0)
        .set_light_brightness(2.0)
        .render(&map);

    let mut data = Vec::new();
    for x in 0..width {
        for y in 0..height {
            let c = image.get_value(x, y);
            data.extend(c);
        }
    }
    data
}
