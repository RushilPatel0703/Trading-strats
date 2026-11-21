//! tcore: performance-critical primitives. Streaming (O(1) per update) so the same
//! code serves backtests and live trading.

/// Streaming exponential moving average.
#[derive(Debug, Clone)]
pub struct Ema {
    alpha: f64,
    value: Option<f64>,
}

impl Ema {
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "period must be > 0");
        Self { alpha: 2.0 / (period as f64 + 1.0), value: None }
    }

    #[inline]
    pub fn update(&mut self, x: f64) -> f64 {
        let v = match self.value {
            None => x,
            Some(prev) => prev + self.alpha * (x - prev),
        };
        self.value = Some(v);
        v
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ema_seeds_with_first_value_and_converges() {
        let mut e = Ema::new(3);
        assert_eq!(e.update(10.0), 10.0);
        for _ in 0..100 {
            e.update(20.0);
        }
        assert!((e.value().unwrap() - 20.0).abs() < 1e-9);
    }
}
