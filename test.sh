#!/usr/bin/env bash

function _myshuf {
    shuf --random-source=Cargo.toml -i 1-$1
}

exit_code=0

diff -aruN <(wc -l Cargo.toml | sed 's/ .*//') <(./target/release/count < Cargo.toml) || exit_code=1
diff -aruN <(_myshuf 100 | awk '{x+=$1} END {print(x/100)}') <(_myshuf 100 | ./target/release/mean) || exit_code=1
diff -aruN <(_myshuf 6 | awk 'BEGIN {x=1}{x*=$1} END {print(x)}') <(_myshuf 6 | ./target/release/product) || exit_code=1
diff -aruN <(_myshuf 100 | awk '{x+=$1} END {print(x)}') <(_myshuf 100 | ./target/release/sum) || exit_code=1

exit $exit_code
