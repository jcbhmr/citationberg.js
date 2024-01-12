build-release:
	wasm-pack build --target web --release --out-dir ../target/js/release citationberg --features "wasm-bindgen"
	$(MAKE) apply-js-release

build:
	wasm-pack build --target web --dev --out-dir ../target/js/debug citationberg --features "wasm-bindgen"
	$(MAKE) apply-js

apply:
	git -C citationberg apply "$$PWD/citationberg.patch"

diff:
	git -C citationberg diff > citationberg.patch

apply-js-release:
	(cd target/js/release; patch -p1 < ../../js.patch)

apply-js:
	(cd target/js/debug; patch -p1 < ../../js.patch)

diff-js:
	rm -rf target/js/debug
	$(MAKE) build
	git -C target/js/debug init
	git -C target/js/debug add -Af
	git -C target/js/debug commit -m "Initial commit"
	touch js.patch
	(cd target/js/debug; patch -p1 < ../../js.patch)
	read -p "Press enter to continue" x
	git -C target/js/debug diff > js.patch
