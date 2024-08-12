build:
	cd web && pnpm run build
	cd server && cargo build --release

clean:
	rm -rf web-dist

clean-all:
	rm -rf web-dist
	cd server && cargo clean
	cd web && pnpm run clean