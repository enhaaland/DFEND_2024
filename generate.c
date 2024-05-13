#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main() {
    int i;

    //int seed = 1715630993;
    // Seed the random number generator using current time
    //srand(seed);
    srand(time(0));

    // Generate and print 10 random numbers
    printf("Generated random numbers:\n");
    for (i = 0; i < 8; i++) {
        printf("%d\n", rand());
    }

    return 0;
}
