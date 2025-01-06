use nalgebra::Vector2; // You can use other types like Vector2, Vector4, etc.
use plotters::prelude::*;

fn main()
	-> Result<(), Box<dyn std::error::Error>> 
{
    // This Function demonstrates how to multiply a vector by a scalar
    // A scalar is a single number.

    let v1 = Vector2::new(2.0, -1.0);
    let l = 2.0;

    // First the manual way of multiplying a vector by a scalar
    let v1m_manual = Vector2::new(v1.x * l, v1.y * l);
    // Now the operator way
    let v1m = &v1 * l;
    assert!(v1m == v1m_manual);

    println!("{:?} * {} = {:?}", v1, l, v1m);
    println!("See for yourself at images/1-2_vector-multiplication.png");

    // Let's plot the vectors
    let root = BitMapBackend::new("images/1-2_vector-multiplication.png", (800, 600)).into_drawing_area();

    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Geometric Representation of Multiplying a Vector by a Scalar", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-5.0..5.0, -5.0..5.0)?;

    chart.configure_mesh().draw()?;

    // Vector v1
    chart.draw_series(LineSeries::new(
        [
            (0.0, 0.0), // X, Y start i.e. start at the origin
            (v1.x, v1.y), // X, Y end i.e. -3, 1
            ].iter().copied(),
        &BLUE,
    ))?.label("v1").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));


    // Let's create a dashed line for the multiplied vector
    let dashedline_style = ShapeStyle {
        color: RED.to_rgba(),
        filled: true,
        stroke_width: 2,
    };
    // Vector v1 * l
    chart.draw_series(DashedLineSeries::new(
        [
            (0.0, 0.0), // X, Y start i.e. start at the origin
            (v1m.x, v1m.y), // X, Y end i.e. -3, 1
            ].iter().copied(),
            5,
            10,
            dashedline_style,
    ))?.label("v1 * 2").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // add the legend
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?; // Save the plot

    Ok(())
}