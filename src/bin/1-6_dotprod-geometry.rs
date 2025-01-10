// This example demonstrates the geometric interpretation of the dot product
// and that it is equivalent to the algebraic interpretation

use nalgebra::Vector3;

fn main() {
    let v1: Vector3<f64> = Vector3::new(3.0, 4.0, 0.0);
    let v2: Vector3<f64> = Vector3::new(0.0, -3.0, -3.0);

    // algebraic
    let dp_a = v1.dot(&v2);

    // geometric
    // the angle between the two vectors scaled by the product of the lengths of the vectors
    // |v1| * |v2| * cos(theta)
    let dp_g = v1.norm() * v2.norm() * v1.angle(&v2).cos();
    println!("Dot product (algebraic): {}", dp_a);
    println!("Dot product (geometric): {}", dp_g);
}