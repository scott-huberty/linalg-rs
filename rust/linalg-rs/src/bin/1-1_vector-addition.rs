use ndarray::Array;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This Function demonstrates how to add two vectors

    // We'll use the ndarray crate to represent vectors
    // so that we can use the `+` operator to add them
    // two vectors in R2
    let v1 = Array::from_vec(vec![3, -1]);
    let v2 = Array::from_vec(vec![2, 4]);

    // add the vectors the manual way
    let v3_manual = Array::from_vec(vec![v1[0] + v2[0], v1[1] + v2[1]]);
    // add the vectors the operator way
    let v3 = &v1 + &v2;
    assert!(v3 == v3_manual);

    println!("{:?} + {:?} = {:?}", v1.to_vec(), v2.to_vec(), v3.to_vec());
    println!("(See for yourself at images/1-1_vector-addition.png)");

    // Plot the vectors
    let root = BitMapBackend::new("images/1-1_vector-addition.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Geometric Representation of adding two Vectors", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-5..5, -5..5)?;

    chart.configure_mesh().draw()?;

    // Vector v1
    chart.draw_series(LineSeries::new(
        [
            (0, 0), // X, Y start i.e. start at the origin
            (v1[0], v1[1]), // X, Y end i.e. -3, 1
            ].iter().copied(),
        &BLUE,
    ))?.label("v1").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Plot v2 but starting at the tip of v1
    let vector_2_origin = (0 + v1[0], 0 + v1[1]);
    let vector_2_end = (v2[0] + v1[0], v2[1] + v1[1]);
    
    chart.draw_series(LineSeries::new(
        [vector_2_origin, vector_2_end].iter().copied(),
        &RED,
    ))?.label("v2").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.draw_series(LineSeries::new(
        [(0, 0), (v3[0], v3[1])].iter().copied(),
        &BLACK,
    ))?.label("v1 + v2").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    // add the legend
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?; // Save the plot

    Ok(())
}