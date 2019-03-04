# ed25519-dalek

Javascript bindings to *some* of the functionality in the brilliant Rust library `ed25519-dalek`<sup>[1](#note1)</sup>

### Installing and compiling

If you have a CPU with AVX2 extensions, run

```
npm install
npm link
RUSTFLAGS='-C target-feature=+avx2' npx neon build --release
```

If not,

```
npm install
npm link
npx neon build --release
```

### Using

Go to the project where you want to use this, and run

```
npm link ed25519-dalek
```

### Benchmarks

||ed25519<sup>[2](#note2)</sup>|sodium-native<sup>[3](#note3)</sup>|dalek|dalek w/ AVX2|
|:--|--:|--:|--:|--:|
|Generate|41,565|23,172|25,099|23,861|
|Sign|43,173|24,548|26,873|25,565|
|Verify|121,710|75,394|73,841|46,266|
*Time in ms; 1,000,000 calls, on a 3GHz MBP*


Notes:<br>

<br><a name="note1"> 1) https://github.com/dalek-cryptography/ed25519-dalek</a><br>
<br><a name="note2"> 2) https://www.npmjs.com/package/ed25519</a><br>
<br><a name="note3"> 3) https://www.npmjs.com/package/sodium-native</a><br>
