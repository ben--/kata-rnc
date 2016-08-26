#ifndef RNC_H
#define RNC_H

#include <stdbool.h>
#include <stddef.h>

int rnc_add(char *sum, size_t sumlen, const char *num_l, const char *num_r);
bool rnc_larger(char l, char r);
int rnc_normalize(char *out, size_t outlen, const char *denormal);
int rnc_denormalize(char *out, size_t outlen, const char *normal);

#endif /* !RNC_H */
