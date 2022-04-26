mod charts;
mod data;

use chrono::{Date, DateTime, Datelike /* for .year() */, TimeZone, Utc};
use csv;
use plotters::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use charts::*;
use data::*;

fn main() {
    let hours_worked = get_monthly_hours();
    println!("{:?}", hours_worked);
    let awards_earned = get_monthly_awards();
    println!("{:?}", awards_earned);
    let wardens_per_contest = get_wardens_per_contest();
    println!("{:?}", wardens_per_contest);

    // ctx.draw_series takes ownership
    create_dual_plot(hours_worked.clone(), awards_earned.clone());
    create_hourly_rate_plot(hours_worked.clone(), awards_earned.clone());
    create_warden_participation_plot(wardens_per_contest.clone());
}
