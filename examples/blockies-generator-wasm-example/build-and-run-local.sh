# prerequisites packages (Ubuntu): build-essential, pkg-config, libssl-dev

command -v wasm-pack || cargo install wasm-pack
command -v wasm-opt || cargo install wasm-opt
command -v wasm-bindgen || cargo install wasm-bindgen-cli --version "0.2.87"

rm -rf pkg
rm -rf gh-pages
mkdir -p gh-pages

wasm-pack build --target web || exit 1
for f in pkg/*.wasm
do
	OPT_OPTS="-Oz"
	PASS_OPTS="--vacuum --flatten --rereloop --reorder-functions --strip-target-features --strip-producers --code-folding --gufa-optimizing"
	wasm-opt "$f" $PASS_OPTS $OPT_OPTS -o - | tee gh-pages/"$(basename "$f")" | wc -c
done

for f in src/wasm-runner/*
do
	minify "$f" -o pkg/"$(basename "$f")"
done
for f in pkg/*.js
do
	minify "$f" -o "$f"
done



# copy binary & asset to gh-pages

cp -af pkg/*.html gh-pages
cp -af pkg/*.css gh-pages
cp -af pkg/*.js gh-pages
cp -af pkg/*.wasm gh-pages
cp -af asset/* gh-pages


# run
cd gh-pages
python3 -m http.server 0 --bind localhost
