# The various representations of WASM

https://webassembly.github.io/wabt/demo/wat2wasm/







## Text representation - S-expressions

./my-function.wat

```
(module
  (func $addTwo (param i32 i32) (result i32)
    get_local 0
    get_local 1
    i32.add)
  (export "addTwo" (func $addTwo)))
```








## Binary representation

./my-function.wasm

#### Hex:
00 61 73 6d 01 00 00 00 01 07 01 60 02 7f 7f 01
7f 03 02 01 00 07 0a 01 06 61 64 64 54 77 6f 00
00 0a 09 01 07 00 20 00 20 01 6a 0b 00 19 04 6e
61 6d 65 01 09 01 00 06 61 64 64 54 77 6f 02 07
01 00 02 00 00 01 00

#### Binary:
000000001100001011100110110110100000001000000000000000000000000000000010000011100000001011000000000001001111111011111110000000101111111000000110000001000000001000000000000011100001010000000010000011001100001011001000110010001010100011101110110111100000000000000000000101000001001000000010000011100000000001000000000000000100000000000010110101000001011000000000001100100000100011011100110000101101101011001010000000100001001000000010000000000000110011000010110010001100100010101000111011101101111000000100000011100000001000000000000001000000000000000000000000100000000

#### Annotated:
```
|-------|-----------------------------------------|---------------------|
|Address| Instructions                            | Comments            |
|-------|-----------------------------------------|---------------------|
0000000: 0061 736d                                 ; WASM_BINARY_MAGIC
0000004: 0100 0000                                 ; WASM_BINARY_VERSION

; section "Type" (1)
0000008: 01                                        ; section code
0000009: 00                                        ; section size (guess)
000000a: 01                                        ; num types

; type 0
000000b: 60                                        ; func
000000c: 02                                        ; num params
000000d: 7f                                        ; i32
000000e: 7f                                        ; i32
000000f: 01                                        ; num results
0000010: 7f                                        ; i32
0000009: 07                                        ; FIXUP section size

; section "Function" (3)
0000011: 03                                        ; section code
0000012: 00                                        ; section size (guess)
0000013: 01                                        ; num functions
0000014: 00                                        ; function 0 signature index
0000012: 02                                        ; FIXUP section size

; section "Export" (7)
0000015: 07                                        ; section code
0000016: 00                                        ; section size (guess)
0000017: 01                                        ; num exports
0000018: 06                                        ; string length
0000019: 6164 6454 776f                    addTwo  ; export name
000001f: 00                                        ; export kind
0000020: 00                                        ; export func index
0000016: 0a                                        ; FIXUP section size

; section "Code" (10)
0000021: 0a                                        ; section code
0000022: 00                                        ; section size (guess)
0000023: 01                                        ; num functions

; function body 0
0000024: 00                                        ; func body size (guess)
0000025: 00                                        ; local decl count
0000026: 20                                        ; get_local
0000027: 00                                        ; local index
0000028: 20                                        ; get_local
0000029: 01                                        ; local index
000002a: 6a                                        ; i32.add
000002b: 0b                                        ; end
0000024: 07                                        ; FIXUP func body size
0000022: 09                                        ; FIXUP section size

; section "name"
000002c: 00                                        ; section code
000002d: 00                                        ; section size (guess)
000002e: 04                                        ; string length
000002f: 6e61 6d65                           name  ; custom section name
0000033: 01                                        ; function name type
0000034: 00                                        ; subsection size (guess)
0000035: 01                                        ; num functions
0000036: 00                                        ; function index
0000037: 06                                        ; string length
0000038: 6164 6454 776f                    addTwo  ; func name 0
0000034: 09                                        ; FIXUP subsection size
000003e: 02                                        ; local name type
000003f: 00                                        ; subsection size (guess)
0000040: 01                                        ; num functions
0000041: 00                                        ; function index
0000042: 02                                        ; num locals
0000043: 00                                        ; local index
0000044: 00                                        ; string length
0000045: 01                                        ; local index
0000046: 00                                        ; string length
000003f: 07                                        ; FIXUP subsection size
000002d: 19                                        ; FIXUP section size
```

## Calling our code

```js
const obj = await WebAssembly.instantiateStreaming(fetch('add-function.wasm'));
obj.instance.exports.addTo(1, 1)
// 2
```
