RUSTC ?= rustc

.PHONY: giffy_stardust

giffy_stardust:
	$(RUSTC) -o $@ -L src src/main.rs
