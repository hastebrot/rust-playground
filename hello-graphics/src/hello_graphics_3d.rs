extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::camera::FirstPerson;
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Point2, Point3, UnitQuaternion, Vector3};

fn main() {
    let mut window = Window::new("cube");
    let mut cube = window.add_cube(1.0, 1.0, 1.0);

    let eye = Point3::new(2.0, 2.0, 2.0);
    let at = Point3::origin();
    let mut camera = FirstPerson::new(eye, at);

    cube.set_color(1.0, 0.0, 0.0);
    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render_with_camera(&mut camera) {
        cube.prepend_to_local_rotation(&rot);
        // window.draw_planar_line(
        //     &Point2::new(0.0, 0.0),
        //     &Point2::new(200.0, 200.0),
        //     &Point3::new(1.0, 1.0, 1.0));
    }
}
