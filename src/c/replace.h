#ifndef REPLACE_H
#define REPLACE_H

#include <stddef.h>

/* Macro to simplify calling replace with static from/to strings */
#define REPLACE(buf, buflen, from, to) \
    replace(buf, buflen, from, sizeof(from) - sizeof(""), to, sizeof(to) - sizeof(""))

int replace(char *buf, size_t buflen,
            const char *from, size_t fromlen,
            const char *to, size_t tolen);

#endif /* ! REPLACE_H */
