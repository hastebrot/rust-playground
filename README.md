# rust-playground

## hello-llvm

Dependencies.

* [llvm-sys - crates.io: Rust Package Registry](https://crates.io/crates/llvm-sys)

Install external dependencies.

~~~console
$ brew install llvm
...

$ brew --prefix llvm
/usr/local/opt/llvm
~~~

Run binary of the package.

~~~console
$ LLVM_SYS_100_PREFIX=/usr/local/opt/llvm cargo run --bin hello_llvm
~~~

