#include "rnc.h"

#include "replace.h"

#include <stdbool.h>
#include <string.h>

bool rnc_larger(char l, char r)
{
    if (!r) return true;
    else if (!l) return false;
    else if (r == 'M') return false;
    else if (l == 'M') return true;
    else if (r == 'D') return false;
    else if (l == 'D') return true;
    else if (r == 'C') return false;
    else if (l == 'C') return true;
    else if (r == 'L') return false;
    else if (l == 'L') return true;
    else if (r == 'X') return false;
    else if (l == 'X') return true;
    else return r != 'V';
}

static int _merge_sorted_chars(char *out, size_t out_len, const char *x, const char *y)
{
    size_t l = 0;

    while (*x || *y) {
        if (rnc_larger(*x, *y)) {
            if (++l > out_len) return 1;
            *out++ = *x++;
        } else {
            if (++l > out_len) return 1;
            *out++ = *y++;
        }
    }

    if (++l > out_len) return 1;
    *out = '\0';

    return 0;
}

int rnc_add(char *sum, size_t sumlen, const char *raw_l, const char *raw_r)
{
    char buf_l[sizeof("MMMDCCCCLXXXXVIIII")];
    char buf_r[sizeof("MMMDCCCCLXXXXVIIII")];
    char buf_sum[sizeof("MDCCCCLXXXXVIIIIMDCCCCLXXXXVIIII")];

    if (0 != rnc_denormalize(buf_l, sizeof(buf_l), raw_l)) return 1;
    if (0 != rnc_denormalize(buf_r, sizeof(buf_r), raw_r)) return 1;

    if (0 != _merge_sorted_chars(buf_sum, sizeof(buf_sum), buf_l, buf_r)) return 1;

    rnc_normalize(buf_sum, sizeof(buf_sum));

    if (stpncpy(sum, buf_sum, sumlen) >= sum + sumlen) {
        return 1;
    }

    return 0;
}

static char *_find_digit(char *buf, size_t buf_len, char digit) {
    char *l = strchr(buf, digit);
    if (!l) {
        if (0 != rnc_borrow(buf, buf_len, digit)) {
            return NULL;
        }
        l = strchr(buf, digit);
        if (!l) {
            return NULL;
        }
    }

    return l;
}

static int _sub_digit(char *buf, size_t buf_len, char digit) {
    char *l = _find_digit(buf, buf_len, digit);
    if (!l) {
        return 1;
    }

    memmove(l, l + sizeof(char), buf_len - (l-buf) - sizeof(char));

    return 0;
}

static int _sub_digits(char *from, size_t from_len, char *digits) {
    for (char *r = digits; *r; r++) {
        if (0 != _sub_digit(from, from_len, *r)) {
            return 1;
        }
    }

    return 0;
}

int rnc_sub(char *diff, size_t diff_len, const char *num_l, const char *num_r)
{
    char buf_l[sizeof("MMMDCCCCLXXXXVIIIII")];
    char buf_r[sizeof("MMMDCCCCLXXXXVIIII")];
    rnc_denormalize(buf_l, sizeof(buf_l), num_l);
    rnc_denormalize(buf_r, sizeof(buf_r), num_r);

    if (0 != _sub_digits(buf_l, sizeof(buf_l), buf_r)) {
        return 1;
    }

    rnc_normalize(buf_l, sizeof(buf_l));
    if (stpncpy(diff, buf_l, diff_len) >= diff + diff_len) {
        return 1;
    }

    return 0;
}

static const char *_expansion(char digit)
{
    switch (digit) {
        case 'M': return "DD";
        case 'D': return "CCCCC";
        case 'C': return "LL";
        case 'L': return "XXXXX";
        case 'X': return "VV";
        case 'V': return "IIIII";
    }
    return NULL;
}

static char *_last_larger_digit(char *num, char digit) {
    for (char *p = num + strlen(num); p >= num; p--) {
        if (rnc_larger(*p, digit)) {
            return p;
        }
    }

    return NULL;
}

int rnc_borrow(char *num, size_t numlen, char numeral)
{
    char suffix[sizeof("CCCCLXXXXVIIII")];

    char *p = _last_larger_digit(num, numeral);
    if (p == NULL) {
        return 1;
    }

    if (stpncpy(suffix, p + sizeof(char), sizeof(suffix)) >= suffix + sizeof(suffix)) {
        return 1;
    }

    while (*p != numeral) {
        p = stpncpy(p, _expansion(*p), numlen - (p-num));
        if (p >= num + numlen) {
            return 1;
        }
        p--;
    }
    p++;

    p = stpncpy(p, suffix, numlen - (p-num));
    if (p >= num + numlen) {
        return 1;
    }

    return 0;
}

int rnc_denormalize(char *out, size_t outlen, const char *normal)
{
    if (stpncpy(out, normal, outlen) >= out + outlen) {
        return 1;
    }

    return
        REPLACE(out, outlen, "IV", "IIII") ||
        REPLACE(out, outlen, "IX", "VIIII") ||
        REPLACE(out, outlen, "XL", "XXXX") ||
        REPLACE(out, outlen, "XC", "LXXXX") ||
        REPLACE(out, outlen, "CD", "CCCC") ||
        REPLACE(out, outlen, "CM", "DCCCC");
}

int rnc_normalize(char *buf, size_t buflen)
{
    REPLACE(buf, buflen, "IIIII", "V");
    REPLACE(buf, buflen, "IIII", "IV");
    REPLACE(buf, buflen, "VV", "X");
    REPLACE(buf, buflen, "VIV", "IX");
    REPLACE(buf, buflen, "XXXXX", "L");
    REPLACE(buf, buflen, "XXXX", "XL");
    REPLACE(buf, buflen, "LL", "C");
    REPLACE(buf, buflen, "LXL", "XC");
    REPLACE(buf, buflen, "CCCCC", "D");
    REPLACE(buf, buflen, "CCCC", "CD");
    REPLACE(buf, buflen, "DD", "M");
    REPLACE(buf, buflen, "DCD", "CM");

    return 0;
}
