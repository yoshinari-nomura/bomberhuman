#!/bin/bash

package=www
wasm-pack build
(cd $package && npm run build && npm run start -- --port 9000)
echo "open http://localhost:9000/"
