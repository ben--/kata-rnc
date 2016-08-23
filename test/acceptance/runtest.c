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
    int errs = 0;

    errs += _sum("I", "I", "II");
    errs += _sum("I", "II", "III");

    return errs;
    (void)argc;
    (void)argv;
}
