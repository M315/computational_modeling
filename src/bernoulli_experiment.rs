use rand::*;
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::ThreadRng;

fn bernoulli_experiment(rng: &mut ThreadRng, p: f64, n: usize) -> u64
{
    let bern: Bernoulli = Bernoulli::new(p).unwrap();
    bern.sample_iter(rng)
        .take(n)
        .map(|x| x as u64)
        .sum()
}

fn main() {
    let n: usize = 200;

    // init rng
    let mut rng = rand::thread_rng();

    // collect the results into a vector:
    let v: Vec<f64> = (0..n).map(|_| rng.gen::<f64>()).collect();

    let mut plot = plotly::Plot::new();
    plot.add_trace(plotly::Scatter::new((0..n).collect(), v).mode(plotly::common::Mode::Markers));
    let layout = plotly::Layout::new().x_axis(plotly::layout::Axis::new().show_grid(false))
                                              .y_axis(plotly::layout::Axis::new().show_grid(false));
    plot.set_layout(layout);
    plot.write_html("random_scatter.html");

    // Bernoulli trials
    let n: usize = 20;
    let p: f64 = 0.25;
    let num_experiments: usize = 1000;
    let results: Vec<u64> = (0..num_experiments).map(|_| bernoulli_experiment(&mut rng, p, n)).collect();
    print!("{:?}", results);
}