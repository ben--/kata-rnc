#include "rnc.h"

#include <stdio.h>
#include <string.h>

static int _sum(const char *l, const char *r, const char *expected_sum)
{
    char actual_sum[256];

    rnc_add(actual_sum, sizeof(actual_sum), l, r);
    if (0 != strcmp(expected_sum, actual_sum)) {
        fprintf(stderr, "Failed adding %s + %s: %s != %s\n", l, r, actual_sum, expected_sum);
        return 1;
    }

    return 0;
}

int main(int argc, char **argv)
{
    int errs = 0, tests = 0;

    tests++; errs += _sum("I", "I", "II");
    tests++; errs += _sum("I", "II", "III");
    tests++; errs += _sum("II", "III", "V");
    tests++; errs += _sum("IV", "I", "V");
    tests++; errs += _sum("V", "I", "VI");
    tests++; errs += _sum("V", "V", "X");

    printf("test result: %d passed, %d failed\n", tests - errs, errs);

    return errs;
    (void)argc;
    (void)argv;
}
