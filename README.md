# wasm-sample-vue
wasm sample in vue

## setup
```sh
# build wasm
cd wasm/rust/num_ext
wasm-pack build # output in ./pkg

# install node module
cd ../../..
yarn install

# run!
yarn dev
```
