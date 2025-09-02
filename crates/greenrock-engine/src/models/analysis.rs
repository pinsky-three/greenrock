use ta::{
    Next,
    indicators::{
        AverageTrueRange, ExponentialMovingAverage, MovingAverageConvergenceDivergence,
        MovingAverageConvergenceDivergenceOutput, RelativeStrengthIndex,
    },
};

use crate::models::timeseries::Candle;

/// Example usage of the SuperTrend indicators:
///
/// ```rust
/// use crate::models::analysis::TechnicalAnalysis;
///
/// // Assuming you have a Vec<Candle> called `candles`
///
/// // Basic SuperTrend with factor 2.0
/// let supertrend = candles.supertrend(10, 2.0);
///
/// // SuperTrend clustering (like Pine Script strategy)
/// let best_supertrend = candles.best_supertrend_from_cluster(
///     10,    // ATR period
///     1.0,   // min factor
///     5.0,   // max factor  
///     0.5,   // step
///     10.0,  // performance alpha
///     "Best" // cluster type: "Best", "Average", "Worst"
/// );
///
/// // ATR calculation
/// let atr_value = candles.atr(14);
///
/// // HL2 (typical price)
/// let hl2_value = candles.hl2();
/// ```

#[derive(Debug, Clone)]
pub struct SuperTrendOutput {
    pub upper: f64,
    pub lower: f64,
    pub value: f64,
    pub trend: i32, // 1 for bullish, 0 for bearish
    pub factor: f64,
}

#[derive(Debug, Clone)]
pub struct SuperTrendCluster {
    pub factors: Vec<f64>,
    pub outputs: Vec<SuperTrendOutput>,
    pub performance: f64,
    pub cluster_id: usize,
}

#[derive(Debug, Clone)]
pub struct AdaptiveMAOutput {
    pub value: f64,
    pub performance_index: f64,
}

pub trait TechnicalAnalysis {
    fn ema(&self, period: usize) -> f64;

    fn rsi(&self, period: usize) -> f64;

    fn macd(
        &self,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> MovingAverageConvergenceDivergenceOutput;

    /// Calculate Average True Range
    fn atr(&self, period: usize) -> f64;

    /// Calculate HL2 (typical price) for the last candle
    fn hl2(&self) -> f64;

    /// Calculate SuperTrend with a single factor
    fn supertrend(&self, atr_period: usize, factor: f64) -> SuperTrendOutput;

    /// Calculate SuperTrend with multiple factors (clustering approach)
    fn supertrend_cluster(
        &self,
        atr_period: usize,
        min_factor: f64,
        max_factor: f64,
        step: f64,
        performance_alpha: f64,
    ) -> Vec<SuperTrendOutput>;

    /// Calculate adaptive moving average based on performance
    fn adaptive_ma(&self, trailing_stop: f64, performance_index: f64, prev_ama: f64) -> f64;

    /// Calculate performance index for clustering
    fn performance_index(&self, alpha: f64) -> f64;

    /// Get the sign of price change
    fn price_change_sign(&self) -> f64;

    /// Calculate absolute average of price changes (for performance denominator)
    fn avg_abs_price_change(&self, period: usize) -> f64;

    /// Calculate K-means clustering for SuperTrend factors (simplified version)
    fn kmeans_cluster_supertrend(
        &self,
        atr_period: usize,
        min_factor: f64,
        max_factor: f64,
        step: f64,
        performance_alpha: f64,
        max_iterations: usize,
    ) -> Vec<SuperTrendCluster>;

    /// Get best SuperTrend from cluster analysis
    fn best_supertrend_from_cluster(
        &self,
        atr_period: usize,
        min_factor: f64,
        max_factor: f64,
        step: f64,
        performance_alpha: f64,
        cluster_type: &str, // "Best", "Average", "Worst"
    ) -> SuperTrendOutput;
}

impl TechnicalAnalysis for Vec<Candle> {
    fn ema(&self, period: usize) -> f64 {
        let mut ema = ExponentialMovingAverage::new(period).unwrap();

        for candle in self.iter() {
            ema.next(candle.close);
        }

        let last_candle = self.last().unwrap();

        ema.next(last_candle.close)
    }

    fn rsi(&self, period: usize) -> f64 {
        let mut rsi = RelativeStrengthIndex::new(period).unwrap();

        for candle in self.iter() {
            rsi.next(candle.close);
        }

        rsi.next(self.last().unwrap().close)
    }

    fn macd(
        &self,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> MovingAverageConvergenceDivergenceOutput {
        let mut macd =
            MovingAverageConvergenceDivergence::new(fast_period, slow_period, signal_period)
                .unwrap();

        for candle in self.iter() {
            macd.next(candle.close);
        }

        macd.next(self.last().unwrap().close)
    }

    fn atr(&self, period: usize) -> f64 {
        if self.len() < 2 {
            return 0.0;
        }

        let mut atr = AverageTrueRange::new(period).unwrap();

        for candle in self.iter() {
            atr.next(
                &ta::DataItem::builder()
                    .open(candle.open)
                    .high(candle.high)
                    .low(candle.low)
                    .close(candle.close)
                    .volume(candle.volume)
                    .build()
                    .unwrap(),
            );
        }

        atr.next(
            &ta::DataItem::builder()
                .open(self.last().unwrap().open)
                .high(self.last().unwrap().high)
                .low(self.last().unwrap().low)
                .close(self.last().unwrap().close)
                .volume(self.last().unwrap().volume)
                .build()
                .unwrap(),
        )
    }

    fn hl2(&self) -> f64 {
        if let Some(last_candle) = self.last() {
            (last_candle.high + last_candle.low) / 2.0
        } else {
            0.0
        }
    }

    fn supertrend(&self, atr_period: usize, factor: f64) -> SuperTrendOutput {
        if self.len() < atr_period {
            return SuperTrendOutput {
                upper: 0.0,
                lower: 0.0,
                value: 0.0,
                trend: 0,
                factor,
            };
        }

        let atr_value = self.atr(atr_period);
        let hl2_value = self.hl2();
        let close = self.last().unwrap().close;

        let basic_upper = hl2_value + atr_value * factor;
        let basic_lower = hl2_value - atr_value * factor;

        // For simplicity, we'll implement a basic SuperTrend
        // In a full implementation, you'd need to maintain state across candles
        let trend = if close > basic_upper {
            1
        } else if close < basic_lower {
            0
        } else {
            1
        };

        let final_upper = basic_upper;
        let final_lower = basic_lower;
        let supertrend_value = if trend == 1 { final_lower } else { final_upper };

        SuperTrendOutput {
            upper: final_upper,
            lower: final_lower,
            value: supertrend_value,
            trend,
            factor,
        }
    }

    fn supertrend_cluster(
        &self,
        atr_period: usize,
        min_factor: f64,
        max_factor: f64,
        step: f64,
        _performance_alpha: f64,
    ) -> Vec<SuperTrendOutput> {
        let mut results = Vec::new();
        let mut current_factor = min_factor;

        while current_factor <= max_factor {
            let st_output = self.supertrend(atr_period, current_factor);
            results.push(st_output);
            current_factor += step;
        }

        results
    }

    fn adaptive_ma(&self, trailing_stop: f64, performance_index: f64, prev_ama: f64) -> f64 {
        prev_ama + performance_index * (trailing_stop - prev_ama)
    }

    fn performance_index(&self, alpha: f64) -> f64 {
        if self.len() < 2 {
            return 0.0;
        }

        let price_changes: Vec<f64> = self
            .windows(2)
            .map(|window| window[1].close - window[0].close)
            .collect();

        if price_changes.is_empty() {
            return 0.0;
        }

        // Calculate EMA of absolute price changes
        let mut abs_change_ema = ExponentialMovingAverage::new(alpha as usize).unwrap();
        for change in &price_changes {
            abs_change_ema.next(change.abs());
        }

        let denominator = abs_change_ema.next(price_changes.last().unwrap().abs());

        // Performance is based on positive returns
        let performance = price_changes.iter().sum::<f64>().max(0.0);

        if denominator != 0.0 {
            performance / denominator
        } else {
            0.0
        }
    }

    fn price_change_sign(&self) -> f64 {
        if self.len() < 2 {
            return 0.0;
        }

        let last_two: Vec<&Candle> = self.iter().rev().take(2).collect();
        if last_two.len() == 2 {
            (last_two[0].close - last_two[1].close).signum()
        } else {
            0.0
        }
    }

    fn avg_abs_price_change(&self, period: usize) -> f64 {
        if self.len() < 2 {
            return 0.0;
        }

        let mut ema = ExponentialMovingAverage::new(period).unwrap();

        for window in self.windows(2) {
            let change = (window[1].close - window[0].close).abs();
            ema.next(change);
        }

        if let Some(last_window) = self.windows(2).last() {
            let change = (last_window[1].close - last_window[0].close).abs();
            ema.next(change)
        } else {
            0.0
        }
    }

    fn kmeans_cluster_supertrend(
        &self,
        atr_period: usize,
        min_factor: f64,
        max_factor: f64,
        step: f64,
        performance_alpha: f64,
        _max_iterations: usize,
    ) -> Vec<SuperTrendCluster> {
        // Generate all SuperTrend outputs for different factors
        let supertrend_outputs =
            self.supertrend_cluster(atr_period, min_factor, max_factor, step, performance_alpha);

        // Calculate performance for each SuperTrend
        let mut performances: Vec<f64> = Vec::new();
        for _st_output in &supertrend_outputs {
            // Simplified performance calculation
            let perf = self.performance_index(performance_alpha);
            performances.push(perf);
        }

        // Simplified K-means clustering (3 clusters)
        let mut clusters = Vec::new();

        // Sort by performance to create clusters
        let mut indexed_perfs: Vec<(usize, f64)> = performances
            .iter()
            .enumerate()
            .map(|(i, &perf)| (i, perf))
            .collect();
        indexed_perfs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let cluster_size = indexed_perfs.len() / 3;

        for cluster_id in 0..3 {
            let start_idx = cluster_id * cluster_size;
            let end_idx = if cluster_id == 2 {
                indexed_perfs.len()
            } else {
                (cluster_id + 1) * cluster_size
            };

            let cluster_indices: Vec<usize> = indexed_perfs[start_idx..end_idx]
                .iter()
                .map(|(idx, _)| *idx)
                .collect();

            let cluster_factors: Vec<f64> = cluster_indices
                .iter()
                .map(|&idx| supertrend_outputs[idx].factor)
                .collect();

            let cluster_outputs: Vec<SuperTrendOutput> = cluster_indices
                .iter()
                .map(|&idx| supertrend_outputs[idx].clone())
                .collect();

            let avg_performance = cluster_indices
                .iter()
                .map(|&idx| performances[idx])
                .sum::<f64>()
                / cluster_indices.len() as f64;

            clusters.push(SuperTrendCluster {
                factors: cluster_factors,
                outputs: cluster_outputs,
                performance: avg_performance,
                cluster_id,
            });
        }

        clusters
    }

    fn best_supertrend_from_cluster(
        &self,
        atr_period: usize,
        min_factor: f64,
        max_factor: f64,
        step: f64,
        performance_alpha: f64,
        cluster_type: &str,
    ) -> SuperTrendOutput {
        let clusters = self.kmeans_cluster_supertrend(
            atr_period,
            min_factor,
            max_factor,
            step,
            performance_alpha,
            1000, // max_iterations
        );

        let target_cluster_idx = match cluster_type {
            "Best" => clusters
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.performance.partial_cmp(&b.performance).unwrap())
                .map(|(idx, _)| idx),
            "Worst" => clusters
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.performance.partial_cmp(&b.performance).unwrap())
                .map(|(idx, _)| idx),
            "Average" => {
                // Get the middle cluster by performance
                let mut sorted_indices: Vec<(usize, f64)> = clusters
                    .iter()
                    .enumerate()
                    .map(|(idx, cluster)| (idx, cluster.performance))
                    .collect();
                sorted_indices.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                if sorted_indices.len() > 1 {
                    Some(sorted_indices[1].0)
                } else {
                    sorted_indices.first().map(|(idx, _)| *idx)
                }
            }
            _ => Some(0),
        };

        if let Some(cluster_idx) = target_cluster_idx {
            if let Some(cluster) = clusters.get(cluster_idx) {
                if let Some(best_output) = cluster.outputs.first() {
                    return best_output.clone();
                }
            }
        }

        // Fallback to simple SuperTrend with mid-range factor
        let fallback_factor = (min_factor + max_factor) / 2.0;
        self.supertrend(atr_period, fallback_factor)
    }
}
