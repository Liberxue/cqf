extern crate core;
extern crate plotters;

use clap::Parser;
use core::models::{BlackScholesModel, OptionParameters, OptionPricingModel};
use plotters::prelude::*;

#[derive(Parser)]
struct Opts {
    #[arg(short, long)]
    s: f64,
    #[arg(short, long)]
    k: f64,
    #[arg(short, long)]
    r: f64,
    #[arg(short = 'm', long)]
    sigma: f64,
    #[arg(short, long)]
    t: f64,
}
// example
fn main() {
    let opts: Opts = Opts::parse();

    let model = BlackScholesModel;
    let params = OptionParameters {
        s: opts.s,
        k: opts.k,
        r: opts.r,
        sigma: opts.sigma,
        t: opts.t,
    };

    let call_price = model.call_price(&params);
    let put_price = model.put_price(&params);

    println!("Call Price: {:.2}", call_price);
    println!("Put Price: {:.2}", put_price);

    generate_chart(&model, &params);
}

fn generate_chart(model: &BlackScholesModel, params: &OptionParameters) {
    let root_area = BitMapBackend::new("options_price_chart.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Option Prices", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(80.0..120.0, 0.0..20.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let call_series = (80..=120).map(|s| {
        let mut params = params.clone();
        params.s = s as f64;
        (s as f64, model.call_price(&params))
    });

    let put_series = (80..=120).map(|s| {
        let mut params = params.clone();
        params.s = s as f64;
        (s as f64, model.put_price(&params))
    });

    chart
        .draw_series(LineSeries::new(call_series, &RED))
        .unwrap()
        .label("Call Price")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(put_series, &BLUE))
        .unwrap()
        .label("Put Price")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}
