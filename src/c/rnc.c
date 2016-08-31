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

static void _merge_sorted_chars(char *out, const char *x, const char *y)
{
    while (*x || *y) {
        if (rnc_larger(*x, *y)) {
            *out++ = *x++;
        } else {
            *out++ = *y++;
        }
    }
    *out = '\0';
}

int rnc_add(char *sum, size_t sumlen, const char *raw_l, const char *raw_r)
{
    char buf_l[20], buf_r[20], buf_sum[32];
    rnc_denormalize(buf_l, sizeof(buf_l), raw_l);
    rnc_denormalize(buf_r, sizeof(buf_r), raw_r);

    _merge_sorted_chars(buf_sum, buf_l, buf_r);

    rnc_normalize(buf_sum, sizeof(buf_sum));

    if (strlen(buf_sum) > sumlen - sizeof("")) {
        return 1;
    }
    strcpy(sum, buf_sum);

    return 0;
}

int rnc_sub(char *diff, size_t diff_len, const char *num_l, const char *num_r)
{
    char buf_l[32], buf_r[20];
    rnc_denormalize(buf_l, sizeof(buf_l), num_l);
    rnc_denormalize(buf_r, sizeof(buf_r), num_r);

    if (!strstr(buf_l, buf_r)) {
        rnc_borrow(buf_l, sizeof(buf_l), *buf_r);
        if (!strstr(buf_l, buf_r)) {
            return 1;
        }
    }
    strcpy(diff, buf_l);
    replace(diff, diff_len, buf_r, strlen(buf_r), "", strlen(""));

    rnc_normalize(diff, diff_len);
    return 0;
}

int rnc_borrow(char *num, size_t numlen, char numeral)
{
    switch (numeral) {
        case 'I':
            REPLACE(num, numlen, "V", "IIIII");
            break;
        case 'V':
            REPLACE(num, numlen, "X", "VV");
            break;
    }
    return 0;
}

int rnc_denormalize(char *out, size_t outlen, const char *normal)
{
    strcpy(out, normal);

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
