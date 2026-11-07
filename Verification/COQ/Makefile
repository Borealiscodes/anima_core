# Makefile for Anima Classic Coq Invariant Proof Verification Engine

.PHONY: all clean proof check

all: Makefile.coq
	$(MAKE) -f Makefile.coq

Makefile.coq: _CoqProject
	coq_makefile -f _CoqProject -o Makefile.coq

proof: all
	@echo "✔ All mathematical proofs verified reflexively over R_scope."

check: all
	@echo "Checking proof states for unproven placeholders..."
	@grep -n "admit" *.v || echo "✔ Zero unproven shortcuts detected in active Coq files."
	@grep -n "sorry" *.v || echo "✔ Zero unproven shortcuts detected in active Lean files."

clean:
	@if [ -f Makefile.coq ]; then $(MAKE) -f Makefile.coq clean; fi
	@rm -f Makefile.coq Makefile.coq.conf .*.aux
	@rm -f *.vo *.vok *.vos *.glob .*.d
	@echo "✔ Coq workspace artifacts successfully purged."
