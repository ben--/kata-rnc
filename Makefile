.PHONY: ci
ci: unit-test all acceptance-test

LIBDIR=build/c
LIB=$(LIBDIR)/librnc.so

CFLAGS=-Wall -Wextra -Werror -fpic
CPPFLAGS=-Isrc/c

########################################################################
# Main build

SRCS=$(wildcard src/c/*.c)
OBJS=$(SRCS:src/c/%.c=build/c/%.o)

.PHONY: all
all: $(LIB)

$(LIB): $(OBJS)
	gcc -shared -o $@ $^

build/c/%.o: src/c/%.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

########################################################################
# Unit Tests

TEST_SRCS=$(wildcard test/unit/c/*.check)
TEST_BINS=$(TEST_SRCS:test/unit/c/%.check=unit-test/c/%)
TEST_OBJS=$(SRCS:src/c/%.c=unit-test/c/%.o)

.PHONY: unit-test
unit-test: $(TEST_BINS)
	mv unit-test/c/test_sum.c unit-test/c/test_sum.bak
	@for t in $^; do \
	    cd unit-test/c && ./$$(basename $$t) ; \
	 done

$(TEST_BINS): unit-test/c/%: unit-test/c/%.o $(TEST_OBJS)
	$(CC) $(LDFLAGS) $^ -lcheck -o $@

unit-test/c/%.o: src/c/%.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

unit-test/c/%.o: unit-test/c/%.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

unit-test/c/%.c: test/unit/c/%.check
	checkmk $< > $@

########################################################################
# Acceptance Tests

.PHONY: acceptance-test
acceptance-test: acceptance-test/c/runtest
	LD_LIBRARY_PATH=$(LIBDIR) $<

acceptance-test/c/runtest: acceptance-test/c/runtest.o $(LIB)
	$(CC) $(LDFLAGS) -L$(LIBDIR) $< -lrnc -o $@

acceptance-test/c/%.o: test/acceptance/%.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@
