#include <stdint.h>

#ifndef COOL_H
#define COOL_H

typedef struct {
    uint16_t x;
    int y;
} CoolStruct;

float cool_add(float v1, float v2);

void cool_function(int i, char c, CoolStruct * const cs, CoolStruct cs2);

#endif /* COOL_H */