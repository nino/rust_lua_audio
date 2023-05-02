.PHONY: build
build:
	cargo build --release
	rm -f ./lua/my_module.so || true
	cp ./target/release/librust_lua_audio.dylib ./lua/my_module.so
	# if your Rust project has dependencies,
	# you'll need to do this as well
	mkdir -p ./lua/deps/
	cp ./target/release/deps/*.rlib ./lua/deps/
