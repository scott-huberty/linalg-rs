use nalgebra::DVector;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // First the dot product the (almost) manual way
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0, -4, -3, 6, 5];
    let v3_manual = vec![
        v1[0] * v2[0],
        v1[1] * v2[1],
        v1[2] * v2[2],
        v1[3] * v2[3],
        v1[4] * v2[4],
    ].iter().sum();

    // Now the dot product using the dot method
    let v1 = DVector::from_vec(v1); // We'll take advantage of nalgebra's DVector
    let v2 = DVector::from_vec(v2);

    let v3 = v1.dot(&v2);

    assert_eq!(v3, v3_manual);

    // convert back to Vec for printing purposes
    println!("The dot product of {:?} and {:?} is {}", v1.data.as_vec(), v2.data.as_vec(), v3);
    Ok(())
}
