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

YAJL_MAJOR=2
YAJL_MINOR=1
YAJL_PATCH=2
YAJL_DIST_NAME = yajl-${YAJL_MAJOR}.${YAJL_MINOR}.${YAJL_PATCH}
YAJL_INCLUDE = build/$(YAJL_DIST_NAME)/include
SOLIB = target/debug/libyajl.so
RLIB = target/debug/libyajl.rlib
CFLAGS = -I$(YAJL_INCLUDE)
YAJL_TEST = build/test/parsing/yajl_test
YAJL_TEST_API = build/tests/api/gen-extra-close
YAJL_TEST_RS = target/debug/yajl_test
INCLUDES = build/$(YAJL_DIST_NAME)/include/yajl/yajl_common.h\
	build/$(YAJL_DIST_NAME)/include/yajl/yajl_gen.h\
	build/$(YAJL_DIST_NAME)/include/yajl/yajl_parse.h\
	build/$(YAJL_DIST_NAME)/include/yajl/yajl_tree.h\
	build/$(YAJL_DIST_NAME)/include/yajl/yajl_version.h

distro: distro.include distro.pkgconfig
build/$(YAJL_DIST_NAME)/include/yajl:
	mkdir -p $@
build/$(YAJL_DIST_NAME)/share/pkgconfig:
	mkdir -p $@
build/tests/api:
	mkdir -p $@
build/test/parsing:
	mkdir -p $@

build/$(YAJL_DIST_NAME)/include/yajl/%.h: include/yajl/%.h build/$(YAJL_DIST_NAME)/include/yajl
	cp $< $@
distro.include: $(INCLUDES)
distro.pkgconfig: build/$(YAJL_DIST_NAME)/share/pkgconfig/yajl.pc
build/$(YAJL_DIST_NAME)/include/yajl/yajl_version.h: include/yajl/yajl_version.h.in build/$(YAJL_DIST_NAME)/include/yajl
	sed 's/define YAJL_MAJOR/define YAJL_MAJOR ${YAJL_MAJOR}/' $< > $@
	sed -i 's/define YAJL_MINOR/define YAJL_MINOR ${YAJL_MINOR}/' $@
	sed -i 's/define YAJL_MICRO/define YAJL_MICRO ${YAJL_PATCH}/' $@

build/$(YAJL_DIST_NAME)/share/pkgconfig/yajl.pc: share/pkgconfig/yajl.pc build/$(YAJL_DIST_NAME)/share/pkgconfig
	sed 's/Version: ../Version: ${YAJL_MAJOR}.${YAJL_MINOR}.${YAJL_PATCH}/' $< > $@

$(SOLIB): crates/yajl-clib/Cargo.toml crates/yajl-clib/src/*.rs
	cargo build --package yajl-clib

$(RLIB): crates/yajl/Cargo.toml crates/yajl/src/**/*.rs
	cargo build --package yajl
bin/parse_config: examples/parse_config.o $(SOLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

bin/json_verify: verify/json_verify.o $(SOLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

bin/json_reformat: reformatter/json_reformat.o $(SOLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

$(YAJL_TEST): tests/parsing/yajl_test.c $(SOLIB) build/test/parsing
	$(CC) -Wall $(CFLAGS) $< -l:libyajl.so -Ltarget/debug -o $@

$(YAJL_TEST_API): tests/api/gen-extra-close.c $(SOLIB) build/tests/api
	$(CC) -Wall $(CFLAGS) $< -l:libyajl.so -Ltarget/debug -o $@

$(YAJL_TEST_RS): examples/yajl_test/src/main.rs examples/yajl_test/Cargo.toml $(RLIB)
	cargo build --package yajl_test

run-parse-config: bin/parse_config
	LD_LIBRARY_PATH=target/debug bin/parse_config < assets/sample.config

run-parse-config-rs:
	cargo run --package parse-config < assets/sample.config

run-json-verify: bin/json_verify
	LD_LIBRARY_PATH=target/debug bin/json_verify -c < assets/sample.config

run-json-verify-rs:
	cargo run --package json-verify -- -c < assets/sample.config

run-json-verify-r-rs:
	cargo run --package json-verify-r -- -c < assets/sample.config

run-json-reformat: bin/json_reformat
	LD_LIBRARY_PATH=target/debug bin/json_reformat < assets/sample.config

run-json-reformat-rs:
	cargo run --package json-reformat < assets/sample.config

run-test-parsing: test-parsing
test-parsing: $(YAJL_TEST)
	# cd test/parsing && ./run_tests.sh
	cd tests/parsing && LD_LIBRARY_PATH=../../target/debug ./run_tests.sh

run-test-parsing-rs: test-parsing-rs
test-parsing-rs: $(YAJL_TEST_RS) $(RLIB)
	cd tests/parsing && ./run_tests.sh "../../$(YAJL_TEST_RS)"

test-api: $(YAJL_TEST_API)
	cd build/tests/api && LD_LIBRARY_PATH=../../../target/debug ../../../tests/api/run_tests.sh

bin/perftest: perf/documents.o perf/perftest.o $(SOLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

run-perftest: bin/perftest
	LD_LIBRARY_PATH=target/debug bin/perftest

target/debug/perftest: examples/perftest/src/*.rs examples/perftest/Cargo.toml
	cargo build --package perftest

.PHONY: run-perftest-rs
run-perftest-rs: target/debug/perftest
	target/debug/perftest
