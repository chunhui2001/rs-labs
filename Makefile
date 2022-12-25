
### 当前 Makefile 文件物理路径
ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

P := lab1

build:
	cargo build

clear:
	rm -rf ./*/src/tmp*
	rm -rf ./*/src/*/tmp*
	cargo clean

### make run P=lab1
run: clear fmt
	cargo run -p $(P)

test:
	cargo test -p $(P)

fmt:
	cargo fmt

new:
	cargo new $(P) --bin

newlab:
	cargo new $(P) --lib
