#include "replace.h"

#include <string.h>

int replace(char *buf, size_t buflen,
            const char *from, size_t fromlen,
            const char *to, size_t tolen)
{
    char *match = strstr(buf, from);

    if (match) {
        char *tail = match + fromlen;
        memmove(match + tolen, tail, strlen(tail) + sizeof(""));
        strncpy(match, to, tolen);
    }

    return 0;
    (void)buflen;
    (void)fromlen;
}
