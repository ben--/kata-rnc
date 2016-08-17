.PHONY: ci
ci: unit-test all acceptance-test


all:
	@echo "NotImplemented"

.PHONY: acceptance-test
acceptance-test: acceptance-test/c/runtest
	$<

acceptance-test/c/runtest: test/acceptance/c/runtest.c build/c/librnc.a
	$(CC) $(LDFLAGS) -Lbuild/c $@ $< -lrnc
