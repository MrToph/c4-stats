use chrono::{Date, DateTime, Datelike /* for .year() */, TimeZone, Utc};
use csv;
use plotters::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Clockify {
    #[serde(alias = "Start Date")]
    start_date: String,
    #[serde(alias = "Start Time")]
    start_time: String,
    #[serde(alias = "End Date")]
    end_date: String,
    #[serde(alias = "End Time")]
    end_time: String,
    #[serde(alias = "Description")]
    description: String,
}

struct TimeEntry {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl From<Clockify> for TimeEntry {
    fn from(item: Clockify) -> Self {
        let start = Utc
            .datetime_from_str(
                &format!("{} {}", item.start_date, item.start_time),
                "%d.%m.%Y %H:%M",
            )
            .unwrap();
        let end = Utc
            .datetime_from_str(
                &format!("{} {}", item.end_date, item.end_time),
                "%d.%m.%Y %H:%M",
            )
            .unwrap();

        TimeEntry { start, end }
    }
}

fn get_monthly_hours() -> Vec<(Date<Utc>, f64)> {
    let mut data = HashMap::<Date<Utc>, f64>::new();

    let mut rdr = csv::Reader::from_path("stats/raw/clockify.csv").unwrap();
    rdr.deserialize()
        .filter(|r: &Result<Clockify, csv::Error>| !r.is_err())
        .map(|r| r.unwrap())
        .filter(|c| c.description == "C4" || c.description == "Code423n4")
        .for_each(|c| {
            let t: TimeEntry = c.into();
            let duration_hours: f64 = (t.end.timestamp() - t.start.timestamp()) as f64 / 3600.0;
            // we assume a time entry never spans more than 1 month, important for splitting
            assert!(duration_hours > 0.0 && duration_hours < 28.0 * 24.0);

            let month_key_start = Utc.ymd(t.start.date().year(), t.start.date().month(), 1);
            let month_key_end = Utc.ymd(t.end.date().year(), t.end.date().month(), 1);
            if month_key_start == month_key_end {
                *data.entry(month_key_start).or_insert(0.0) += duration_hours;
            } else {
                // split on month, given the < 1 month assumption above
                let split_time = month_key_end.and_hms_milli(0, 0, 0, 0);
                // time from start .. end of month
                let duration_hours: f64 =
                    (split_time.timestamp() - t.start.timestamp()) as f64 / 3600.0;
                *data.entry(month_key_start).or_insert(0.0) += duration_hours;

                // time from end of start month .. end time
                let duration_hours: f64 =
                    (t.end.timestamp() - split_time.timestamp()) as f64 / 3600.0;
                *data.entry(month_key_end).or_insert(0.0) += duration_hours;
            }
        });

    let mut v = Vec::from_iter(data.into_iter());
    v.sort_by_key(|&(date, _)| date);
    v
}

#[derive(Debug, Deserialize)]
struct ContestRaw {
    #[serde(alias = "contestid")]
    id: String,
    // 2021-02-17T00:00:00.000
    start_time: String,
    end_time: String,
}
struct ContestDuration {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl From<ContestRaw> for ContestDuration {
    fn from(item: ContestRaw) -> Self {
        let start = item.start_time.parse::<DateTime<Utc>>().unwrap();
        let end = item.end_time.parse::<DateTime<Utc>>().unwrap();

        ContestDuration { start, end }
    }
}

#[derive(Debug, Deserialize)]
struct Finding {
    contest: String,
    handle: String,
    #[serde(alias = "awardUSD")]
    award_usd: f64,
}

fn get_monthly_awards() -> Vec<(Date<Utc>, f64)> {
    // https://raw.githubusercontent.com/code-423n4/code423n4.com/main/_data/contests/contests.csv
    // https://raw.githubusercontent.com/code-423n4/code423n4.com/main/_data/findings/findings.csv
    let mut contests = HashMap::<String, ContestDuration>::new();

    let mut rdr = csv::Reader::from_path("stats/raw/contests.csv").unwrap();
    rdr.deserialize()
        .filter(|r: &Result<ContestRaw, csv::Error>| !r.is_err())
        .map(|r| r.unwrap())
        .for_each(|c| {
            contests.insert(c.id.clone(), c.into());
        });

    let mut data = HashMap::<Date<Utc>, f64>::new();
    let mut rdr = csv::Reader::from_path("stats/raw/findings.csv").unwrap();
    rdr.deserialize()
        .filter(|r: &Result<Finding, csv::Error>| !r.is_err())
        .map(|r| r.unwrap())
        .filter(|f| f.handle == "cmichel")
        .for_each(|f| {
            let t = contests.get(&f.contest).unwrap();
            let duration_total: f64 = (t.end.timestamp() - t.start.timestamp()) as f64;
            // we assume a contest never spans more than 1 month, important for splitting
            assert!(duration_total > 0.0 && duration_total < 28.0 * 24.0 * 60.0 * 60.0);

            let month_key_start = Utc.ymd(t.start.date().year(), t.start.date().month(), 1);
            let month_key_end = Utc.ymd(t.end.date().year(), t.end.date().month(), 1);
            if month_key_start == month_key_end {
                *data.entry(month_key_start).or_insert(0.0) += f.award_usd;
            } else {
                // split on month, given the < 1 month assumption above
                let split_time = month_key_end.and_hms_milli(0, 0, 0, 0);
                // time from start .. end of month
                let duration: f64 = (split_time.timestamp() - t.start.timestamp()) as f64;
                *data.entry(month_key_start).or_insert(0.0) +=
                    f.award_usd * duration / duration_total;

                // time from end of start month .. end time
                let duration: f64 = (t.end.timestamp() - split_time.timestamp()) as f64;
                *data.entry(month_key_end).or_insert(0.0) +=
                    f.award_usd * duration / duration_total;
            }
        });

    let mut v = Vec::from_iter(data.into_iter());
    v.sort_by_key(|&(date, _)| date);
    v
}

fn create_dual_plot(hours_worked: Vec<(Date<Utc>, f64)>, awards_earned: Vec<(Date<Utc>, f64)>) {
    let root_area =
        BitMapBackend::new("plots/work_awards_dual.png", (1280, 768)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let min_date = 0;
    let max_date = hours_worked.len() - 1;
    // does not work, cannot sort f64s ...
    // let max_val = *hours_worked.iter().map(|(_, minutes)| minutes).max().unwrap();
    let max_hours = hours_worked
        .iter()
        .map(|(_, minutes)| *minutes)
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
        // We can also change the format of the label text
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

fn create_hourly_rate_plot(
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
        // We can also change the format of the label text
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

fn main() {
    let hours_worked = get_monthly_hours();
    println!("{:?}", hours_worked);
    let awards_earned = get_monthly_awards();
    println!("{:?}", awards_earned);

    // ctx.draw_series takes ownership
    create_dual_plot(hours_worked.clone(), awards_earned.clone());
    create_hourly_rate_plot(hours_worked.clone(), awards_earned.clone());
}
