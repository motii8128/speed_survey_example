extern crate nalgebra as na;
use plotlib::repr::Plot;
use plotlib::page::Page;
use plotlib::view::ContinuousView;
use plotlib::style::PointStyle;
use std::time::Instant;

fn main() {
    let set_times = 100;
    let survey_times = 100;

    let mut point1 = vec![(0.0, 0.0)];
    let mut point2 = vec![(0.0, 0.0)];

    for j in 0..survey_times
    {
        let mut total_time1 = 0.0;
        let mut total_time2 = 0.0;
        let mut result_time1 = 0.0;
        let mut result_time2 = 0.0;
        for i in 0..set_times
        {
            let start1 = Instant::now();

                let a = na::Matrix2x3::new(
                    1, 1, 1,
                    2, 2, 2,
                );

                let transpose_a = a.transpose();

            let end1 = start1.elapsed().as_nanos() as f64;

            let start2 = Instant::now();

                let b = na::Matrix2x3::new(
                    1, 1, 1,
                    2, 2, 2,
                );

                let transpose_b = na::Matrix3x2::new(
                    b.m11, b.m21,
                    b.m12, b.m22,
                    b.m13, b.m23,
                );

            let end2 = start2.elapsed().as_nanos() as f64;

            total_time1 += end1;
            total_time2 += end2;
        }

        result_time1 = total_time1 / set_times as f64;
        result_time2 = total_time2 / set_times as f64;
        point1.push((j as f64, result_time1));
        point2.push((j as f64, result_time2));
    }

    let plot1 = Plot::new(point1).point_style(
        PointStyle::new()
            .colour("#DD3355")
            .size(3.0),
    );

    let plot2 = Plot::new(point2).point_style(
        PointStyle::new()
            .colour("#0000ff")
            .size(3.0),
    );

    let view = ContinuousView::new()
        .add(plot1)
        .add(plot2)
        .x_range(0.0, survey_times as f64)
        .y_range(0.0, 1000.0)
        .x_label("times")
        .y_label("secs(nano)");

    Page::single(&view).save("./img/result.svg").unwrap();
}
