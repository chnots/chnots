build:
	cd web && pnpm run build
	cd server && cargo build --release

build-web:
	cd vendor/turndown && git clean -xfd && pnpm install && pnpm run build
	cd web && git clean -xfd && pnpm install && pnpm run build

install:
	cd web && pnpm run build
	cd server && cargo install --path .
clean:
	rm -rf web-dist

clean-all:
	rm -rf web-dist
	cd server && cargo clean
	cd web && pnpm run clean