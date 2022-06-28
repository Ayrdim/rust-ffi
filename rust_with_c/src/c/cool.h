#include <stdint.h>

#ifndef COOL_H
#define COOL_H

typedef struct {
    uint16_t x;
    int y;
} CoolStruct;

void cool_function(int i, char c, CoolStruct * const cs);

float cool_add(float v1, float v2);

#endif /* COOL_H */