set dotenv-load

default:
	just --list --unsorted


publish:
	cargo publish --allow-dirty --no-verify

run example:
	cargo run --example {{example}}


# build wasm mega command, upstream changest beetmash_template
build-wasm:
	just _build-wasm sewb app ./target/wasm

_build-wasm crate example wasm-dir *args:
	cargo build \
	--target wasm32-unknown-unknown \
	--example {{example}} \
	--release {{args}}
	rm -rf {{wasm-dir}} || true
	mkdir -p {{wasm-dir}} || true
	wasm-bindgen \
	--out-name main \
	--out-dir {{wasm-dir}} \
	--target web \
	$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/examples/{{example}}.wasm \
	--no-typescript
	mkdir -p /home/pete/me/beetmash-api/target/storage/apps/{{crate}} || true
	cp -r {{wasm-dir}}/* /home/pete/me/beetmash-api/target/storage/apps/{{crate}}
	just wasm-opt {{wasm-dir}}

@wasm-opt wasm-dir:
	just list-size {{wasm-dir}} "pre wasm-opt\t"
	wasm-opt -Oz --output target/optimized.wasm target/wasm/main_bg.wasm
	mv target/optimized.wasm target/wasm/main_bg.wasm
	just list-size {{wasm-dir}} "post wasm-opt\t"

@list-size wasm-dir prefix:
	file_size_bytes=$(stat -c%s "{{wasm-dir}}/main_bg.wasm") \
	&& file_size_mb=$(echo "scale=2; $file_size_bytes / 1024 / 1024" | bc) \
	&& echo "{{prefix}} $file_size_mb MB"
