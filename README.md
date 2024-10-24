# Building a Calculator of Wasm Components

This tutorial walks through how to compose a component to build a Wasm calculator.
The WIT package for the calculator consists of a world for each mathematical operator
add an `op` enum that delineates each operator. The following example interface only
has an `add` operation:

```wit
package docs:calculator@0.1.0;

interface calculate {
    enum op {
        add,
    }
    eval-expression: func(op: op, x: u32, y: u32) -> u32;
}

interface add {
    add: func(a: u32, b: u32) -> u32;
}

world adder {
    export add;
}

world calculator {
    export calculate;
    import add;
}
```

To expand the exercise to add more components, add another operator world, expand the enum, and modify the `command` component to call it.

## Prerequisites

- [`rust and cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools)
- [`wac`](https://github.com/bytecodealliance/wac)
- [`wasmtime`](https://wasmtime.dev/)

## Building and running the example

To compose a calculator component with an add operator, run the following:

```sh
(cd calculator && cargo component build --release)
(cd adder && cargo component build --release)
(cd subtractor && cargo component build --release)
(cd multiplier && cargo component build --release)
(cd divisor && cargo component build --release)
(cd squarer && cargo component build --release)
(cd command && cargo component build --release)
```

```sh
cd exp
wit-bindgen c --autodrop-borrows yes --world exponential  ../wit/calculator.wit
```

For the C component

```sh
/opt/wasi-sdk/bin/clang exp.c exponential.c exponential_component_type.o -o exponential-core.wasm -mexec-model=reactor
wasm-tools component new ./exponential-core.wasm -o exponential-component.wasm
wasm-tools component wit exponential-component.wasm
```

Create the composed and the command

```sh
wac plug calculator/target/wasm32-wasi/release/calculator.wasm --plug adder/target/wasm32-wasi/release/adder.wasm --plug subtractor/target/wasm32-wasi/release/subtractor.wasm --plug multiplier/target/wasm32-wasi/release/multiplier.wasm --plug divisor/target/wasm32-wasi/release/divisor.wasm --plug squarer/target/wasm32-wasi/release/squarer.wasm -o composed.wasm
```

```sh
wac plug command/target/wasm32-wasi/release/command.wasm --plug composed.wasm -o command.wasm
```

## Running with wasmtime

You must have wasmtime for this to work. Make sure to follow the build step above first.

```sh
wasmtime run --wasm component-model command.wasm 1 2 add
1 + 2 = 3
wasmtime run --wasm component-model command.wasm 6 2 sub
6 - 2 = 4
wasmtime run --wasm component-model command.wasm 6 2 add
6 + 2 = 8
wasmtime run --wasm component-model command.wasm 6 2 mul
6 * 2 = 12
wasmtime run --wasm component-model command.wasm 6 2 div
6 / 2 = 3
wasmtime run --wasm component-model command.wasm 6 2 exp
6 ^ 2 = 36
wasmtime run --wasm component-model command.wasm 6 2 square
6 ² = 36
```
