#include "rnc.h"

#include <stdio.h>
#include <string.h>

static int _sum(const char *l, const char *r, const char *expected_sum)
{
    char actual_sum[256] = { 'u', 'n', 'i', 't', 'i', 'a', 'l', 'i', 'z', 'e', 'd' };

    int actual_ret = rnc_add(actual_sum, sizeof(actual_sum), l, r);

    if (0 != actual_ret) {
        fprintf(stderr, "Failed adding %s + %s: ret = %d\n", l, r, actual_ret);
        return 1;
    }
    if (0 != strcmp(expected_sum, actual_sum)) {
        fprintf(stderr, "Failed adding %s + %s: %s != %s\n", l, r, actual_sum, expected_sum);
        return 1;
    }

    return 0;
}

static int _fail(const char *l, const char *r, size_t buflen)
{
    char buf[256] = { 'u', 'n', 'i', 't', 'i', 'a', 'l', 'i', 'z', 'e', 'd' };

    int actual_ret = rnc_add(buf, buflen, l, r);

    if (0 == actual_ret) {
        fprintf(stderr, "Did not fail when putting %s + %s in a %lu character buffer\n", l, r, buflen);
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
    tests++; errs += _sum("IV", "V", "IX");
    tests++; errs += _sum("V", "V", "X");
    tests++; errs += _sum("VI", "IV", "X");
    tests++; errs += _sum("I", "IX", "X");
    tests++; errs += _sum("IX", "X", "XIX");
    tests++; errs += _sum("V", "XIV", "XIX");
    tests++; errs += _sum("V", "XV", "XX");
    tests++; errs += _sum("L", "I", "LI");
    tests++; errs += _sum("L", "XI", "LXI");
    tests++; errs += _sum("XLIX", "I", "L");
    tests++; errs += _sum("XX", "XX", "XL");
    tests++; errs += _sum("XL", "X", "L");
    tests++; errs += _sum("L", "L", "C");
    tests++; errs += _sum("XCIX", "II", "CI");
    tests++; errs += _sum("CC", "CC", "CD");
    tests++; errs += _sum("CDXCIX", "I", "D");
    tests++; errs += _sum("DCD", "C", "M");
    tests++; errs += _sum("DCDXCIX", "V", "MIV");
    tests++; errs += _sum("DCCCXCIX", "X", "CMIX");
    tests++; errs += _sum("MCMXC", "L", "MMXL");
    tests++; errs += _sum("I", "MMMCMXCVIII", "MMMCMXCIX");
    tests++; errs += _sum("MMMDCCCLXXXVIII", "I", "MMMDCCCLXXXIX");

    tests++; errs += _fail("I", "I", 2);
    //tests++; errs += _fail("IL", "I", 32);

    printf("test result: %d passed, %d failed\n", tests - errs, errs);

    return errs;
    (void)argc;
    (void)argv;
}
