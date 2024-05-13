use suoi_rwin::{Camera, Screen};
use suoi_types::{Matrix, Matrix4};

#[test]
fn projection_matrix() {
    let screen = Screen::new(800, 480);
    let cam = Camera::default();

    assert_eq!(Matrix4::identity(), &cam.inverse_projection_matrix(&screen) * &cam.projection_matrix(&screen));
}
