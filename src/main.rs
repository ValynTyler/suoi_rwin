use suoi_types::{Matrix, Matrix4, Quaternion, Rad, Transform, Vector, Vector3};

fn main() {
    let q = Quaternion::axis_angle(Vector3::new(0.3, 0.2, -0.5).unit(), Rad(1.234));

    let mut t = Transform::default();
    t.set_position(Vector3::new(0.3, 1.234, 96.240));
    // t.set_rotation(q);

    let u = t.up();
    let r = t.right();
    let f = t.forward();

    let t = Vector3 {
        x: -r.dot(t.position()),
        y: -u.dot(t.position()),
        z: -f.dot(t.position()),
    };

    let mat = Matrix4::look_at(f, u, r, t).transpose();

    println!("{}", &mat * &mat.inverse());
}
