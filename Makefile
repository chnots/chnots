WEB_DIST = web-dist
WEB_DIR = web
SERVER_DIR = server

run-web:
	cd $(WEB_DIR) && pnpm install && pnpm run dev

run-server-sqlite:
	cd $(SERVER_DIR) && cargo run -- --config ../config/config.sqlite.toml

run-server-postgres:
	cd $(SERVER_DIR) && cargo run -- --config ../config/config.postgres.toml

init:
	git submodule update --init --recursive

sync-struct:  
	cargo install cargo-expand
	python tools/sync-struct.py
	cd $(WEB_DIR) && pnpm install 
	find -name 'dto.ts' -o -name 'po.ts' |  xargs ./web/node_modules/.bin/prettier --ignore-unknown --write

build-web:
	mkdir -p $(WEB_DIR)/public/static/favicon/ && cp data/icon/chnots.svg $(WEB_DIR)/public/static/favicon/
	cd $(WEB_DIR) && pnpm install && pnpm run build

build-server:
	test -f web-dist/index.html && cd $(SERVER_DIR) && cargo build 

build-tauri-desktop:
	test -f web-dist/index.html && cd ./tauri && pnpm install && pnpm tauri build

build-tauri-android:
	test -f web-dist/index.html && cd ./tauri && pnpm install && pnpm tauri android build --target aarch64
