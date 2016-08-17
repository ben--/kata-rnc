.PHONY: ci
ci: unit-test all acceptance-test

LIB=build/c/librnc.a

CFLAGS=-Wall -Wextra -Werror
CPPFLAGS=-Isrc/c


.PHONY: all
all: $(LIB)

$(LIB): $(OBJS)
	ar rcs $@ $^


.PHONY: acceptance-test
acceptance-test: acceptance-test/c/runtest
	$<

acceptance-test/c/runtest: acceptance-test/c/runtest.o build/c/librnc.a
	$(CC) $(LDFLAGS) -Lbuild/c $< -lrnc -o $@

acceptance-test/c/%.o: test/acceptance/c/%.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@
