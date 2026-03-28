# --- BUILD LIBRARY ---
```bashrustc \
  +nightly \
  --edition 2024 \
  --crate-type rlib \
  --crate-name arx_engine \
  --target x86_64-unknown-linux-gnu \
  -C opt-level=3 \
  -C lto=fat \
  -C codegen-units=1 \
  -C panic=abort \
  -C strip=symbols \
  -C target-cpu=x86-64 \
  -C force-frame-pointers=no \
  -C llvm-args=--inline-threshold=9999 \
  -C no-redzone=yes \
  -C relocation-model=static \
  -C code-model=small \
  -C no-vectorize-loops \
  -C no-vectorize-slp \
  -C link-dead-code=no \
  -C overflow-checks=no \
  -C debug-assertions=no \
  -Z merge-functions=aliases \
  -Z plt=no \
  -Z relax-elf-relocations=yes \
  -Z share-generics=no \
  -Z mir-opt-level=4 \
  -Z inline-mir=yes \
  -Z inline-mir-threshold=9999 \
  -Z trap-unreachable=yes \
  -Z fewer-names=yes \
  arx_engine.rs
```

# --- BUILD EXAMPLE ---
```bashrustc \
  +nightly \
  --edition 2024 \
  --crate-type bin \
  --extern arx_engine=libarx_engine.rlib \
  --target x86_64-unknown-linux-gnu \
  -C opt-level=3 \
  -C lto=fat \
  -C codegen-units=1 \
  -C panic=abort \
  -C strip=symbols \
  -C target-cpu=x86-64 \
  -C force-frame-pointers=no \
  -C llvm-args=--inline-threshold=9999 \
  -C no-redzone=yes \
  -C relocation-model=static \
  -C code-model=small \
  -C no-vectorize-loops \
  -C no-vectorize-slp \
  -C overflow-checks=no \
  -C debug-assertions=no \
  -C link-arg=-nostdlib \
  -C link-arg=-static \
  -C link-arg=-Wl,--gc-sections \
  -C link-arg=-Wl,--as-needed \
  -C link-arg=-Wl,-O2 \
  -C link-arg=-Wl,--build-id=none \
  -C link-arg=-Wl,-z,now \
  -C link-arg=-Wl,-z,relro \
  -C link-arg=-Wl,-z,noexecstack \
  -Z merge-functions=aliases \
  -Z plt=no \
  -Z relax-elf-relocations=yes \
  -Z mir-opt-level=4 \
  -Z inline-mir=yes \
  -Z inline-mir-threshold=9999 \
  -Z fewer-names=yes \
  main.rs -o arx
```
