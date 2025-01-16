use plotters::prelude::*;

use nalgebra::Vector2;

fn plot_vectors(vectors: Vec<Vector2<f64>>) -> Result<(), Box<dyn std::error::Error>>
{
    let root = BitMapBackend::new("images/relative_vector_angles.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap(); // Fill the background with white
    let mut chart = ChartBuilder::on(&root)
        .caption("Relative Vector Angles", ("sans-serif", 25).into_font()) // Set the title of the plot
        .margin(5) // Set the margin of the plot
        .x_label_area_size(30) // Add space for the x label
        .y_label_area_size(30) // Add space for the y label
        .build_cartesian_2d(-5.0..5.0, -5.0..5.0).unwrap(); // Set the range of the x and y axis

    chart.configure_mesh().draw().unwrap(); // Draw the mesh

    for vector in vectors {
        chart.draw_series(LineSeries::new(
            [(0.0, 0.0), (vector.x, vector.y)].iter().copied(),
            &BLUE,
        )).unwrap(); // Draw the vector
    }

    // draw the axes from the origin
    chart.draw_series(LineSeries::new(
        [(0.0, 0.0), (0.0, 5.0), // Vertical Line (Top)
         (0.0, 0.0), (5.0, 0.0), // Horizontal Line (Right)
         (0.0, 0.0), (0.0, -5.0), // Vertical Line (Bottom)
         (0.0, 0.0), (-5.0, 0.0) // Horizontal Line (Left)
        ].iter().copied(),
        &BLACK,
    )).unwrap();

    root.present().unwrap(); // Save the plot
    Ok(())
}

fn main() {

    let a: Vector2<f64> = nalgebra::Vector2::new(1.0, -2.0);
    let b: Vector2<f64> = nalgebra::Vector2::new(2.0, 3.0);
    let c: Vector2<f64> = nalgebra::Vector2::new(0.0, 2.0);

    let dot_ab: f64 = a.dot(&b);
    let dot_bc: f64 = b.dot(&c);
    let dot_ac: f64 = a.dot(&c);

    for dotprod in vec![dot_ab, dot_bc, dot_ac] {
        if dotprod < 0.0 {
            println!("The angle between the vectors is obtuse");
        } else if dotprod == 0.0 {
            println!("The vectors are orthogonal");
        } else {
            println!("The angle between the vectors is acute");
        }
    }
    let _ = plot_vectors(vec![a, b]);
}