# 定义变量
PNPM_INSTALL = pnpm install
PNPM_BUILD = pnpm run build
CARGO_BUILD = cargo build --release
CARGO_CLEAN = cargo clean

# 定义清理操作
WEB_DIST = web-dist
WEB_DIR = web
SERVER_DIR = server

run-web:
	cd $(WEB_DIR) && pnpm run dev

run-server:
	cd $(SERVER_DIR) && cargo run -- --config ../config/config.example.toml

build-server:
	cd $(SERVER_DIR) && $(CARGO_BUILD)

build-web-dir:
	cd $(WEB_DIR) && git clean -xfd && $(PNPM_INSTALL) && $(PNPM_BUILD)

build-web: build-web-dir

build: build-web build-server

install:
	cd $(SERVER_DIR) && cargo install --path .

clean-web-dist:
	rm -rf $(WEB_DIST)

clean-all:
	clean-web-dist
	cd $(SERVER_DIR) && $(CARGO_CLEAN)
	cd $(WEB_DIR) && pnpm run clean

