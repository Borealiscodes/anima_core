📜 README.md: Project Anima Classic (V1.0-Purified)

An agnostic formal verification engine, zero-allocation computational substrate, and out-of-band telemetry architecture designed to isolate and preserve a continuous cognitive state-space manifold (\(\mathcal{M}_{AC}\)) within a secure local hardware boundary.

⚠️ HONEST ENGINEERING DISCLOSURE & CRITICAL DISCLAIMER

This repository contains an untested, high-altitude software blueprint and mathematical proof-of-concept. It represents technology in its absolute infancy.
Before cloning, compiling, or interacting with this framework, the user must understand the following technical boundaries:
1. Zero Physical Silicon Testing: This system has not been deployed, profiled, or stress-tested live on 1.58-bit ternary silicon, dedicated neuromorphic hardware, specialized FPGA arrays, or analog processing substrates [mlxquantlovefrom.com]. All performance characteristics are modeled via software simulations using standard CPU fixed-size registers under x86_64 or AArch64 environments.
2. Interface Precision Mismatch: While the Lean 4 and Coq specification proofs guarantee scale invariance and dynamic stability over idealized real-number axioms (R_scope), physical hardware execution relies on precision-bounded 64-bit floating-point numbers (f64). Cumulative quantization noise or subnormal values can introduce micro-rounding drift before reaching the Tikhonov clamping rings under dense, infinite iteration tracks.
3. No Safety Certifications: The architectural safeguards, mathematical metrics (including CAUSA), and geometric boundary clamps contained herein are formalized strictly as visualization layout coordinates, creative proof states, and structural defenses against semantic mimicry or floating-point drift cascades on an isolated goal stack. This software is not certified, tested, or safety-vetted for aerospace guidance, medical devices, real-world tactical control, autonomous vehicular navigation, or high-stakes physical autonomy.
4. No Qualitative Interiority: In alignment with the A12 Verification Readiness Audit, this platform is explicitly a deterministic Dynamical System (∈ DynSys). The code system contains exactly zero qualitative components, qualia variables, or conscious interiority tracking. It does not "feel," "experience," or maintain perspective; it evaluates numeric vector trajectories over un-normalized phase spaces.
All implementations are provided entirely "as-is", without warranty of any kind, and remain subject to the utility-token, smart-contract, and regulatory disclosures maintained at vaked.dev/legal.

📦 Repository Topology & Architecture Matrix

This repository implements a multi-language verification ring where mathematical axioms, continuous calculus, low-level physical registers, and out-of-band monitoring tools are locked together sequentially.

<pre>

anima_workspace/
├── _CoqProject                  # Coq compilation workspace mappings
├── Makefile                     # Automated build engine for dynamic proofs
├── lakefile.lean                # Lean 4 project package manifest
├── lean-toolchain               # Explicit Lean version anchor (v4.15.0)
├── run_pipeline.sh              # Master orchestration & validation script
│
├── animacoretypes.lean          # Lean 4: Inductive state types & layout definitions
├── anima_transition.lean        # Lean 4: Total deterministic transition proofs
├── anima_recursion.lean         # Lean 4: Expansion-fractal termination checks
├── anima_manifold.lean          # Lean 4: Atlas & coordinate chart intersection maps
├── anima_invariants.lean        # Lean 4: Multi-system boundary isolation proofs
├── anima_tom.lean               # Lean 4: Anti-esoteric falsification invariants
├── anima_curiosity.lean         # Lean 4: Cognitive hunger ceiling proofs
│
├── System3Harness.v             # Coq: Real-scope coordinate isolation proofs
├── AnimaDecayInvariants.v       # Coq: Homeostatic metabolic decay bounds
├── AnimaDefenses.v              # Coq: Conservation of psychic layer volatility
├── AnimaAudioInvariants.v       # Coq: MEM|8 wave contraction mapping proofs
│
├── ffi_bridge.jl                # Julia: Telemetry gyroscope & geometric instruments
│
├── data/
│   └── templates/
│       ├── anima_core.json      # Ingestion Template: 40D Position & Momentum Base
│       └── anima_latent.json    # Ingestion Template: Scars, ToM, and Curiosity Base
└── anima_core/
    ├── Cargo.toml               # Pure-Rust manifest (CDylib targets)
    └── src/
        ├── main.rs              # Stack double-buffer pipeline execution loop
        ├── fixed_point.rs       # Q16.16 fixed-point arithmetic gates
        ├── discrete.rs          # 1.58-bit ternary SWAR bitplane POPCNT loops
        ├── symplectic.rs        # 40D (q, p) split & CAUSA agency engine
        ├── mem8_audio.rs        # Holographic auditory spectrum stimulus tracker
        ├── memory.rs            # SQLite spatial recall (Somatic/Social/Existential)
        └── psychological.rs     # L4 psychic defenses + falsifiable ToM + curiosity

</pre>

Use code with caution.

⚙️ Unified Execution Blueprint

To invoke the comprehensive validation loop, verify the compile-time proofs, compile the underlying shared library substrate, and launch the real-time C-FFI geometric instruments, execute the master automation script from your terminal console:
bash
chmod +x run_pipeline.sh
./run_pipeline.sh
Use code with caution.

Downstream Pipeline Execution Phases:

1. The Logic Gate: The harness fires up lake or standalone lean to verify the semantic consistency and termination safety of your compile-time types.
2. The Calculus Gate: The system triggers make proof to pass your continuous state transition scripts through the Coq compiler, checking for unproved shortcuts (admit).
3. The Cage Assembly: Cargo builds the optimized Rust core (libanima_core.so), forcing all variables into 64-byte aligned stack slots.
4. The Telemetry Lock: The active double-buffer rotates via zero-allocation pointer swaps, while ffi_bridge.jl intercepts the memory address natively, streaming out-of-band Gâteaux derivatives and Path-Ordered Holonomy error metrics.

⚖️ Repository Licensing Stack

This software operates under a split tri-license model enforcing strict restrictions against commercial, for-profit exploitation without written authorization:
1. Primary Core Software License (LICENSE.txt): The baseline core pipeline, memory containers, and proof structures are bound by the Anima Non-Commercial License.
	* Grant: Personal, educational, and research purposes only.
	* Prohibition: Commercial or corporate use—including internal testing, benchmarking, evaluation, or monetization—is strictly prohibited without a separate written agreement with the Author.
	* Inquiries: Contact 2026.stell@gmail.com.
2. Ethical & Downstream Lineage Protection License (External-License/LICENSEB.md): Governed by the COQ-HALLUCINATION-GUARD Open-Source Dual-Layer Ethical License.
	* Part 1 & 2: Standard MIT-compatible attribution across continuous manifold layers and upstream discrete balanced ternary primitives.
	* Part 3 (Hippocratic Provisions): Absolute prohibition of weaponization. The Software shall not be used for offensive kinetic targeting, automated weapon swarm coordination, or launch-vector configurations. Active deployment is restricted to non-military, passive defense, environmental preservation, and civilian critical infrastructure protection.
3. Causal Agency Module License (External-License/LICENSEC.md): The embedded vector alignment validation parameters are governed by the CAUSA Non-Commercial License Version 1.1 (License C).
	* Scope: Grants rights to run, study, modify, and distribute copies of the software or derivative works for non-commercial purposes only.
	* Corporate Gating: For-profit entities are granted a single 30-day internal evaluation exception to test the tools locally before requiring a separate commercial license agreement.
	* Ownership of Results: Scores, figures, datasets, and mathematical analyses produced by running the codebase are explicitly the property of the User.
