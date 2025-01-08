use nalgebra::DVector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_vector: Vec<f64> = vec![3.0, 4.0];
    // the length of a vector in can be found via the pythagorean theorem
    // this is sensible in R2 (two dimensions) but the principle extends to higher dimensions
    let x: f64 = my_vector[0];
    let y: f64 = my_vector[1];
    let length = (x.powf(2.0) + y.powf(2.0)).sqrt();

    println!("Vector {:?} length (Pythagorean): {}", my_vector, length);

    let my_vector = DVector::from_vec(my_vector);
    let length = my_vector.dot(&my_vector).sqrt();
    println!("Vector {:?} length (dot product): {}", my_vector.data.as_vec(), length);

    // We can also take the norm of the vector. The norm IS the length of the vector
    let length = my_vector.norm();
    println!("Vector  {:?} length (norm): {}", my_vector.data.as_vec(), length);
    Ok(())
}