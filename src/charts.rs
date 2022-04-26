use chrono::{Date, DateTime, Utc};
use plotters::prelude::*;

pub fn create_dual_plot(hours_worked: Vec<(Date<Utc>, f64)>, awards_earned: Vec<(Date<Utc>, f64)>) {
    let root_area =
        BitMapBackend::new("plots/work_awards_dual.png", (1280, 768)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let min_date = 0;
    let max_date = hours_worked.len() - 1;
    // does not work, cannot sort f64s ...
    // let max_val = *hours_worked.iter().map(|(_, minutes)| minutes).max().unwrap();
    let max_hours = hours_worked
        .iter()
        .map(|(_, hours)| *hours)
        .fold(f64::NEG_INFINITY, f64::max);
    let max_awards = awards_earned
        .iter()
        .map(|(_, awards)| *awards)
        .fold(f64::NEG_INFINITY, f64::max);

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Right, 70)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Hours worked / $ earned (per month)", ("sans-serif", 40))
        .build_cartesian_2d(min_date..max_date, 0.0..max_hours)
        .unwrap()
        .set_secondary_coord(min_date..max_date, 0.0..max_awards);

    ctx.configure_mesh()
        .x_labels(hours_worked.len())
        .x_label_formatter(&|d| hours_worked[*d].0.format("%b-%y").to_string())
        .y_desc("Hours worked")
        .draw()
        .unwrap();

    ctx.configure_secondary_axes()
        .y_desc("$ earned")
        .draw()
        .unwrap();

    ctx.draw_series(LineSeries::new(
        hours_worked
            .iter()
            .enumerate()
            .map(|(index, &(_x, y))| (index, y)),
        &BLUE,
    ))
    .unwrap()
    .label("hours worked")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.draw_secondary_series(LineSeries::new(
        awards_earned
            .iter()
            .enumerate()
            .map(|(index, &(_x, y))| (index, y)),
        &RED,
    ))
    .unwrap()
    .label("$ earned")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.configure_series_labels()
        .background_style(&RGBColor(255, 255, 255))
        .draw()
        .unwrap();
}

pub fn create_hourly_rate_plot(
    hours_worked: Vec<(Date<Utc>, f64)>,
    awards_earned: Vec<(Date<Utc>, f64)>,
) {
    let hourly_rate: Vec<(Date<Utc>, f64)> = hours_worked
        .iter()
        .zip(awards_earned.iter())
        .map(|(&(date, hours), &(_date, awards))| (date, awards / hours))
        .collect();
    println!("{:?}", hourly_rate);
    let root_area = BitMapBackend::new("plots/hourly_rate.png", (1280, 768)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let min_date = 0;
    let max_date = hourly_rate.len() - 1;
    let max_hours = hourly_rate
        .iter()
        .map(|(_, val)| *val)
        .fold(f64::NEG_INFINITY, f64::max);

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Hourly rate (per month)", ("sans-serif", 40))
        .build_cartesian_2d(min_date..max_date, 0.0..max_hours)
        .unwrap();

    ctx.configure_mesh()
        .x_labels(hours_worked.len())
        .x_label_formatter(&|d| hours_worked[*d].0.format("%b-%y").to_string())
        .y_desc("Hourly rate $/h")
        .draw()
        .unwrap();

    ctx.draw_series(LineSeries::new(
        hourly_rate
            .iter()
            .enumerate()
            .map(|(index, &(_x, y))| (index, y)),
        &BLUE,
    ))
    .unwrap()
    .label("hourly rate $/h");
}

pub fn create_warden_participation_plot(wardens_per_contest: Vec<(DateTime<Utc>, u64)>) {
    let root_area =
        BitMapBackend::new("plots/wardens_per_contest.png", (1280, 768)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let min_date = *wardens_per_contest
        .iter()
        .map(|(date, _)| date)
        .min()
        .unwrap();
    let max_date = *wardens_per_contest
        .iter()
        .map(|(date, _)| date)
        .max()
        .unwrap();
    let max_wardens = *wardens_per_contest.iter().map(|(_, v)| v).max().unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Wardens per contest", ("sans-serif", 40))
        .build_cartesian_2d(min_date..max_date, 0..max_wardens)
        .unwrap();

    ctx.configure_mesh()
        .x_labels(10)
        .x_label_formatter(&|d| d.format("%d-%b-%y").to_string())
        .y_desc("wardens / contest")
        .draw()
        .unwrap();

    ctx.draw_series(LineSeries::new(wardens_per_contest, &BLUE))
        .unwrap()
        .label("wardens / contest");
}
