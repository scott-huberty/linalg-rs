// the magnitude of the dot product of two vectors is less than or equal to the product of their magnitudes
use rand::Rng;
use nalgebra::Vector5;

fn main() {
    // Cauchy Schwarz Inequality for vectors
    let mut rng = rand::thread_rng();
    let n = 5;
    let a: Vec<f64> = (0..n).map(|_| rng.gen_range(-10.0..10.0)).collect();
    let b: Vec<f64> = (0..n).map(|_| rng.gen_range(-10.0..10.0)).collect();
    // C will be one random number times the elements in a
    let randn: f64 = rng.gen_range(-10.0..10.0);
    let c: Vec<f64> = a.iter().map(|x| x * randn).collect();
    println!("a: {:?}\nb: {:?}\nc: {:?}", a, b, c);

    let a: Vector5<f64> = Vector5::from_vec(a);
    let b: Vector5<f64> = Vector5::from_vec(b);
    let c: Vector5<f64> = Vector5::from_vec(c);

    let aTb = a.dot(&b);
    let aTc = a.dot(&c);

    // demonstrate the (in)equalities
    let a_mag = a.norm();
    let b_mag = b.norm();
    let c_mag = c.norm();

    let inequality1 = aTb.abs() <= a_mag * b_mag;
    let inequality2 = aTc.abs() <= a_mag * c_mag;
    println!("If two vectors form a linear independent set, the magnitude of their dot product is less than the product of their magnitudes");
    println!("|aTb| <= |a||b|: {} <= {}*{} = {}", aTb.abs(), a_mag, b_mag, a_mag * b_mag);
    println!("If two vectors form a dependent set (rank deficient), the magnitude of their dot product is equal to the product of their magnitudes");
    println!("|aTc| <= |a||c|: {} <= {}*{} = {}", aTc.abs(), a_mag, c_mag, a_mag * c_mag);

}