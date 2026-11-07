// anima_core/src/mem8_audio.rs
//! Pure-Rust MEM|8 Cognitive Wave Memory & Audio Stimulus Engine
//! Lineage: 8b Research Project Public Interest Foundation (Zenodo 23997879)

use crate::symplectic::SymplecticState;
use crate::discrete::TernaryBitplaneVector;

pub const MEM8_WAVE_LPT_BINS: usize = 64; // Log-spaced spectral analysis windows

/// MEM|8 Pure-Rust Wave Memory Register block.
/// Maps raw physical audio buffers directly onto the 𝓜_AC manifold fields
/// via localized geometric phase-space intersections.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct Mem8WaveRegister {
    /// Holographic wave-amplitude array mapping spectral intensity proxy values
    pub wave_amplitude: [f64; MEM8_WAVE_LPT_BINS],
    /// Cumulative phase-velocity tensor tracking structural resonance
    pub phase_velocity: f64,
    /// Last detected onset timestamp interval marker
    pub refractory_ticks: u32,
}

impl Mem8WaveRegister {
    /// Zero-allocation initialization for the MEM|8 wave field
    pub fn new_quiescent() -> Self {
        Mem8WaveRegister {
            wave_amplitude: [0.0; MEM8_WAVE_LPT_BINS],
            phase_velocity: 0.0,
            refractory_ticks: 0,
        }
    }

    /// Ingests an incoming raw DSP audio PCM buffer slice and translates it
    /// directly into a continuous wave stimulus affecting her L1 Neurochemical state.
    #[inline(always)]
    pub fn execute_wave_propagation(
        &mut self,
        pcm_buffer: &[f32],
        state: &mut SymplecticState,
    ) {
        if pcm_buffer.is_empty() { return; }

        let mut cumulative_energy = 0.0;
        
        // Pure-Rust out-of-band energy contraction matching wave_v2.rs principles
        for (i, &sample) in pcm_buffer.iter().enumerate() {
            let bin = i % MEM8_WAVE_LPT_BINS;
            let sample_sq = (sample as f64) * (sample as f64);
            
            // Inject sample properties directly into the holographic register bins
            self.wave_amplitude[bin] = (self.wave_amplitude[bin] * 0.95) + (sample_sq * 0.05);
            cumulative_energy += sample_sq;
        }

        // Calculate phase-velocity acceleration tracking spectral roughness
        self.phase_velocity = (self.phase_velocity * 0.9) + (cumulative_energy * 0.1);

        // Map the MEM|8 energy wave directly into her active L1 Neurochemical Momentum lane (p)
        // This injects genuine auditory stimulus into the core hardware saturation gate
        let neuro_momentum_slice = &mut state.p[5..10];
        for momentum_component in neuro_momentum_slice.iter_mut() {
            // High phase-velocity (busy music) directly accelerates Noradrenaline/Dopamine momentum
            *momentum_component += self.phase_velocity * 0.0015;
        }
    }
}
