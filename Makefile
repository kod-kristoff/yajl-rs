.DEFAULT: help

help:
	@echo "Usage:"
	@echo "    test-parsing"
	@echo "            run test-suite in C using yajl-clib"
	@echo "    test-parsing-rs"
	@echo "            run test-suite in Rust"
	@echo "    run-perftest"
	@echo "            run perftest in C using yajl-clib"
	@echo "    run-perftest-rs"
	@echo "            run perftest in Rust"
	@echo "    "
	@echo "    Examples"
	@echo "    ========"
	@echo "    run-parse-config"
	@echo "            run parse_config example in C using yajl-clib"
	@echo "    run-json-verify"
	@echo "            run json_verify example in C using yajl-clib"

RLIB = target/debug/libyajl.so
CFLAGS = -Ibuild/yajl-2.1.1/include
YAJL_TEST = build/test/parsing/yajl_test
YAJL_TEST_RS = target/debug/yajl_test

bin/parse_config: examples/parse_config.o $(RLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

bin/json_verify: verify/json_verify.o $(RLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

bin/json_reformat: reformatter/json_reformat.o $(RLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

$(YAJL_TEST): tests/parsing/yajl_test.c $(RLIB)
	$(CC) -Wall $(CFLAGS) tests/parsing/yajl_test.c -l:libyajl.so -Ltarget/debug -o $@

$(YAJL_TEST_RS): examples/yajl_test/src/main.rs examples/yajl_test/Cargo.toml
	cargo build --package yajl_test

$(RLIB): crates/yajl-clib/Cargo.toml crates/yajl-clib/src/*.rs
	cargo build --package yajl-clib

run-parse-config: bin/parse_config
	LD_LIBRARY_PATH=target/debug bin/parse_config < examples/sample.config

run-parse-config-rs:
	cargo run --package parse-config < examples/sample.config

run-json-verify: bin/json_verify
	LD_LIBRARY_PATH=target/debug bin/json_verify -c < examples/sample.config

run-json-verify-rs:
	cargo run --package json-verify -- -c < examples/sample.config

run-json-reformat: bin/json_reformat
	LD_LIBRARY_PATH=target/debug bin/json_reformat < examples/sample.config

run-json-reformat-rs:
	cargo run --package json-reformat < examples/sample.config

test-parsing: $(YAJL_TEST)
	# cd test/parsing && ./run_tests.sh
	cd tests/parsing && LD_LIBRARY_PATH=../../target/debug ./run_tests.sh

test-parsing-rs: $(YAJL_TEST_RS)
	cd tests/parsing && ./run_tests.sh "../../$(YAJL_TEST_RS)"

bin/perftest: perf/documents.o perf/perftest.o $(RLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

run-perftest: bin/perftest
	LD_LIBRARY_PATH=target/debug bin/perftest

target/debug/perftest: examples/perftest/src/*.rs examples/perftest/Cargo.toml
	cargo build --package perftest

.PHONY: run-perftest-rs
run-perftest-rs: target/debug/perftest
	target/debug/perftest
