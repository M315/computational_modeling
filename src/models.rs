#[derive(Debug, Copy, Clone)]
pub struct ExpModel {
    λ: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct LogisticModel {
    p: f64,
    α: f64,
    total: f64,
}

impl ExpModel {
    pub fn new(λ: f64) -> Self {
        Self { λ }
    }

    /// Simulates n steps of the Exponential Model.
    ///
    /// # Examples
    ///
    /// ```
    /// let n: usize = 1000;
    /// 
    /// let e: ExpModel = ExpModel { λ: 1.01 };
    /// let infected: Vec<f64> = e.simulate(1.0, n);
    /// 
    /// let mut plot = plotly::Plot::new();
    /// plot.add_trace(plotly::Scatter::new((0..n).collect(), infected.iter().map(|&x| f64::log10(x)).collect()));
    /// plot.write_html("exp.html");
    /// ``` 
    pub fn simulate(&self, i0: f64, n: usize) -> Vec<f64> {
        (0..n).scan(i0, |acc, _| {*acc *= self.λ; Some(*acc)}).collect()
    }
}

impl LogisticModel {
    pub fn new(p: f64, α: f64, total: f64) -> Self {
        Self { p, α, total }
    }

    /// Simulates n steps of the Logistic Model.
    ///
    /// # Examples
    ///
    /// ```
    /// let n: usize = 1000;
    /// 
    /// let lg: LogisticModel = LogisticModel { p: 0.02, α: 0.01, total: 1000.0 };
    /// let infected: Vec<f64> = lg.simulate(1.0, n);
    ///
    /// let mut plot = plotly::Plot::new();
    /// plot.add_trace(plotly::Scatter::new((0..n).collect(), infected.iter().map(|&x| f64::log10(x)).collect()));
    /// plot.write_html("logist.html");
    /// ```
    fn β(&self, susceptible: f64) -> f64 {
        self.p * self.α * susceptible
    }

    pub fn simulate(&self, i0: f64, n: usize) -> Vec<f64> {
        (0..n).scan(i0, |acc, _| { *acc = *acc + Self::β(&self, self.total - *acc) * *acc; Some(*acc) }).collect()
    }
}

#[test]
fn simualte_exp() {
    let n: usize = 10;

    let e: ExpModel = ExpModel { λ: 1.01 };
    let infected: Vec<f64> = e.simulate(1.0, n);

    assert_eq!(infected, vec![1.01, 1.0201, 1.030301, 1.04060401, 1.0510100501, 1.061520150601, 1.0721353521070098, 1.08285670562808, 1.0936852726843609, 1.1046221254112045]);
}

#[test]
fn simualte_logistic() {
    let n: usize = 10;

    let lg: LogisticModel = LogisticModel { p: 0.02, α: 0.01, total: 1000.0 };
    let infected: Vec<f64> = lg.simulate(1.0, n);

    assert_eq!(infected, vec![1.1998, 1.439472095992, 1.7269520992073721, 2.071746046338255, 2.4852368292698026, 2.9810489147042514, 3.57548136711873, 4.288020827141153, 5.141947568046585, 6.165049156697394]);
}