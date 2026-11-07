// anima_core/src/persistence.rs
//! Phase V Architecture: Partitioned JSON Inter-Session State Persistence

use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};
use crate::symplectic::SymplecticState;
use crate::psychological::{PsychicLayerEngine, MAX_SCARS};

/// Target Data Model for high-frequency, low-latency manifold tracking
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimaCorePayload {
    pub q_position: Vec<f64>,
    pub p_momentum: Vec<f64>,
    pub phase_velocity: f64,
}

/// Target Data Model for slow-metabolizing chronic psychological scars
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimaLatentPayload {
    pub latent_pressure: f64,
    pub shame_threshold: f64,
    pub active_shame: f64,
    pub structural_scars: Vec<f64>,
}

pub struct PersistenceManager;

impl PersistenceManager {
    /// Serializes active field coordinates cleanly to anima_core.json
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

    /// Serializes slow psychological traits cleanly to anima_latent.json
    pub fn write_latent_frame(psyche: &PsychicLayerEngine) -> Result<(), Box<dyn std::error::Error>> {
        let mut scars_vec = Vec::with_capacity(psyche.scar_count);
        for i in 0..psyche.scar_count {
            if i < MAX_SCARS {
                scars_vec.push(psyche.structural_scars[i]);
            }
        }

        let payload = AnimaLatentPayload {
            latent_pressure: psyche.latent_pressure,
            shame_threshold: psyche.shame_threshold,
            active_shame: psyche.active_shame,
            structural_scars: scars_vec,
        };

        let json_str = serde_json::to_string_pretty(&payload)?;
        let mut file = File::create("data/anima_latent.json")?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }
}
