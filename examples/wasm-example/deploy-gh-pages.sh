SUBTREE_OUTPUT=`git subtree split --prefix examples/wasm-example/gh-pages/ master`
[ $? != 0 ] && exit 1
git push origin "$SUBTREE_OUTPUT":gh-pages --force
