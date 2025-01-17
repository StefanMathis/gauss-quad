/*!
Numerical integration using the Simpson rule.

A popular quadrature rule (also known as Kepler's barrel rule). It can be derived
in the simplest case by replacing the integrand with a parabola that has the same
function values at the end points a & b, as well as the Simpson m=(a+b)/2, which
results in the integral formula
S(f) = (b-a)/6 * [ f(a) + 4f(m) + f(b) ]

Dividing the interval [a,b] into N neighboring intervals of length h = (b-a)/N and
applying the Simpson rule to each subinterval, the integral is given by

S(f) = h/6 * [ f(a) + f(b) + 2*Sum_{k=1..N-1} f(x_k) + 4*Sum_{k=1..N} f( (x_{k-1} + x_k)/2 )]

with x_k = a + k*h.



```
use gauss_quad::{Simpson};
use approx::assert_abs_diff_eq;

use std::f64::consts::PI;

fn main() {

    let eps = 0.001;

    let n = 10;
    let quad = Simpson::init(n);

    // integrate some functions
    let integrate_euler = quad.integrate(0.0, 1.0, |x| x.exp());
    assert_abs_diff_eq!(integrate_euler, 1.0_f64.exp() - 1.0, epsilon = eps);

    let integrate_sin = quad.integrate(-PI, PI, |x| x.sin());
    assert_abs_diff_eq!(integrate_sin, 0.0, epsilon = eps);
}
```

!*/

#[derive(Debug, Clone)]
/// A Simpson rule quadrature scheme.
/// ```
/// # extern crate gauss_quad;
/// # use gauss_quad::Simpson;
/// # fn main() {
/// #
/// // initialize a Simpson rule with 100 subintervals
/// let quad: Simpson = Simpson::init(100);
///
/// // numerically integrate a function from -1.0 to 1.0 using the Simpson rule
/// let approx = quad.integrate(-1.0, 1.0, |x| x * x);
/// # }
/// ```
pub struct Simpson {
    /// The dimensionless Simpsons
    nodes: Vec<f64>,
}

impl Simpson {
    /// Initialize a new Simpson rule with `degree` being the number of intervals
    pub fn init(degree: usize) -> Self {
        assert!(degree >= 1, "Degree of Simpson rule needs to be >= 1");
        Self {
            nodes: Self::nodes(degree),
        }
    }

    /// Generate vector of indices for the subintervals
    fn nodes(degree: usize) -> Vec<f64> {
        let mut nodes = Vec::new();
        nodes.reserve(degree);
        for idx in 0..degree {
            nodes.push(idx as f64);
        }

        nodes
    }

    /// Integrate over the domain [a, b].
    pub fn integrate<F>(&self, a: f64, b: f64, integrand: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let n = self.nodes.len() as f64;

        let h = (b - a) / n;

        // first sum over the interval edges. Skips first index to sum 1..n-1
        let sum_over_interval_edges: f64 = self
            .nodes
            .iter()
            .skip(1)
            .map(|&node| integrand(a + node * h))
            .sum();

        // sum over the midpoints f( (x_{k-1} + x_k)/2 ), as node N is not included,
        // add it in the final result
        let sum_over_midpoints: f64 = self
            .nodes
            .iter()
            .skip(1)
            .map(|&node| integrand(a + (2.0 * node - 1.0) * h / 2.0))
            .sum();

        h / 6.0
            * (2.0 * sum_over_interval_edges
                + 4.0 * sum_over_midpoints
                + 4.0 * integrand(a + (2.0 * n - 1.0) * h / 2.0)
                + integrand(a)
                + integrand(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_simpson_integration() {
        let quad = Simpson::init(2);
        let integral = quad.integrate(0.0, 1.0, |x| x * x);
        approx::assert_abs_diff_eq!(integral, 1.0 / 3.0, epsilon = 0.0001);
    }
}
