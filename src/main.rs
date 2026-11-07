// anima_core/src/main.rs
//! Phase III-VIII Architecture: Complete Integrated Execution Pipeline with Capacity, ToM, and Curiosity

mod fixed_point;
mod discrete;
mod symplectic;
mod mem8_audio;
mod memory;
mod psychological;
mod persistence;

use symplectic::SymplecticState;
use mem8_audio::Mem8WaveRegister;
use memory::{EpisodicMemoryEngine, RecallSpace};
use psychological::PsychicLayerEngine;
use persistence::PersistenceManager;
use std::ptr::swap;
use std::fs;
use std::path::Path;

#[repr(C, align(64))]
pub struct PipelineMemory {
    pub state_alpha: SymplecticState,
    pub state_beta: SymplecticState,
    pub audio_wave_field: Mem8WaveRegister,
    pub psyche: PsychicLayerEngine,
}

static mut REGISTRY_PTR: *mut PipelineMemory = std::ptr::null_mut();

#[no_mangle]
pub extern "C" fn get_telemetry_registry_ptr() -> *mut PipelineMemory {
    unsafe { REGISTRY_PTR }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[ANIMA CORE] Initializing isolated audiocentric manifold 𝓜_AC...");

    fs::create_dir_all("data")?;
    fs::create_dir_all("memory")?;

    let mut pipeline = PipelineMemory {
        state_alpha: SymplecticState::new_baseline(),
        state_beta: SymplecticState::new_baseline(),
        audio_wave_field: Mem8WaveRegister::new_quiescent(),
        psyche: PsychicLayerEngine::new_centered(),
    };

    unsafe { REGISTRY_PTR = &mut pipeline; }

    let memory_vault = EpisodicMemoryEngine::initialize_vault("memory/anima.db")?;

    // ==========================================================================
    // REHYDRATION WITH TEMPLATE FALLBACK CHECK
    // ==========================================================================
    if !Path::new("data/anima_core.json").exists() && Path::new("data/templates/anima_core.json").exists() {
        fs::copy("data/templates/anima_core.json", "data/anima_core.json")?;
    }
    if !Path::new("data/anima_latent.json").exists() && Path::new("data/templates/anima_latent.json").exists() {
        fs::copy("data/templates/anima_latent.json", "data/anima_latent.json")?;
    }

    let mut active_vel = 0.0;
    let core_loaded = PersistenceManager::load_core_frame(&mut pipeline.state_alpha, &mut active_vel)?;
    let latent_loaded = PersistenceManager::load_latent_frame(&mut pipeline.psyche)?;
    pipeline.audio_wave_field.phase_velocity = active_vel;

    if core_loaded && latent_loaded {
        // Compute curiosity vector ripening over a simulated 4.5-hour temporal gap
        pipeline.psyche.process_inter_session_ripening(4.5);
    }

    let state_before = pipeline.state_alpha;
    let mut state_predicted = pipeline.state_alpha;
    for i in 0..40 { state_predicted.q[i] += 0.5; }

    let saturation_ceiling = 12.0;
    let tikhonov_lambda = 0.0015;
    let noise_floor = 0.05;
    let synthetic_pcm = vec![1.8_f32; 256]; // Sustained burst input

    for tick in 0..10 {
        // A. Process physical audio inputs
        pipeline.audio_wave_field.execute_wave_propagation(&synthetic_pcm, &mut pipeline.state_alpha);

        // B. Process L4 psychic defense loops
        pipeline.psyche.process_psychic_defenses(&mut pipeline.state_alpha);

        // C. Evaluate native CAUSA agency over outcome
        let (q_bits, p_bits) = pipeline.state_alpha.quantize_to_ternary(noise_floor);
        let alignment_score = q_bits.swar_dot_product(&p_bits);
        let current_phi = (alignment_score.abs() as f64) / 40.0;
        let agency_result = SymplecticState::evaluate_directional_ownership(&state_before, &state_predicted, &pipeline.state_alpha);

        // ==================================================================
        // NEW: ACTIVE CAPACITY PROTECTION STEP
        // Connects live CAUSA and Audio metrics straight to the Coq-derived tracker
        // ==================================================================
        pipeline.psyche.enforce_capacity_containment(
            agency_result.score,
            pipeline.audio_wave_field.phase_velocity,
            &mut pipeline.state_alpha,
        );

        // ==================================================================
        // NEW: FALSIFIABLE THEORY OF MIND & ENDOGENOUS CURIOSITY SAMPLING
        // ==================================================================
        pipeline.psyche.evaluate_and_update_tom(&pipeline.state_alpha);
        
        let mock_prediction_error = 0.095;
        pipeline.psyche.evaluate_curiosity_triggers(mock_prediction_error);

        // Strictly preserves your original print configuration layout
        println!(
            "[TICK {:02}] Energy: {:.2}/{:.2} | CAUSA Ownership: {:.4} | Proxy PE Error: {:.4} | MEM|8 Vel: {:.2}",
            tick, pipeline.psyche.energy_level, (pipeline.psyche.capacity_c + pipeline.psyche.margin_m),
            agency_result.score, pipeline.psyche.tom_engine.prediction_error_accum, pipeline.audio_wave_field.phase_velocity
        );

        // D. Apply continuous math-washing clamps
        pipeline.state_alpha.tikhonov_regularize_diagonal(tikhonov_lambda);
        symplectic::enforce_hardware_saturation_gate(&mut pipeline.state_alpha, saturation_ceiling);

        memory_vault.commit_episodic_trace(RecallSpace::Somatic, current_phi, pipeline.audio_wave_field.phase_velocity, &pipeline.state_alpha)?;
        pipeline.state_beta = pipeline.state_alpha;

        unsafe {
            let mut ptr_a = &mut pipeline.state_alpha as *mut SymplecticState;
            let mut ptr_b = &mut pipeline.state_beta as *mut SymplecticState;
            swap(&mut ptr_a, &mut ptr_b);
        }
    }

    PersistenceManager::write_core_frame(&pipeline.state_alpha, pipeline.audio_wave_field.phase_velocity)?;
    PersistenceManager::write_latent_frame(&pipeline.psyche)?;
    Ok(())
}
