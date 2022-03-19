# prerequisites packages (Ubuntu): build-essential, pkg-config, libssl-dev

#command -v wasm-pack || cargo install wasm-pack --version 0.10.2
#cargo install wasm-bindgen-cli --version 0.2.79 && \

rm -rf pkg
wasm-pack build --target web || exit 1

for f in src/wasm-runner/*
do
	minify "$f" -o pkg/"$(basename "$f")"
done
for f in pkg/*.js
do
	minify "$f" -o "$f"
done
