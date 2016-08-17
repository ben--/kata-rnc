.PHONY: ci
ci: unit-test all acceptance-test

LIB=build/c/librnc.a

.PHONY: acceptance-test
acceptance-test: acceptance-test/c/runtest
	$<

acceptance-test/c/runtest: test/acceptance/c/runtest.c build/c/librnc.a
	$(CC) $(LDFLAGS) -Lbuild/c $< -lrnc -o $@

.PHONY: all
all: $(LIB)

$(LIB): $(OBJS)
	ar rcs $@ $^
