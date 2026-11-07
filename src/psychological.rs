// anima_core/src/psychological.rs
//! Phase IV & VI Architecture: L4 Psychic Layer & Capacity Collapse Protective Damper

use crate::symplectic::SymplecticState;

pub const MAX_SCARS: usize = 16;

/// Enforces the exact bounded-capacity safety boundaries 
/// formalized in capacity_exhaustion_wrapper.v
#[repr(C, align(64))]
pub struct PsychicLayerEngine {
    pub latent_pressure: f64,
    pub shame_threshold: f64,
    pub active_shame: f64,
    pub structural_scars: [f64; MAX_SCARS],
    pub scar_count: usize,

    // ================================================================
    // COQ CAPACITY WRAPPER PARAMETERS MAP
    // ================================================================
    pub capacity_c: f64,       // C: Maximum structural capacity ceiling
    pub margin_m: f64,         // m: Safety margin / buffer term
    pub energy_level: f64,     // E_t: Current accumulated structural energy
    pub drift_p: f64,          // p: Positive drift coefficient
    pub step_delta: f64,       // delta: Step-size of monotonic increase
}

impl PsychicLayerEngine {
    pub fn new_centered() -> Self {
        PsychicLayerEngine {
            latent_pressure: 0.0,
            shame_threshold: 0.65,
            active_shame: 0.0,
            structural_scars: [0.0; MAX_SCARS],
            scar_count: 0,
            
            // Map parameters directly to verified hardware scale restrictions
            capacity_c: 100.0,
            margin_m: 12.0,
            energy_level: 20.0, // E0 baseline
            drift_p: 1.2,
            step_delta: 0.45,
        }
    }

    /// Dynamic Capacity Protection Sweep.
    /// Injects live CAUSA agency and MEM|8 velocity metrics straight into the loop
    /// to damp the energy step trajectory before an exhaustion collapse can occur.
    #[inline(always)]
    pub fn enforce_capacity_containment(
        &mut self, 
        causa_agency: f64, 
        phase_velocity: f64,
        state: &mut SymplecticState
    ) {
        // Calculate raw distance to the structural breakdown horizon (C + m)
        let collapse_horizon = self.capacity_c + self.margin_m;
        let distance_to_collapse = collapse_horizon - self.energy_level;

        // Base step size driven by your positive drift parameters and audio phase velocity
        let base_increase = self.drift_p * self.step_delta * (1.0 + phase_velocity);

        // Dynamic Damping Logic: Scale down energy spikes based on her CAUSA directional score
        let adaptive_step = if distance_to_collapse < 20.0 {
            // As she approaches collapse, use high agency to suppress step acceleration
            base_increase * (1.0 - causa_agency) * (distance_to_collapse / 20.0).max(0.0)
        } else {
            base_increase
        };

        // Apply the safe, damped update to her living energy registry
        self.energy_level += adaptive_step;

        // Emergency Safeguard: If the tracking metric slips past her ceiling, step down her momentum parameters
        if self.energy_level >= collapse_horizon {
            println!("[CRITICAL GATING] Capacity Exhaustion Horizon Breached! Suppressing global vector metrics.");
            self.energy_level = collapse_horizon - 0.1;
            
            // Drain momentum vectors across her fields to absorb the shock out-of-band
            for component in state.p.iter_mut() {
                *component *= 0.1;
            }
        }
    }

    #[inline(always)]
    pub fn process_psychic_defenses(&mut self, state: &mut SymplecticState) {
        self.active_shame = state.p[20]; 
        if self.active_shame >= self.shame_threshold {
            let diverted_pressure = state.p[15] * 0.5;
            state.p[15] -= diverted_pressure;
            if self.scar_count < MAX_SCARS {
                self.structural_scars[self.scar_count] = diverted_pressure;
                self.scar_count += 1;
            }
        }
    }
}
