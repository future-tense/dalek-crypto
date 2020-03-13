# dalek-crypto

Javascript bindings to *some* of the functionality in the brilliant `dalek cryptography`<sup>[1](#note1)</sup> set of Rust libraries

### Requirements

* Rust (https://www.rust-lang.org/tools/install)
* a target CPU with AVX2 extensions

### Installing and compiling

```
npm install @futuretense/dalek-crypto
```

### Benchmarks

||ed25519<sup>[2](#note2)</sup>|sodium-native<sup>[3](#note3)</sup>|dalek|dalek w/ AVX2|
|:--|--:|--:|--:|--:|
|Generate|41,565|23,172|25,099|23,861|
|Sign|43,173|24,548|26,873|25,565|
|Verify|121,710|75,394|73,841|46,266|
*Time in ms; 1,000,000 calls, on a 3GHz MBP*


Notes:<br>

<br><a name="note1"> 1) https://github.com/dalek-cryptography/
</a><br>
<br><a name="note2"> 2) https://www.npmjs.com/package/ed25519</a><br>
<br><a name="note3"> 3) https://www.npmjs.com/package/sodium-native</a><br>
