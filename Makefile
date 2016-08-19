.PHONY: ci
ci: unit-test all acceptance-test

LIBDIR=build/c
LIB=$(LIBDIR)/librnc.a

CFLAGS=-Wall -Wextra -Werror
CPPFLAGS=-Isrc/c


.PHONY: all
all: $(LIB)

$(LIB): $(OBJS)
	ar rcs $@ $^


.PHONY: acceptance-test
acceptance-test: acceptance-test/c/runtest
	$<

acceptance-test/c/runtest: acceptance-test/c/runtest.o $(LIB)
	$(CC) $(LDFLAGS) -L$(LIBDIR) $< -lrnc -o $@

acceptance-test/c/%.o: test/acceptance/%.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@
