#include "rnc.h"

#include <stdio.h>
#include <string.h>

static int _add(const char *l, const char *r, const char *expected_sum)
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

static int _add_fail(const char *l, const char *r, size_t buflen)
{
    char buf[256] = { 'u', 'n', 'i', 't', 'i', 'a', 'l', 'i', 'z', 'e', 'd' };

    int actual_ret = rnc_add(buf, buflen, l, r);

    if (0 == actual_ret) {
        fprintf(stderr, "Did not fail when putting %s + %s in a %lu character buffer\n", l, r, buflen);
        return 1;
    }

    return 0;
}

static int _sub(const char *l, const char *r, const char *expected_dif)
{
    char actual_dif[256] = { 'u', 'n', 'i', 't', 'i', 'a', 'l', 'i', 'z', 'e', 'd' };

    int actual_ret = rnc_sub(actual_dif, sizeof(actual_dif), l, r);

    if (0 != actual_ret) {
        fprintf(stderr, "Failed subtracting %s - %s: ret = %d\n", l, r, actual_ret);
        return 1;
    }
    if (0 != strcmp(expected_dif, actual_dif)) {
        fprintf(stderr, "Failed subtracting %s - %s: %s != %s\n", l, r, actual_dif, expected_dif);
        return 1;
    }

    return 0;
}

static int _sub_fail(const char *l, const char *r)
{
    char buf[256] = { 'u', 'n', 'i', 't', 'i', 'a', 'l', 'i', 'z', 'e', 'd' };

    int actual_ret = rnc_sub(buf, sizeof(buf), l, r);

    if (0 == actual_ret) {
        fprintf(stderr, "Did not fail when subtracting %s - %s\n", l, r);
        return 1;
    }

    return 0;
}

int main(int argc, char **argv)
{
    int errs = 0, tests = 0;

    tests++; errs += _add("I", "I", "II");
    tests++; errs += _add("I", "II", "III");
    tests++; errs += _add("II", "III", "V");
    tests++; errs += _add("IV", "I", "V");
    tests++; errs += _add("V", "I", "VI");
    tests++; errs += _add("IV", "V", "IX");
    tests++; errs += _add("V", "V", "X");
    tests++; errs += _add("VI", "IV", "X");
    tests++; errs += _add("I", "IX", "X");
    tests++; errs += _add("IX", "X", "XIX");
    tests++; errs += _add("V", "XIV", "XIX");
    tests++; errs += _add("V", "XV", "XX");
    tests++; errs += _add("L", "I", "LI");
    tests++; errs += _add("L", "XI", "LXI");
    tests++; errs += _add("XLIX", "I", "L");
    tests++; errs += _add("XX", "XX", "XL");
    tests++; errs += _add("XL", "X", "L");
    tests++; errs += _add("L", "L", "C");
    tests++; errs += _add("XCIX", "II", "CI");
    tests++; errs += _add("CC", "CC", "CD");
    tests++; errs += _add("CDXCIX", "I", "D");
    tests++; errs += _add("CM", "C", "M");
    tests++; errs += _add("CMXCIX", "V", "MIV");
    tests++; errs += _add("DCCCXCIX", "X", "CMIX");
    tests++; errs += _add("MCMXC", "L", "MMXL");
    tests++; errs += _add("I", "MMMCMXCVIII", "MMMCMXCIX");
    tests++; errs += _add("MMMDCCCLXXXVIII", "I", "MMMDCCCLXXXIX");
    tests++; errs += _add("MCMXCIX", "MCMXCIX", "MMMCMXCVIII");

    tests++; errs += _add_fail("I", "I", 2);
    tests++; errs += _add_fail("MCMXCIX", "MMCMXCIX", 32);
    tests++; errs += _add_fail("J", "I", 32);
    tests++; errs += _add_fail("IM", "I", 32);

    tests++; errs += _sub("II", "I", "I");
    tests++; errs += _sub("III", "I", "II");
    tests++; errs += _sub("V", "I", "IV");
    tests++; errs += _sub("IV", "I", "III");
    tests++; errs += _sub("V", "IV", "I");
    tests++; errs += _sub("X", "V", "V");
    tests++; errs += _sub("X", "I", "IX");
    tests++; errs += _sub("X", "VI", "IV");
    tests++; errs += _sub("CX", "V", "CV");
    tests++; errs += _sub("CXI", "V", "CVI");
    tests++; errs += _sub("M", "I", "CMXCIX");

    tests++; errs += _sub_fail("I", "II");
    tests++; errs += _sub_fail("J", "I");

    printf("test result: %d passed, %d failed\n", tests - errs, errs);

    return errs;
    (void)argc;
    (void)argv;
}
