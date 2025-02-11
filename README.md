# CLAIR—Command Line Arithmetic in Rust

CLAIR is a small personal project for exploring the [Rust programming language](https://www.rust-lang.org/).
It implements arithmetic operations on lines of string (currently only stdin, but support for files as input is planned).
This is inspired by using [`awk`](https://www.gnu.org/software/gawk/manual/gawk.html) for command line arithmetics.

## Build

```shell
cargo build [--release]
```

## Run

```shell
cargo run [--release] --bin={count,mean,product,sum} [</path/to/file/as/stdin] [--help]
```
or directly form `target`:

```shell
./target/release/{count,mean,product,sum}
```

### Example
#### Count

```shell
$ ./target/release/count < Cargo.toml
24

# Reference:
$ wc -l Cargo.toml
24 Cargo.toml
```

#### Mean

```shell
$ shuf -i 1-100 | ./target/release/mean
50.5

# Reference:
$ shuf -i 1-100 | awk '{x += $1} END {print(x/100)}'
50.5
```

#### Product

```shell
$ shuf -i 1-6 | ./target/release/product
720

# Reference:
$ shuf -i 1-6 | awk 'BEGIN {x=1} {x *= $1} END {print(x)}'
720
```

#### Sum

```shell
$ shuf -i 1-100 | ./target/release/sum
5050

# Reference:
$ shuf -i 1-100 | awk '{x += $1} END {print(x)}'
5050
```

