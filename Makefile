RUSTC ?= rustc

.PHONY: giffy_stardust

giffy_stardust:
	$(RUSTC) -o $@ src/main.rs
