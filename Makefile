# Paths to your cargo projects
KATANA_DIR := katana
PAPYRUS_DIR := papyrus

.PHONY: bench bench_katana bench_papyrus

bench: bench_katana bench_papyrus

bench_katana:
	@echo "Running benchmarks for Katana"
	cd $(KATANA_DIR) && cargo bench

bench_papyrus:
	@echo "Running benchmarks for Papyrus"
	cd $(PAPYRUS_DIR) && cargo bench
