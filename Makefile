build:
	wasm-pack build --target web --dev
	rm -rf pkg/.git
	git -C pkg init
	find pkg -type f -name '*.{orig,rej}' -delete
	git -C pkg add -Af
	git -C pkg commit -m "Initial commit"
	git -C pkg apply ../pkg.patch

setup:
	rustup target add wasm32-unknown-unknown
	command -v cargo-binstall || curl -L --proto '=https' --tlsv1.2 -sSf https://github.com/jcbhmr/cargo-binstall/raw/patch-1/install-from-binstall-release.sh | bash
	command -v wasm-pack || cargo binstall wasm-pack -y

diff: build
	git -C pkg add -Af
	git -C pkg diff --staged > pkg.patch

publish: build
	npm publish ./pkg

clean:
	rm -rf pkg target
