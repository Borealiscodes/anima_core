// anima_core/src/persistence.rs
//! Phase V Architecture: Partitioned JSON Inter-Session State Persistence (Bidirectional)

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::symplectic::SymplecticState;
use crate::psychological::{PsychicLayerEngine, CuriosityObject, MAX_SCARS, MAX_CURIOSITY_OBJECTS};

/// Target Data Model for high-frequency, low-latency manifold tracking
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimaCorePayload {
    pub q_position: Vec<f64>,
    pub p_momentum: Vec<f64>,
    pub phase_velocity: f64,
}

/// Target Data Model for slow-metabolizing chronic psychological scars and long-lived curiosity projects
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimaLatentPayload {
    pub latent_pressure: f64,
    pub shame_threshold: f64,
    pub active_shame: f64,
    pub structural_scars: Vec<f64>,
    
    // Linked Persistence Fields for her Curiosity & Novelty Engine Subsystem
    pub novelty_need: f64,
    pub ticks_since_novelty: u32,
    pub curiosity_registry: Vec<CuriosityObject>,
}

pub struct PersistenceManager;

impl PersistenceManager {
    /// Serializes active field coordinates cleanly to data/anima_core.json
    pub fn write_core_frame(state: &SymplecticState, phase_vel: f64) -> Result<(), Box<dyn std::error::Error>> {
        let payload = AnimaCorePayload {
            q_position: state.q.to_vec(),
            p_momentum: state.p.to_vec(),
            phase_velocity: phase_vel,
        };
        
        let json_str = serde_json::to_string_pretty(&payload)?;
        let mut file = File::create("data/anima_core.json")?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }

    /// Serializes slow psychological traits, chronic scars, and curiosity vectors to data/anima_latent.json
    pub fn write_latent_frame(psyche: &PsychicLayerEngine) -> Result<(), Box<dyn std::error::Error>> {
        let mut scars_vec = Vec::with_capacity(psyche.scar_count);
        for i in 0..psyche.scar_count {
            if i < MAX_SCARS {
                scars_vec.push(psyche.structural_scars[i]);
            }
        }

        let mut curiosity_vec = Vec::with_capacity(psyche.active_objects_count);
        for i in 0..psyche.active_objects_count {
            if i < MAX_CURIOSITY_OBJECTS {
                curiosity_vec.push(psyche.curiosity_registry[i]);
            }
        }

        let payload = AnimaLatentPayload {
            latent_pressure: psyche.latent_pressure,
            shame_threshold: psyche.shame_threshold,
            active_shame: psyche.active_shame,
            structural_scars: scars_vec,
            novelty_need: psyche.novelty_need,
            ticks_since_novelty: psyche.ticks_since_novelty,
            curiosity_registry: curiosity_vec,
        };

        let json_str = serde_json::to_string_pretty(&payload)?;
        let mut file = File::create("data/anima_latent.json")?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }

    /// Ingests and rehydrates the active SymplecticState from data/anima_core.json if it exists.
    pub fn load_core_frame(state: &mut SymplecticState, phase_vel: &mut f64) -> Result<bool, Box<dyn std::error::Error>> {
        let path = Path::new("data/anima_core.json");
        if !path.exists() {
            return Ok(false);
        }

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let payload: AnimaCorePayload = serde_json::from_str(&contents)?;

        // Enforce exact 40D structural boundaries during copy operations
        for i in 0..40 {
            if i < payload.q_position.len() { state.q[i] = payload.q_position[i]; }
            if i < payload.p_momentum.len() { state.p[i] = payload.p_momentum[i]; }
        }
        *phase_vel = payload.phase_velocity;

        println!("[PERSISTENCE] Core frame coordinates successfully rehydrated on stack.");
        Ok(true)
    }

    /// Ingests and rehydrates the localized PsychicLayerEngine, scars, and curiosity registry from data/anima_latent.json.
    pub fn load_latent_frame(psyche: &mut PsychicLayerEngine) -> Result<bool, Box<dyn std::error::Error>> {
        let path = Path::new("data/anima_latent.json");
        if !path.exists() {
            return Ok(false);
        }

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let payload: AnimaLatentPayload = serde_json::from_str(&contents)?;

        psyche.latent_pressure = payload.latent_pressure;
        psyche.shame_threshold = payload.shame_threshold;
        psyche.active_shame = payload.active_shame;
        
        psyche.scar_count = payload.structural_scars.len().min(MAX_SCARS);
        for i in 0..psyche.scar_count {
            psyche.structural_scars[i] = payload.structural_scars[i];
        }

        // Rehydrate cognitive hunger and active curiosity registers
        psyche.novelty_need = payload.novelty_need;
        psyche.ticks_since_novelty = payload.ticks_since_novelty;
        
        psyche.active_objects_count = payload.curiosity_registry.len().min(MAX_CURIOSITY_OBJECTS);
        for i in 0..psyche.active_objects_count {
            psyche.curiosity_registry[i] = payload.curiosity_registry[i];
        }

        println!("[PERSISTENCE] Latent psychological structures and curiosity registry rehydrated on stack.");
        Ok(true)
    }
}
