.PHONY: ci
ci: unit-test all acceptance-test

LIBDIR=build/c
LIB=$(LIBDIR)/librnc.a

CFLAGS=-Wall -Wextra -Werror
CPPFLAGS=-Isrc/c

SRCS=src/c/rnc.c
OBJS=$(SRCS:src/c/%.c=build/c/%.o)

.PHONY: all
all: $(LIB)

$(LIB): $(OBJS)
	ar rcs $@ $^

build/c/%.o: src/c/%.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

.PHONY: acceptance-test
acceptance-test: acceptance-test/c/runtest
	$<

acceptance-test/c/runtest: acceptance-test/c/runtest.o $(LIB)
	$(CC) $(LDFLAGS) -L$(LIBDIR) $< -lrnc -o $@

acceptance-test/c/%.o: test/acceptance/%.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@
