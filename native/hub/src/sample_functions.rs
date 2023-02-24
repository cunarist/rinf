use crate::api::DotAddress;
use crate::bridge::update_viewmodel_with_bytes;
use crate::bridge::update_viewmodel_with_json;
use crate::model;
use serde_json::json;

pub fn calculate_something(json_value: serde_json::Value) {
    let _ = json_value;

    let mut value = model::COUNT.write().unwrap();
    *value = sample_crate::add_seven(*value);
    println!("{:}", *value);
    let json_value = json!({ "value": *value });

    update_viewmodel_with_json(DotAddress::from("someItemCategory.count"), json_value)
}

pub fn start_drawing_mandelbrot() {
    std::thread::spawn(|| {
        let mut scale: f64 = 1.0;
        loop {
            let mandelbrot = sample_crate::mandelbrot(
                sample_crate::Size {
                    width: 64,
                    height: 64,
                },
                sample_crate::Point {
                    x: 0.360,
                    y: -0.641,
                },
                scale,
                4,
            )
            .unwrap();
            update_viewmodel_with_bytes(
                DotAddress::from("someItemCategory.mandelbrot"),
                mandelbrot,
            );
            std::thread::sleep(std::time::Duration::from_millis(20));
            scale *= 0.95;
            if scale < 1e-9 {
                scale = 1.0
            };
        }
    });
}
