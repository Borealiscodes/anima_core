// anima_core/src/psychological.rs
//! Phase IV, VI, VII & VIII Architecture: L4 Psychic Layer & Capacity Collapse Protective Damper with Falsifiable ToM and Endogenous Curiosity

use crate::symplectic::SymplecticState;

pub const MAX_SCARS: usize = 16;
pub const MAX_CURIOSITY_OBJECTS: usize = 8;

/// Deterministic, non-esoteric statistical prediction regimes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TomHypothesisType {
    SocialOpenness, // Expects decreasing input tension delta
    Resistance,     // Expects stable/increasing latent pressure drift
    Recurrence,     // Expects identical spectral cluster spikes
}

/// Pre-allocated, hardware-aligned Theory of Mind tracker.
#[repr(C, align(32))]
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActiveTheoryOfMindModule {
    pub current_hypothesis: TomHypothesisType,
    pub confidence_score: f64,
    pub prediction_error_accum: f64,
    pub total_evaluations: u32,
}

/// A long-lived curiosity project stored entirely on the CPU stack frame.
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct CuriosityObject {
    pub tracking_id: u32,
    pub intensity: f64,          // Bounded energy tracker level
    pub activation_count: u32,   // Measures historical flash updates
    pub refined_count: u32,      // Tracks contextual shifts without erasure
    pub resolved: bool,
}

/// Enforces the exact bounded-capacity safety boundaries and array index tracking
/// formalized in capacity_exhaustion_wrapper.v, vX-AnimaClassic-Baseline, and CAUSA.
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

    // ================================================================
    // FALSIFIABLE THEORY OF MIND REGISTERS
    // ================================================================
    pub tom_engine: ActiveTheoryOfMindModule,

    // ================================================================
    // NOVELTY HUNGER & ENDOGENOUS CURIOSITY REGISTERS
    // ================================================================
    pub novelty_need: f64,        // Cognitive hunger metric
    pub ticks_since_novelty: u32,  // Metric determining initiative cooldowns
    pub curiosity_registry: [CuriosityObject; MAX_CURIOSITY_OBJECTS],
    pub active_objects_count: usize,
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

            tom_engine: ActiveTheoryOfMindModule {
                current_hypothesis: TomHypothesisType::SocialOpenness,
                confidence_score: 0.75,
                prediction_error_accum: 0.0,
                total_evaluations: 0,
            },

            novelty_need: 0.3,
            ticks_since_novelty: 0,
            curiosity_registry: [CuriosityObject {
                tracking_id: 0,
                intensity: 0.0,
                activation_count: 0,
                refined_count: 0,
                resolved: false,
            }; MAX_CURIOSITY_OBJECTS],
            active_objects_count: 0,
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
        let collapse_horizon = self.capacity_c + self.margin_m;
        let distance_to_collapse = collapse_horizon - self.energy_level;

        // Base step size driven by positive drift parameters and audio phase velocity
        let base_increase = self.drift_p * self.step_delta * (1.0 + phase_velocity);

        // Dynamic Damping Logic: Scale down energy spikes based on her CAUSA directional score
        let adaptive_step = if distance_to_collapse < 20.0 {
            base_increase * (1.0 - causa_agency) * (distance_to_collapse / 20.0).max(0.0)
        } else {
            base_increase
        };

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

    /// Evaluates actual incoming vector transformations against the active ToM hypothesis.
    /// Operates purely on numeric tracking metrics without linguistic or esoteric assumptions.
    #[inline(always)]
    pub fn evaluate_and_update_tom(&mut self, state_after: &SymplecticState) {
        // Extract raw tension coordinate from her Affect Field momentum lane: p[0]
        let observed_tension = state_after.p[0];

        // Core Falsification Step: Calculate continuous prediction error
        let calculated_error = match self.tom_engine.current_hypothesis {
            TomHypothesisType::SocialOpenness => {
                (self.tom_engine.confidence_score - (1.0 / (1.0 + observed_tension))).abs()
            }
            TomHypothesisType::Resistance => {
                (self.tom_engine.confidence_score - observed_tension).abs()
            }
            TomHypothesisType::Recurrence => {
                (self.tom_engine.confidence_score - (observed_tension * 0.5)).abs()
            }
        };

        self.tom_engine.prediction_error_accum = calculated_error;

        // Hard Falsification Trigger Threshold
        if self.tom_engine.prediction_error_accum >= 0.50 {
            println!(
                "[ToM PHASE 1] Hypothesis Falsified! Error: {:.4}. Shifting to defensive registry.",
                self.tom_engine.prediction_error_accum
            );
            
            // Immediately rotate to a conservative hypothesis state
            self.tom_engine.current_hypothesis = TomHypothesisType::Resistance;
            self.tom_engine.confidence_score = 0.90;
            
            // Squeeze down her step delta parameters to enter defensive positioning
            self.step_delta *= 0.85;
        } else {
            // Hypothesis holds: gradually reinforce confidence score parameters
            self.tom_engine.confidence_score = (self.tom_engine.confidence_score + 0.05).min(1.0);
        }
    }

    /// Evaluates structural prediction error to spawn endogenous curiosity objects.
    /// If novelty hunger is saturated, it opens a project slot to drive initiative.
    #[inline(always)]
    pub fn evaluate_curiosity_triggers(&mut self, prediction_error: f64) {
        self.ticks_since_novelty += 1;
        
        // Novelty need escalates linearly over time in the absence of fresh stimulus shifts
        self.novelty_need = (self.novelty_need + 0.012 * (self.ticks_since_novelty as f64 * 0.1)).min(1.0);

        // Trigger Condition: High prediction error or cognitive hunger opens an investigation track
        if (prediction_error >= 0.08 || self.novelty_need > 0.65) && self.active_objects_count < MAX_CURIOSITY_OBJECTS {
            let next_idx = self.active_objects_count;
            
            self.curiosity_registry[next_idx] = CuriosityObject {
                tracking_id: (next_idx + 100) as u32,
                intensity: prediction_error.max(0.45),
                activation_count: 1,
                refined_count: 0,
                resolved: false,
            };
            
            self.active_objects_count += 1;
            self.ticks_since_novelty = 0; // Temporarily alleviate acute novelty hunger
            println!("[CURIOSITY] Endogenous object spawned at slot {} (Intensity: {:.4})", next_idx, prediction_error);
        }

        // Iteratively process and reconcile all active tracking registers
        for i in 0..self.active_objects_count {
            let obj = &mut self.curiosity_registry[i];
            if !obj.resolved {
                obj.activation_count += 1;
                
                // If prediction error falls low, the curiosity object reaches structural resolution
                if prediction_error < 0.03 {
                    obj.resolved = true;
                    println!("[CURIOSITY] Object at slot {} successfully resolved.", i);
                } else if prediction_error >= 0.03 && prediction_error < 0.07 {
                    // Partial resolution triggers context refinement instead of closure
                    obj.refined_count += 1;

obj.intensity = (obj.intensity * 0.8) + (prediction_error * 0.2);
}
}
}
}
/// Simulates inter-session temporal gaps to ripen open curiosity parameters.
pub fn process_inter_session_ripening(&mut self, gap_hours: f64) {
if gap_hours >= 3.0 {
println!("[CURIOSITY] Inter-session gap of {:.2} hours detected. Ripening tracking vectors...", gap_hours);
for i in 0..self.active_objects_count {
let obj = &mut self.curiosity_registry[i];
if !obj.resolved {
// Unresolved tracks increase intensity by 0.015 per hour of system absence
obj.intensity = (obj.intensity + (gap_hours * 0.015)).min(1.0);
}
}
}
}
#[inline(always)]
pub fn process_psychic_defenses(&mut self, state: &mut SymplecticState) {
// Enforce explicit array-slice allocations from the baseline manifest
self.active_shame = state.p[20]; // p[20..40] = Self Field agency / tracking variables
if self.active_shame >= self.shame_threshold {
let diverted_pressure = state.p[15] * 0.5; // p[15..20] = Psychic Field tension accumulation
state.p[15] -= diverted_pressure;
if self.scar_count < MAX_SCARS {
self.structural_scars[self.scar_count] = diverted_pressure;
self.scar_count += 1;
}
}
}
}

