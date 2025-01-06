use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let v2 = [3, -2]; // 2D Vector
    let v3 = [4, -3, 2]; // 3D Vector
    println!("2D Vector: {:?}", v2);
    println!("3D Vector: {:?}", v3);

    // Plot the 2D vector
    println!("Plotting a 2D Geometric Representation of a Vector to images/2D_vector.png");
    let root = BitMapBackend::new("images/2D_vector.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?; // Fill the background with white
    let mut chart = ChartBuilder::on(&root)
        .caption("Geometric Representation of a 2D Vector", ("sans-serif", 25).into_font()) // Set the title of the plot
        .margin(5) // Set the margin of the plot
        .x_label_area_size(30) // Add space for the x label
        .y_label_area_size(30) // Add space for the y label
        .build_cartesian_2d(-5..5, -5..5)?; // Set the range of the x and y axis
    
    chart.configure_mesh().draw()?; // Draw the mesh

    chart.draw_series(LineSeries::new(
        [(0, 0), (v2[0], v2[1])].iter().copied(),
        &BLUE,
    ))?; // Draw the vector

    // draw the axes from the origin
    chart.draw_series(LineSeries::new(
        [(0, 0), (0, 5), // Vertical Line (Top)
         (0, 0), (5, 0), // Horizontal Line (Right)
         (0, 0), (0, -5), // Vertical Line (Bottom)
         (0, 0), (-5, 0) // Horizontal Line (Left)
        ].iter().copied(),
        &BLACK,
    ))?;

    root.present()?; // Save the plot

    // Plot the 3D vector
    println!("Plotting a 3D Geometric Representation of a Vector to images/3D_vector.png");
    let root = BitMapBackend::new("images/3D_vector.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?; // Fill the background with white
    let mut chart = ChartBuilder::on(&root)
        .caption("Geometric Representation of a 3D Vector", ("sans-serif", 25).into_font()) // Set the title of the plot
        .margin(5) // Set the margin of the plot
        .x_label_area_size(30) // Add space for the x label
        .y_label_area_size(30) // Add space for the y label
        .build_cartesian_3d(-5..5, -5..5, -5..5)?; // Set the range of the x, y, and z axis

    chart.configure_axes().draw()?; // Draw the axes
    
    chart.draw_series(LineSeries::new(
        [(0, 0, 0), (v3[0], v3[1], v3[2])].iter().copied(),
        &BLUE,
    ))?; // Draw the vector

    // draw the (3D!) axes from the origin
    chart.draw_series(LineSeries::new(
        [(0, 0, 0), (0, 0, 5), // Vertical Line (Top)
         (0, 0, 0), (0, 5, 0), // Horizontal Line (Right)
         (0, 0, 0), (0, 0, -5), // Vertical Line (Bottom)
         (0, 0, 0), (0, -5, 0), // Horizontal Line (Left)
         (0, 0, 0), (5, 0, 0), // Depth Line (Front)
         (0, 0, 0), (-5, 0, 0) // Depth Line (Back)
        ].iter().copied(),
        &BLACK,
    ))?;

    root.present()?; // Save the plot

    Ok(()) // Return Ok

}