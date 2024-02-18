RLIB = target/debug/libyajl.so
CFLAGS = -Ibuild/yajl-2.1.1/include
YAJL_TEST = build/test/parsing/yajl_test

parse_config: example/parse_config.o $(RLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

json_verify: verify/json_verify.o $(RLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

$(YAJL_TEST): test/parsing/yajl_test.c $(RLIB)
	$(CC) -Wall $(CFLAGS) test/parsing/yajl_test.c -l:libyajl.so -Ltarget/debug -o $@


$(RLIB): clib/Cargo.toml clib/src/*.rs
	cargo build
 
run-parse-config: parse_config
	LD_LIBRARY_PATH=target/debug ./parse_config < example/sample.config

run-json-verify: json_verify
	LD_LIBRARY_PATH=target/debug ./json_verify < example/sample.config

test-parsing: $(YAJL_TEST)
	# cd test/parsing && ./run_tests.sh
	cd test/parsing && LD_LIBRARY_PATH=../../target/debug ./run_tests.sh
