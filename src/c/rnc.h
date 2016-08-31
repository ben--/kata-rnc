#ifndef RNC_H
#define RNC_H

#include <stdbool.h>
#include <stddef.h>

int rnc_add(char *sum, size_t sumlen, const char *num_l, const char *num_r);
int rnc_sub(char *diff, size_t diff_len, const char *num_l, const char *num_r);

int rnc_borrow(char *num, size_t numlen, char numeral);
bool rnc_larger(char l, char r);
int rnc_normalize(char *buf, size_t buflen);
int rnc_denormalize(char *out, size_t outlen, const char *normal);

#endif /* !RNC_H */
