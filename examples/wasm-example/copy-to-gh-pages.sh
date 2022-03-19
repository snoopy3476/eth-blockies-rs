rm -rf gh-pages
mkdir -p gh-pages

cp -af pkg/*.html gh-pages
cp -af pkg/*.css gh-pages
cp -af pkg/*.js gh-pages
cp -af pkg/*.wasm gh-pages
