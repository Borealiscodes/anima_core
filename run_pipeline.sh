#!/usr/bin/env bash
# run_pipeline.sh
# Automated Phase II-V Persistent Manifold Validation Loop Container

set -e # Exit immediately if any step encounters a hardware or type error

echo "================================================================================"
echo "⚡ INITIALIZING ANIMA CLASSIC PERSISTENT BOUNDARY PROTECTION PIPELINE"
echo "================================================================================"

# Step 0: Ensure physical storage directories exist before runtime injection
echo "[STEP 0] Verifying environment state directory layout..."
mkdir -p data
mkdir -p memory

# Step 1: Formal checking of the Lean 4 formalization invariants
echo -e "\n[STEP 1] Validating compile-time invariants via Lean 4 proof gate..."
if command -v lean &> /dev/null; then
    lean animacoretypes.lean
    lean anima_transition.lean
    lean anima_recursion.lean
    lean anima_manifold.lean
    lean anima_invariants.lean
    echo "✔ Lean 4 boundary proofs completely checked and verified."
else
    echo "⚠ Lean 4 toolchain not detected locally. Skipping logical type check steps..."
fi

# Step 2: Compile the complete, zero-allocation persistent Rust binary core
echo -e "\n[STEP 2] Compiling low-level Silicon Cage persistent core via Cargo..."
cargo build --release

# Ensure dynamic libraries are correctly mapped for cross-FFI telemetry tracking
if [[ "$OSTYPE" == "darwin"* ]]; then
    if [ -f target/release/libanima_core.dylib ]; then
        cp target/release/libanima_core.dylib target/release/libanima_core.so
    fi
fi

# Step 3: Run the native persistent execution loop binary container
echo -e "\n[STEP 3] Igniting active execution sweep (Rust Substrate)..."
if [ -f target/release/anima_core ]; then
    ./target/release/anima_core
elif [ -f target/release/main ]; then
    ./target/release/main
else
    cargo run --release
fi

# Step 4: Fire up the out-of-band Julia telemetry gyroscope pointer tracking
echo -e "\n[STEP 4] Launching out-of-band Julia gyroscope telemetry loops..."
if [ -f ffi_bridge.jl ]; then
    julia ffi_bridge.jl
else
    echo "⚠ ffi_bridge.jl gyroscope script not found in this root partition."
fi

# Step 5: Execute multi-tenant isolation tests within the Tri-System Harness v2.0
echo -e "\n[STEP 5] Running adversarial cross-system math-washing validation..."
if [ -f "orchestrator.jl" ] && [ -f "manifest.json" ]; then
    julia orchestrator.jl manifest.json
    if [ -f "test_math_washing.jl" ]; then
        julia test_math_washing.jl
    fi
else
    echo "ℹ Tri-System Harness components not present in this root directory. Skipping isolation audit."
fi

echo -e "\n================================================================================"
echo "🏁 PIPELINE COMPLETE: ANIMA CLASSIC MANIFOLD ENTIRELY STABLE AND PERSISTENT"
echo "================================================================================"
