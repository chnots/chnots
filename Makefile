WEB_DIST = web-dist
WEB_DIR = web
SERVER_DIR = server
TOOLS_DIR = tools
TAURI_DIR = tauri
LIB_DIR= lib

run-web:
	cd $(WEB_DIR) && pnpm install && pnpm run dev

run-server-sqlite:
	cd $(SERVER_DIR) && cargo run -- --config ../config/config.sqlite.toml

run-server-postgres:
	cd $(SERVER_DIR) && cargo run -- --config ../config/config.postgres.toml

run-tauri-desktop:
	cd $(TAURI_DIR) && pnpm tauri dev

init-workflow:
	cd $(LIB_DIR)	&& git clone https://github.com/wzhchin/chin-tools
	cd $(LIB_DIR)	&& git clone https://github.com/wzhchin/mind-elixir-core -b feat/image-controls
	echo $(TOOLS_DIR)/_impl/git-pre-commit >> .git/hooks/pre-commit
	chmod a+x .git/hooks/pre-commit
	echo $(TOOLS_DIR)/_impl/git-post-commit >> .git/hooks/post-commit
	chmod a+x .git/hooks/post-commit

sync-struct:  
	cargo install cargo-expand
	python tools/sync-struct
	cd $(WEB_DIR) && pnpm install 
	find -name 'dto.ts' -o -name 'po.ts' |  xargs ./web/node_modules/.bin/prettier --ignore-unknown --write

build-web:
	mkdir -p $(WEB_DIR)/public/static/favicon/ && cp data/icon/chnots.svg $(WEB_DIR)/public/static/favicon/
	cd lib/md-codemirror && pnpm install && pnpm run build
	cd lib/mind-elixir-core && pnpm install && pnpm run build
	cd $(WEB_DIR) && pnpm install && pnpm run build

check-web:
	test -f web-dist/index.html || make build-web

build-server:
	make check-web
	test -f web-dist/index.html && cd $(SERVER_DIR) && cargo build --release

build-tauri-desktop:
	make check-web
	test -f web-dist/index.html && cd ./tauri && pnpm install && pnpm tauri build

build-tauri-android:
	tools/check-android-key \
		&& make check-web \
		&& cd ./tauri \
		&& pnpm install \
		&& export PATH="$$PWD/node_modules/.bin/:$$PATH" \
		&& find -name tauri -type f \
		&& pnpm tauri android build --split-per-abi
