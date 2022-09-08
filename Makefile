CARGO          = cargo
CARGO_CCARGS   = 


################################################################################
# Main goals
ci: CARGO_CCARGS = --all-features  --all
ci: test build lint

sync: README.md

test:
	$(CARGO) test $(CARGO_CCARGS)

lint: lint/clippy lint/fmt

fmt: 
	$(CARGO) fmt
	
build: build/crate
################################################################################
build/crate: 
	$(CARGO) build $(CARGO_CCARGS)

lint/clippy:
	$(CARGO) clippy $(CARGO_CCARGS)

lint/fmt:
	$(CARGO) fmt --check

################################################################################
# Make targets
README.md: src/main.rs
	@echo Updating $@
	$(CARGO) sync-readme

################################################################################

.PHONY: sync test \
        $(filter build%, $(MAKECMDGOALS)) \
        $(filter lint%, $(MAKECMDGOALS)) 
