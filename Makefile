.PHONY: ci
ci: unit-test valgrind-test all acceptance-test

LIBDIR=build/c
LIB=$(LIBDIR)/librnc.so

CFLAGS=-Wall -Wextra -Werror -fpic -std=gnu11 -O2
CPPFLAGS=-Isrc/c

########################################################################
# Main build

SRCS=$(wildcard src/c/*.c)
OBJS=$(SRCS:src/c/%.c=build/c/%.o)

.PHONY: all
all: $(LIB)

.PHONY: clean
clean:
	rm -rf build unit-test acceptance-test

$(LIB): $(OBJS)
	gcc -shared -o $@ $^

build/c/%.o: src/c/%.c
	@[ -d $(@D) ] || mkdir -p $(@D)
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

########################################################################
# Unit Tests

TEST_SRCS=$(wildcard test/unit/c/*.check)
TEST_BINS=$(TEST_SRCS:test/unit/c/%.check=unit-test/c/fast/%)
TEST_OBJS=$(SRCS:src/c/%.c=unit-test/c/fast/%.o)

TEST_CFLAGS=-fsanitize=address -g
TEST_LIBS=-lcheck -lasan

.PHONY: unit-test
unit-test: $(TEST_BINS:%=run-%)

$(TEST_BINS:%=run-%): run-%: %
	# Needs to run from a read-write directory...
	cd unit-test/c/fast && ./$(@F)

$(TEST_BINS): unit-test/c/fast/%: unit-test/c/fast/%.o $(TEST_OBJS)
	$(CC) $(LDFLAGS) $^ $(TEST_LIBS) -o $@

unit-test/c/fast/%.o: src/c/%.c
	@[ -d $(@D) ] || mkdir -p $(@D)
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

unit-test/c/fast/%.o: unit-test/c/fast/%.c
	$(CC) $(CPPFLAGS) $(CFLAGS) $(TEST_CFLAGS) -c $< -o $@

unit-test/c/fast/%.c: test/unit/c/%.check
	@[ -d $(@D) ] || mkdir -p $(@D)
	checkmk $< > $@

########################################################################
# Valgrind Tests

VALGRIND_BINS=$(TEST_SRCS:test/unit/c/%.check=unit-test/c/valgrind/%)
VALGRIND_OBJS=$(SRCS:src/c/%.c=unit-test/c/valgrind/%.o)

# There is at least one overflow that is not detected with -O9
VALGRIND_CFLAGS=-g
VALGRIND_LIBS=-lcheck

.PHONY: valgrind-test
valgrind-test: $(VALGRIND_BINS:%=valgrind-%)

$(VALGRIND_BINS:%=valgrind-%): valgrind-%: %
	# Needs to run from a read-write directory...
	cd unit-test/c/valgrind && valgrind ./$(@F)

$(VALGRIND_BINS): unit-test/c/valgrind/%: unit-test/c/valgrind/%.o $(VALGRIND_OBJS)
	$(CC) $(LDFLAGS) $^ $(VALGRIND_LIBS) -o $@

unit-test/c/valgrind/%.o: src/c/%.c
	@[ -d $(@D) ] || mkdir -p $(@D)
	$(CC) $(CPPFLAGS) $(CFLAGS) $(VALGRIND_CFLAGS) -c $< -o $@

unit-test/c/valgrind/%.o: unit-test/c/valgrind/%.c
	$(CC) $(CPPFLAGS) $(CFLAGS) $(VALGRIND_CFLAGS) -c $< -o $@

unit-test/c/valgrind/%.c: test/unit/c/%.check
	@[ -d $(@D) ] || mkdir -p $(@D)
	checkmk $< > $@

########################################################################
# Acceptance Tests

.PHONY: acceptance-test
acceptance-test: acceptance-test/c/runtest
	LD_LIBRARY_PATH=$(LIBDIR) $<

acceptance-test/c/runtest: acceptance-test/c/runtest.o $(LIB)
	$(CC) $(LDFLAGS) -L$(LIBDIR) $< -lrnc -o $@

acceptance-test/c/%.o: test/acceptance/%.c
	@[ -d $(@D) ] || mkdir -p $(@D)
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@
