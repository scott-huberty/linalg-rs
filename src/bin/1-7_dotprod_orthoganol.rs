use nalgebra::Vector3;

fn main() {
    let v1: Vector3<f64> = Vector3::new(16.0, -2.0, 4.0);
    let v2: Vector3<f64> = Vector3::new(0.5, 2.0, -1.0);

    // algebraic
    let dp = v1.dot(&v2);
    let angle = v1.angle(&v2);
    let angle_deg = angle;
    assert_eq!(dp, 0.0);
    assert_eq!(angle, std::f64::consts::FRAC_PI_2);

    println!("Vectors: {:?}, {:?}", v1, v2);
    println!("Are Orthoganol because their dot product is zero: {}", dp);
    println!("And the Angle between them is {}°  degrees", angle_deg);
    println!("###############################################");

    // vectors with acute angle
    let v1: Vector3<f64> = Vector3::new(1.0, 0.0, 0.0);
    let v2: Vector3<f64> = Vector3::new(1.0, 1.0, 0.0);
    let dp = v1.dot(&v2);
    let angle = v1.angle(&v2).to_degrees();
    assert!(dp > 0.0);
    assert!(angle < 90.0);

    println!("Vectors: {:?}, {:?}", v1, v2);
    println!("Vectors with a positive dot product have an acute angle: {}, {:.2}°", dp, angle);
    println!("###############################################");

    // vectors with obtuse angle
    let v1: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0);
    let v2: Vector3<f64> = Vector3::new(-1.0, -1.0, -1.0);

    let dp = v1.dot(&v2);
    let angle = v1.angle(&v2).to_degrees();
    assert!(dp < 0.0);
    assert!(angle > 90.0);

    println!("Vectors: {:?}, {:?}", v1, v2);
    println!("Vectors with a negative dot product have an obtuse angle: {}, {:.2}°", dp, angle);
}