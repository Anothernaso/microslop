#include "microslop.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    char *slop = MICROSLOP_to_slop("Hello, World!");

    char *hall = MICROSLOP_to_hallucination(slop);
    free(slop);

    printf("%s\n", hall);
    free(hall);

    return 0;
}
