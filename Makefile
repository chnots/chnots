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
	cd $(WEB_DIR) && $(PNPM_INSTALL) && pnpm run dev

run-server-sqlite:
	cd $(SERVER_DIR) && cargo run -- --config ../config/config.sqlite.toml

run-server-postgres:
	cd $(SERVER_DIR) && cargo run -- --config ../config/config.postgres.toml

sync-struct:  
	python tools/sync-struct.py
	cd $(WEB_DIR) && $(PNPM_INSTALL)
	find -name 'dto.ts' -o -name 'po.ts' |  xargs ./web/node_modules/.bin/prettier --ignore-unknown --write

build-server:
	cd $(SERVER_DIR) && $(CARGO_BUILD)

build-web-dir:
	mkdir -p $(WEB_DIR)/public/static/favicon/ && cp data/icon/chnots.svg $(WEB_DIR)/public/static/favicon/
	cd $(WEB_DIR) && $(PNPM_INSTALL) && $(PNPM_BUILD)

build-web: build-web-dir

build: sync-struct build-web build-server

build-tauri-android:
	test -f web-dist/index.html && cd ./tauri && pnpm tauri android build --target aarch64

full-build: sync-struct build

install:
	cd $(SERVER_DIR) && cargo install --path .

clean-web-dist:
	rm -rf $(WEB_DIST)

clean-all:
	clean-web-dist
	cd $(SERVER_DIR) && $(CARGO_CLEAN)
	cd $(WEB_DIR) && pnpm run clean

