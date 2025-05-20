use rand::{SeedableRng, rngs::StdRng};
use raylib::prelude::*;

use std::collections::HashMap;

pub fn random() -> StdRng {
    StdRng::from_entropy()
}

pub fn load_models(rlib: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<&'static str, Model> {
    let mut models = HashMap::new();
    let model_srcs = HashMap::from([
        ("red_hoop", "./assets/hoop/red.obj"),
        ("blue_hoop", "./assets/hoop/blue.obj"),
        ("ball", "./assets/ball.glb"),
    ]);

    for (key, value) in model_srcs {
        models.insert(key, rlib.load_model(thread, value).unwrap());
    }

    let mut mut_color = |k: &str, c: Color| {
        models.get_mut(k).unwrap().materials_mut()[0].maps_mut()[0].color = c.into();
    };

    mut_color("red_hoop", Color::RED);
    mut_color("blue_hoop", Color::BLUE);

    models
}
