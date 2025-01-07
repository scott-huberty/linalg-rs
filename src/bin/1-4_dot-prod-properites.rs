use rand::Rng;
use nalgebra::DVector;


fn main() 
    -> Result<(), Box<dyn std::error::Error>> {
    const N: usize = 5;
    let mut rng = rand::thread_rng();
    let  a: Vec<i32> = (0..N).map(|_| rng.gen_range(0..10)).collect();
    let  b: Vec<i32> = (0..N).map(|_| rng.gen_range(0..10)).collect();
    let  c: Vec<i32> = (0..N).map(|_| rng.gen_range(0..10)).collect();
    println!("a: {:?}\nb: {:?}\nc: {:?}", a, b, c);

    let a = DVector::from_vec(a);
    let b = DVector::from_vec(b);
    let c = DVector::from_vec(c);


    // Distributive property
    // a.(b + c) = a.b + a.c
    let res1 = a.dot(&b) + a.dot(&c);
    // we need to clone b and c because the addition consumes them
    let res2 = a.dot(&(b.clone() + c.clone()));

    assert!(res1 == res2);
    println!("Distributive property holds. aT(b + c) == aTb + aTc: {} == {}", res1, res2);

    // Associative property
    // a.(b.c) != (a.b).c
    // This example is a little hacky because the dot product is undefined for a scalar and a vector
    // but in order to demonstrate that the associative property does not hold, we'll broadcast the scalar to a vector
    
    let bc = b.dot(&c);
    // bc is a scalar and We need to broadcast it to a vector of the same size as a for the dot product to be defined
    let bc_broadcasted = DVector::from_vec(vec![bc; N]); // e.g. [bc, bc, bc, bc, bc, bc, bc, bc, bc, bc]
    let res3 = a.dot(&bc_broadcasted);

    let ab = a.dot(&b); // ab is a scalar
    let ab_broadcasted = DVector::from_vec(vec![ab; N]); // broadcast ab to a vector of the same size as c
    let res4 = ab_broadcasted.dot(&c);
    // these are not equal because the dot product is not associative
    assert!(res3 != res4);
    println!("Associative property does not hold. aT(bTc) != (aTb)Tc: {} == {}", res3, res4);

    // Commutative property
    // compute a.b and b.a, test for equality
    let res5 = a.dot(&b);
    let res6 = b.dot(&a);
    assert!(res5 == res6);
    println!("Commutative property holds. aTb == bTa: {} == {}", res5, res6);

    Ok(())
}