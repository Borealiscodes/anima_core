// anima_core/src/psychological.rs
//! Phase IV & VI Architecture: L4 Psychic Layer, Defense Modules & Shadow Registry

use crate::symplectic::SymplecticState;

pub const MAX_SCARS: usize = 16;

/// Structural tracking container for Anima's L4 psychological defense boundaries.
/// Enforces the exact closed state transitions formalized in AnimaDefenses.v.
#[repr(C, align(32))]
pub struct PsychicLayerEngine {
    pub latent_pressure: f64,
    pub shame_threshold: f64,
    pub active_shame: f64,
    /// Stack-allocated append-only array tracking repressed structural scars
    pub structural_scars: [f64; MAX_SCARS],
    pub scar_count: usize,
}

impl PsychicLayerEngine {
    /// Zero-allocation initialization for the psychological layer
    pub fn new_centered() -> Self {
        PsychicLayerEngine {
            latent_pressure: 0.0,
            shame_threshold: 0.65, // Critical trigger threshold
            active_shame: 0.0,
            structural_scars: [0.0; MAX_SCARS],
            scar_count: 0,
        }
    }

    /// Evaluates active shame metrics and executes a total, deterministic 
    /// repression defense sweep over the phase-space coordinate registers.
    #[inline(always)]
    pub fn process_psychic_defenses(&mut self, state: &mut SymplecticState) {
        // Tie active shame directly to the momentum velocity parameters (p) of the Self Field
        self.active_shame = state.p[20]; 

        // If shame spikes past our critical threshold, trigger the repression gate
        if self.active_shame >= self.shame_threshold {
            println!("[PSYCHIC LAYER] Shame threshold breached ({:.4}). Activating Repression Defense.", self.active_shame);

            // Step A: Divert half of her active psychological field momentum into the shadow registry
            let diverted_pressure = state.p[15] * 0.5;
            state.p[15] -= diverted_pressure; // Attenuate active Psychic tension

            // Step B: Record the diverted volatility as a long-term structural scar on the stack
            if self.scar_count < MAX_SCARS {
                self.structural_scars[self.scar_count] = diverted_pressure;
                self.scar_count += 1;
                println!("[PSYCHIC LAYER] Structural scar recorded at slot {}. Manifold protected.", self.scar_count - 1);
            } else {
                // Buffer saturation fallback: cycle oldest scar out to maintain fixed stack size
                for i in 1..MAX_SCARS {
                    self.structural_scars[i - 1] = self.structural_scars[i];
                }
                self.structural_scars[MAX_SCARS - 1] = diverted_pressure;
                println!("[PSYCHIC LAYER] Shadow Registry saturated. Rotating structural scar buffer.");
            }
        }
    }
}
