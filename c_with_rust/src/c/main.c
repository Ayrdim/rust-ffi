#include <stdio.h>
#include "cool.h"

int main() 
{
    float v1 = 5.5;
    float v2 = 5.4;

    float res = cool_add(v1, v2);
    printf("Using rust to add %f and %f returned %f\n", v1, v2, res);

    CoolStruct cs = 
    {
        .x = 65535,
        .y = -404
    };

    CoolStruct cs2 = 
    {
        .x = 1996,
        .y = -30
    };

    printf("Before calling cool function cs.x = %i, cs.y = %i\n", cs.x, cs.y);
    cool_function(-420, 'E', &cs, cs2);
    printf("After calling cool function cs.x = %i, cs.y = %i\n", cs.x, cs.y);
    

    return 0;
}