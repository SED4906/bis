use tailmaw::{
    base::Component,
    geometry::space::{Quaternion, Vector3},
};

fn main() {
    let mut engine = tailmaw::base::Engine::new();
    engine.load_default_assets();
    engine.retitle_window("Give me a drink, bartender");
    let camera = engine.create_entity();
    engine.insert_component(
        camera,
        "camera",
        Component::Camera {
            cam: Vector3::new([5.0, 5.0, 5.0]),
            target: Vector3::fill(0.0),
            up: Vector3::new([0.0, 1.0, 0.0]),
            fov: 1.308,
        },
    );
    let cube = engine.create_entity();
    engine.insert_component(
        cube,
        "model",
        Component::Model {
            model: "quad".into(),
            visible: true,
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
    engine.insert_component(
        cube,
        "camera_reference",
        Component::Reference {
            entity: camera,
            camera: true,
            transformation: false,
            model: false,
            texture_override: false,
        },
    );
    let cube2 = engine.create_entity();
    engine.insert_component(
        cube2,
        "camera_reference",
        Component::Reference {
            entity: camera,
            camera: true,
            transformation: false,
            model: false,
            texture_override: false,
        },
    );
    engine.insert_component(
        cube2,
        "model_reference",
        Component::Reference {
            entity: cube,
            camera: false,
            transformation: false,
            model: true,
            texture_override: false,
        },
    );
    engine.insert_component(
        cube2,
        "transformation",
        Component::Transformation {
            position: Vector3::new([4.0, 0.0, 0.0]),
            rotation: Quaternion::axis_angle(Vector3::new([1.0, 0.0, 1.0]), 1.0),
            scale: Vector3::fill(0.5),
        },
    );
    engine.insert_component(
        cube2,
        "texture_override",
        Component::TextureOverride {
            texture: "tailmaw/res/bis_logo.jxl".into(),
        },
    );
    while engine.update() {
        let timer = engine.get_timer();
        if let Some(Component::Transformation { rotation, .. }) =
            engine.get_component_mut(cube2, "transformation")
        {
            *rotation = Quaternion::axis_angle(Vector3::new([1.0, 0.0, 0.0]), timer);
        }
    }
}
