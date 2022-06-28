#include <stdio.h>
#include "cool.h"

void cool_function(int i, char c, CoolStruct * const cs)
{
    if (0 != cs)
    {
        printf("From C:\n");
        printf("int i = %i, char c = %c\n", i, c);
        printf("cs->x = %i, cs->y = %i... Adding 3 to both\n", cs->x, cs->y);
        cs->x += 3;
        cs->y += 3;
    }
    else 
    {
        printf("You provided a null pointer");
    }
}

float cool_add(float v1, float v2)
{
    printf("Adding %f and %f\n", v1, v2);

    return v1 + v2;
}
