#include "rnc.h"

#include "replace.h"

#include <stdbool.h>
#include <string.h>

bool rnc_larger(char l, char r)
{
    if (!r) return true;
    else if (!l) return false;
    else if (r == 'L') return false;
    else if (l == 'L') return true;
    else if (r == 'X') return false;
    else if (l == 'X') return true;
    else return r != 'V';
}

int rnc_add(char *sum, size_t sumlen, const char *raw_l, const char *raw_r)
{
    char buf_l[12], buf_r[12];
    char *num_l = buf_l;
    char *num_r = buf_r;
    rnc_denormalize(num_l, sizeof(buf_l), raw_l);
    rnc_denormalize(num_r, sizeof(buf_r), raw_r);

    char *out = sum;
    while (*num_l || *num_r) {
        if (rnc_larger(*num_l, *num_r)) {
            *out++ = *num_l++;
        } else {
            *out++ = *num_r++;
        }
    }
    *out = '\0';

    rnc_normalize(sum, sumlen, sum);

    return 0;
}

int rnc_denormalize(char *out, size_t outlen, const char *normal)
{
    strcpy(out, normal);

    REPLACE(out, outlen, "IV", "IIII");
    REPLACE(out, outlen, "XL", "XXXX");
    REPLACE(out, outlen, "IX", "VIIII");

    return 0;
}

int rnc_normalize(char *out, size_t outlen, const char *denormal)
{
    // Temporary -- remove this argument to avoid this approach
    memmove(out, denormal, strlen(denormal) + 1);

    REPLACE(out, outlen, "IIIII", "V");
    REPLACE(out, outlen, "IIII", "IV");
    REPLACE(out, outlen, "VIV", "IX");
    REPLACE(out, outlen, "VV", "X");
    REPLACE(out, outlen, "XXXXX", "L");

    return 0;
}
