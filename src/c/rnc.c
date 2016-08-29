#include "rnc.h"

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
    rnc_denormalize(num_l, sizeof(num_l), raw_l);
    rnc_denormalize(num_r, sizeof(num_r), raw_r);

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
    char *tail;
    strcpy(out, normal);
    if (NULL != (tail = strstr(out, "IV"))) {
        strcpy(tail, "IIII");
    }
    if (NULL != (tail = strstr(out, "XL"))) {
        strcpy(tail, "XXXX");
    }
    if (NULL != (tail = strstr(out, "IX"))) {
        strcpy(tail, "VIIII");
    }

    return 0;
    (void)outlen;
}

int rnc_normalize(char *out, size_t outlen, const char *denormal)
{
    char *tail;
    strcpy(out, denormal);
    while (NULL != (tail = strstr(out, "IIIII"))) {
        *tail++ = 'V';
        strcpy(tail, out + strlen("IIIII"));
    }
    while (NULL != (tail = strstr(out, "IIII"))) {
        strcpy(tail, "IV");
    }
    while (NULL != (tail = strstr(out, "VIV"))) {
        strcpy(tail, "IX");
    }
    while (NULL != (tail = strstr(out, "VV"))) {
        strcpy(tail, "X");
    }

    return 0;

    (void)outlen;
}
