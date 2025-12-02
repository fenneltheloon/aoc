#include "ArrayList.h"
#include <stdlib.h>
#define ARRAYLIST_MIN_CAP 32

typedef struct ArrayList {
  size_t len, capacity;
  void **buf;
} ArrayList;

ArrayList * new_ArrayList(uint capacity) {
  ArrayList *new = malloc(sizeof(ArrayList));
  new->len = 0;
  capacity = 
  // TODO: we're going to round up passed capacity to the next power of two, assuming that the user never wants to worry about that.
  new->capacity = capacity;
  new->buf = malloc(sizeof(void *) * new->capacity);
  return new;
} 

// Frees ALL objects in ArrayList and the ArrayList itself
void ArrayList_free(ArrayList *list) {
  for (size_t i = 0; i < list->capacity; i++) {
    free(list->buf[i]);
  }
  free(list->buf);
  free(list);
}

void ArrayList_expand(ArrayList *list) {
  ArrayList *bigger = new_ArrayList()
  
}

void ArrayList_add(ArrayList *list, void *obj) {
  list->buf[list->len] = obj;
  list->len++;
  (list->len == list->capacity) ? ArrayList_expand(list) :;
}

