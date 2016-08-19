#include "rnc.h"

#include <string.h>

int main(int argc, char **argv)
{
    char actual[256];

    rnc_add(actual, sizeof(actual), "I", "I");
    if (0 != strcmp("II", actual)) return 1;

    return 0;
    (void)argc;
    (void)argv;
}
