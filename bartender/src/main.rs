use tailmaw::{
    base::Component,
    geometry::space::{Quaternion, Vector3},
};

fn main() {
    let mut engine = tailmaw::base::Engine::new();
    engine.load_default_assets();
    engine.retitle_window("Give me a drink, bartender");
    let cube = engine.create_entity();
    engine.insert_component(
        cube,
        "model",
        Component::Model {
            model: "test_cube".into(),
            visible: true,
            texture_override: None,
        },
    );
    engine.insert_component(
        cube,
        "camera",
        Component::Camera {
            cam: Vector3::new([5.0, 5.0, 5.0]),
            target: Vector3::fill(0.0),
            up: Vector3::new([0.0, 1.0, 0.0]),
            fov: 1.308,
        },
    );
    engine.insert_component(
        cube,
        "transformation",
        Component::Transformation {
            position: Vector3::fill(0.0),
            rotation: Quaternion {
                r: 1.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vector3::fill(1.0),
        },
    );
    let cube2 = engine.create_entity();
    engine.insert_component(
        cube2,
        "reference",
        Component::Reference { entity: cube, camera: true, transformation: false, model: true },
    );
    engine.insert_component(
        cube2,
        "transformation",
        Component::Transformation {
            position: Vector3::new([4.0, 0.0, 0.0]),
            rotation: Quaternion {
                r: 1.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vector3::fill(0.5),
        },
    );
    engine.create_system(|engine, entity| {
        let left_mouse_button_pressed = engine.left_mouse_button_pressed();
        if let Some(model) = engine.get_component_mut(entity, "model")
            && let Component::Model {
                texture_override, ..
            } = model
        {
            *texture_override = if left_mouse_button_pressed {
                Some("tailmaw/res/test.jxl".into())
            } else {
                None
            }
        }
    });
    while engine.update() {}
}
