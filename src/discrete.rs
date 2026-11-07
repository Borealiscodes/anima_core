// anima_core/src/discrete.rs
//! Phase II Architecture: Zero-Multiplier Ternary SWAR Bitplane Contractions

/// Fixed storage block size for 1.58-bit compressed architectural fields.
pub const BITPLANE_LANES: usize = 4; // Tracks 256 state components over 64-bit blocks

/// Compressed 1.58-bit Ternary Vector representation utilizing dual-bitplane SWAR packaging.
/// Maps values explicitly to:
///   (Sign, Magnitude) -> (0, 0) =  0 (Quiescent baseline)
///   (Sign, Magnitude) -> (0, 1) =  1 (Positive alignment/excitation)
///   (Sign, Magnitude) -> (1, 1) = -1 (Adversarial resistance/inversion)
#[repr(C, align(32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TernaryBitplaneVector {
    pub sign: [u64; BITPLANE_LANES],
    pub magnitude: [u64; BITPLANE_LANES],
}

impl TernaryBitplaneVector {
    /// Zero-allocator baseline setup.
    #[inline(always)]
    pub fn new_quiescent() -> Self {
        TernaryBitplaneVector {
            sign: [0; BITPLANE_LANES],
            magnitude: [0; BITPLANE_LANES],
        }
    }

    /// High-performance SWAR inner product contraction over the 6-axis clean manifold fields.
    /// Employs parallel popcount loops to eliminate float multipliers entirely.
    #[inline(always)]
    pub fn swar_dot_product(&self, other: &Self) -> i32 {
        let mut matching_positives: u32 = 0;
        let mut matching_negatives: u32 = 0;

        for i in 0..BITPLANE_LANES {
            // Find active intersecting fields between both matrices
            let shared_mag = self.magnitude[i] & other.magnitude[i];

            // Evaluate sign alignments (XOR handles opposite directions, XNOR/NOT handles matching)
            let sign_diff = self.sign[i] ^ other.sign[i];
            
            // Extract population metrics natively within single-cycle CPU registers
            matching_positives += (shared_mag & !sign_diff).count_ones();
            matching_negatives += (shared_mag & sign_diff).count_ones();
        }

        (matching_positives as i32) - (matching_negatives as i32)
    }
}
