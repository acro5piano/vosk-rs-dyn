# vosk-rs-dyn

Vosk binding using libloading

# Setup

```
./bin/download-vosk.sh
./bin/download-model.sh # TODO
```

# Run

```
cargo build
LD_LIBRARY_PATH=./lib/vosk cargo
```

# Generate Rust Binding for Vosk

```
cargo install bindgen
bindgen --dynamic-link-require-all --dynamic-loading Vosk lib/vosk/vosk_api.h > src/vosk_binding.rs
```
