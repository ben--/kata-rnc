#include "replace.h"

#include <string.h>

int replace(char *buf, size_t buflen,
            const char *from, size_t fromlen,
            const char *to, size_t tolen)
{
    char *match = strstr(buf, from);

    if (match) {
        char *tail = match + fromlen;
        size_t tail_len = strlen(tail);

        if ((match + tolen + tail_len + 1) > (buf + buflen)) {
            return 1;
        }
        memmove(match + tolen, tail, tail_len + sizeof(""));
        strncpy(match, to, tolen);
    }

    return 0;
    (void)buflen;
}
