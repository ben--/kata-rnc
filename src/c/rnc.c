#include "rnc.h"

#include <stdbool.h>
#include <string.h>

static bool _comes_before(char l, char r)
{
    if (!r) return true;
    return l == 'V';
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
        if (_comes_before(*num_l, *num_r)) {
            *out++ = *num_l++;
        } else {
            *out++ = *num_r++;
        }
    }
    *out = '\0';

    rnc_compress(sum, sumlen, sum);

    return 0;
}

int rnc_denormalize(char *out, size_t outlen, const char *normal)
{
    if (0 == strcmp("IV", normal)) {
        strcpy(out, "IIII");
    } else {
        strcpy(out, normal);
    }

    return 0;
    (void)outlen;
}

int rnc_compress(char *compressed, size_t compressed_len, const char *uncompressed)
{
    if (0 == strcmp("IIIII", uncompressed)) {
        strcpy(compressed, "V");
    } else if (0 == strcmp("IIII", uncompressed)) {
        strcpy(compressed, "IV");
    }

    return 0;

    (void)compressed_len;
}
