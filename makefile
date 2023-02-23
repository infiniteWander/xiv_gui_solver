clean:
	@echo "[MAKE] Cleaning last release"
	cargo clean
	rm -rf releases
	rm -rf python/build
	rm -rf python/dist

rust:
	@echo "[MAKE] Building the libraries"
	maturin develop --release -q
	cargo build --release -q # --features="no_python"

python:
	@echo "[MAKE] Building the python project"
	cd python && rm -rf build dist
	cd python && pip install -q -r requirements_dev.txt
	cd python && pip install -q -e .
	cd python && pyinstaller src/ffcraft_solver/__main__.py -i "src/ffcraft_solver/ressources/ffcraft_solver.ico" --add-data "src/ffcraft_solver/configs/:ffcraft_solver/configs" --add-data  "src/ffcraft_solver/ressources/:ffcraft_solver/ressources" --name ffcraft_solver_gui  -y > /dev/null 2>pyinstaller_log.txt 



.PHONY: clean rust python
release: clean rust python
	mkdir -p releases # Add version as an arg and things
	@echo "[MAKE] Copying releases in the releases/ directory"
	zip releases/ffcraft_solver_cli.zip -q -j target/release/ffcraft_solver_cli craft.toml -j target/release/libxiv_craft_solver.so 
	cd python/dist/ && zip -q ../../releases/ffcraft_solver_gui.zip -r ffcraft_solver_gui/