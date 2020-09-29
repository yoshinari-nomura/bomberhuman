# bomberhuman

## Setup for developer

### Bootstrap

``` shell
# Install node via nvm
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh | bash
nvm install node

# Install wasm-pback into ~/.cargo/bin/wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install cargo-generate
cargo install cargo-generate

# Create new bomberhuman project
cargo generate --git https://github.com/rustwasm/wasm-pack-template --name bomberhuman

# Build to see if setup is OK
cd bomberhuman; wasm-pack build

# Check pkg/ is created (pkg/ is a Node module)
ls -l pkg/

# Install wasm app under www/
npm init wasm-app www && cd www && npm install
```
