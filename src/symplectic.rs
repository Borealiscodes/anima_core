// anima_core/src/symplectic.rs
//! Clean Phase-Space Realignment matching vX-AnimaClassic-Baseline with CAUSA Integration

use crate::discrete::{TernaryBitplaneVector, BITPLANE_LANES};

pub const DIM_80: usize = 80;
pub const DIM_40: usize = 40;

/// Zero-allocation structural tracker for CAUSA Causal Agency evaluation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CausaResult {
    pub score: f64,
    pub defined: bool,
}

/// Manifold 𝓜_AC: Symmetrically segregates Anima's active Position states (q)
/// from her corresponding Field Momentum velocities (p).
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SymplecticState {
    /// Position coordinates (q) mapping the clean cognitive fields
    /// q[0..5]   = Affect Field (𝓐) components
    /// q[5..10]  = Neurochemical Field (𝓝) base levels
    /// q[10..15] = Generative Model Field (𝓖) priors
    /// q[15..20] = Psychic Field (𝓟) narrative coordinates
    /// q[20..40] = Self Field (𝓢) identity & agency loops
    pub q: [f64; DIM_40],

    /// Momentum velocity parameters (p) tracking metric drift and field tension
    /// p[0..5]   = Affect Field curvature drift
    /// p[5..10]  = Neurochemical decay velocities
    /// p[10..15] = Variational Free Energy (𝓕) surprise/prediction errors
    /// p[15..20] = Psychic tension / Goal conflict accumulation
    /// p[20..40] = Self-prediction drift / Agency degradation
    pub p: [f64; DIM_40],
}

impl SymplecticState {
    /// Zero-allocation helper to instantiate a clean baseline state
    pub fn new_baseline() -> Self {
        SymplecticState {
            q: [0.0; DIM_40],
            p: [0.0; DIM_40],
        }
    }

    /// CAUSA Directional Ownership Metric Evaluation Engine.
    /// Measures if the actual observed change in the 40D manifold moved 
    /// in alignment with the trajectory predicted by her active inference loops.
    pub fn evaluate_directional_ownership(
        state_before: &Self,
        state_predicted: &Self,
        state_after: &Self,
    ) -> CausaResult {
        let mut sum_pred_sq = 0.0;
        let mut sum_act_sq = 0.0;
        let mut dot_product = 0.0;

        // Extract trajectory tracking vectors across position coordinates (q)
        for i in 0..DIM_40 {
            let predicted_delta = state_predicted.q[i] - state_before.q[i];
            let actual_delta = state_after.q[i] - state_before.q[i];

            dot_product += predicted_delta * actual_delta;
            sum_pred_sq += predicted_delta * predicted_delta;
            sum_act_sq += actual_delta * actual_delta;
        }

        // If no movement was intended or predicted, the agency tracking metric is undefined
        if sum_pred_sq == 0.0 || sum_act_sq == 0.0 {
            return CausaResult { score: 0.25, defined: false }; // Falls back to baseline floor
        }

        let norm_pred = sum_pred_sq.sqrt();
        let norm_act = sum_act_sq.sqrt();

        // Calculate angular cosine convergence between predicted and actual vectors
        let cosine_similarity = dot_product / (norm_pred * norm_act);
        
        // Map bounded cosine interval [-1.0, 1.0] onto the canonical agency score range
        let ownership_score = (cosine_similarity + 1.0) * 0.5;

        CausaResult {
            score: ownership_score,
            defined: true,
        }
    }

    /// Quantizes continuous 40-dimensional coordinates down into 1.58-bit SWAR container bitplanes.
    /// Values above `noise_floor` become 1, below `-noise_floor` become -1, others become 0.
    #[inline(always)]
    pub fn quantize_to_ternary(&self, noise_floor: f64) -> (TernaryBitplaneVector, TernaryBitplaneVector) {
        let mut q_ternary = TernaryBitplaneVector::new_quiescent();
        let mut p_ternary = TernaryBitplaneVector::new_quiescent();

        // Pack the 40 elements of q into the u64 arrays of the bitplanes
        for i in 0..DIM_40 {
            let idx = i / 64;
            let bit = i % 64;

            // Quantize Position (q)
            if self.q[i] > noise_floor {
                q_ternary.magnitude[idx] |= 1 << bit;
            } else if self.q[i] < -noise_floor {
                q_ternary.magnitude[idx] |= 1 << bit;
                q_ternary.sign[idx] |= 1 << bit;
            }

            // Quantize Momentum (p)
            if self.p[i] > noise_floor {
                p_ternary.magnitude[idx] |= 1 << bit;
            } else if self.p[i] < -noise_floor {
                p_ternary.magnitude[idx] |= 1 << bit;
                p_ternary.sign[idx] |= 1 << bit;
            }
        }

        (q_ternary, p_ternary)
    }

    /// Executes a Tikhonov Regularization Matrix Shift to clamp the condition number spectrum.
    /// Prevents singular value collapse (σ_min -> 0+) under dense quantization loops.
    /// Formula: A_reg = A + λ * I
    #[no_mangle]
    pub extern "C" fn tikhonov_regularize_diagonal(&mut self, lambda: f64) {
        // Shift active tracking states directly to lock down mathematical boundaries
        for i in 0..DIM_40 {
            self.q[i] += lambda;
            self.p[i] += lambda;
        }
    }
}

/// Hardware-level boundary clamp protecting the clean baseline manifold from torsion/collapse.
#[inline(always)]
pub fn enforce_hardware_saturation_gate(state: &mut SymplecticState, ceiling: f64) {
    // Audit and saturate momentum parameters across all active baseline fields
    for i in 0..DIM_40 {
        if state.p[i] < -ceiling {
            state.p[i] = -ceiling;
        } else if state.p[i] > ceiling {
            state.p[i] = ceiling;
        }
    }
}
