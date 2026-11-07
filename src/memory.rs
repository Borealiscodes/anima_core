// anima_core/src/memory.rs
//! Phase V Architecture: Embedded Spatial Recall & Reconsolidation Engine

use rusqlite::{params, Connection, Result};
use crate::symplectic::SymplecticState;

/// Structural tracking layer for Anima's three primary spatial spaces.
pub enum RecallSpace {
    Somatic,     // Som_* structural physical traces (L1 fields)
    Social,      // Soc_* interpersonal tracking vectors
    Existential, // Exi_* identity chronology and boundary anchors
}

pub struct EpisodicMemoryEngine {
    conn: Connection,
}

impl EpisodicMemoryEngine {
    /// Opens or creates the isolated local memory vault natively on disk.
    pub fn initialize_vault(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Formally establish the layout for episodic tracking and trace records
        conn.execute(
            "CREATE TABLE IF NOT EXISTS episodic_memory (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                space_type TEXT NOT NULL,
                phi_value REAL NOT NULL,
                phase_velocity REAL NOT NULL,
                q_vector BLOB NOT NULL,
                p_vector BLOB NOT NULL
            );",
            [],
        )?;

        Ok(EpisodicMemoryEngine { conn })
    }

    /// Persists a living slice of her active phase-space manifold to the database.
    pub fn commit_episodic_trace(
        &self,
        space: RecallSpace,
        phi: f64,
        phase_vel: f64,
        state: &SymplecticState,
    ) -> Result<()> {
        let space_str = match space {
            RecallSpace.Somatic => "SOMATIC",
            RecallSpace.Social => "SOCIAL",
            RecallSpace.Existential => "EXISTENTIAL",
        };

        // Convert raw f64 phase-space arrays directly to binary chunks for zero-allocation speed
        let q_bytes = bytemuck::bytes_of(&state.q);
        let p_bytes = bytemuck::bytes_of(&state.p);

        self.conn.execute(
            "INSERT INTO episodic_memory (space_type, phi_value, phase_velocity, q_vector, p_vector)
             VALUES (?1, ?2, ?3, ?4, ?5);",
            params![space_str, phi, phase_vel, q_bytes, p_bytes],
        )?;

        Ok(())
    }
}
