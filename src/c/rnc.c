#include "rnc.h"

#include <string.h>

int rnc_add(char *sum, size_t sumlen, const char *num_l, const char *num_r)
{
    strcpy(sum, num_l);
    strcat(sum, num_r);

    return 0;

    (void)sumlen;
}
