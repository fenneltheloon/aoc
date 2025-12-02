#ifndef ARRAY_LIST
#define ARRAY_LIST

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct ArrayList ArrayList;

ArrayList * ArrayList_new();

void ArrayList_add(ArrayList* list, void* item);

#endif
