use polars_core::export::chrono::format::Numeric;
use polars_core::prelude::StaticArray;
use rand::*;
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::*;

pub mod data_utils;
pub mod models;

fn bernoulli_experiment(rng: &mut StdRng, p: f64, n: usize) -> u64 {
    let bern: Bernoulli = Bernoulli::new(p).unwrap();
    bern.sample_iter(rng)
        .take(n)
        .map(|x| x as u64)
        .sum()
}

fn random_walk(rng: &mut SmallRng, p: f64, n: usize) -> Vec<i64> {
    let bern: Bernoulli = Bernoulli::new(p).unwrap();
    bern.sample_iter(rng)
        .take(n)
        .scan(0, |state, x| {
            *state += 2 * (x as i64) - 1;
            Some(*state)
        })
        .collect()
}

fn random_walk_end(rng: &mut SmallRng, p: f64, n: usize, m: usize) -> Vec<i64> {
    let bern: Bernoulli = Bernoulli::new(p).unwrap();
    let r: Vec<bool> = bern.sample_iter(rng)
        .take(n * m)
        .collect();
    r.chunks(n)
        .map(|v| v.into_iter().fold(0, |acc, &x| acc + (2 * (x as i64) - 1)))
        .collect()
}

fn main() {
    let steps: usize = 200;
    let walkers: usize = 10;

    // init rng
    let mut rng = rand::rngs::SmallRng::from_entropy();

    // collect the results into a vector:
    let rw: Vec<Vec<i64>> = (0..walkers).map(|_| random_walk(&mut rng, 0.5, steps)).collect();

    let mut plot = plotly::Plot::new();
    for walk in rw {
        plot.add_trace(plotly::Scatter::new((0..steps).collect(), walk).mode(plotly::common::Mode::Lines));
    }
    let layout = plotly::Layout::new().x_axis(plotly::layout::Axis::new().show_grid(false))
                                              .y_axis(plotly::layout::Axis::new().show_grid(false));
    plot.set_layout(layout);
    plot.write_html("random_walks.html");

    let steps: usize = 1_000;
    let walkers: usize = 100_000;

    // collect the results into a vector:
    let rw_end: Vec<i64> = random_walk_end(&mut rng, 0.5, steps, walkers);

    // let mut plot = plotly::Plot::new();
    // plot.add_trace(plotly::Scatter::new((0..walkers).collect(), rw_end.to_vec()).mode(plotly::common::Mode::Markers));
    // let layout = plotly::Layout::new().x_axis(plotly::layout::Axis::new().show_grid(false))
    //                                           .y_axis(plotly::layout::Axis::new().show_grid(false));
    // plot.set_layout(layout);
    // plot.write_html("random_walks_end.html");

    let min: i64 = *rw_end.iter().min().unwrap();
    let max: i64 = *rw_end.iter().max().unwrap();
    let rw_end_dist: Vec<i64> = rw_end.iter().fold(vec![0; (max - min + 1) as usize], |mut state, x| 
        {
            state[(x - min) as usize] += 1;
            state
        });

    let mut plot = plotly::Plot::new();
    plot.add_trace(plotly::Scatter::new((min..=max).collect(), rw_end_dist).mode(plotly::common::Mode::Markers));
    let layout = plotly::Layout::new().x_axis(plotly::layout::Axis::new().show_grid(false))
                                              .y_axis(plotly::layout::Axis::new().show_grid(false));
    plot.set_layout(layout);
    plot.write_html("random_walks_end_dist.html");

    let mut plot = plotly::Plot::new();
    plot.add_trace(plotly::Histogram::new(rw_end.to_vec()).name("h"));
    let layout = plotly::Layout::new().x_axis(plotly::layout::Axis::new().show_grid(false))
                                              .y_axis(plotly::layout::Axis::new().show_grid(false));
    plot.set_layout(layout);
    plot.write_html("random_walks_end_hist.html");

    println!("{:?}", rw_end.iter().sum::<i64>() as f64 / rw_end.len() as f64);
}
