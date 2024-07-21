#include <stdio.h>
#include <stdlib.h>

// int min(int a, int b) { return a < b ? a : b; }
//
// int surface_area(int l, int w, int h) {
//   int lw = l * w;
//   int lh = l * h;
//   int wh = w * h;
//   return ((lw + lh + wh) << 1) + min(min(lw, lh), wh);
// }

void bubble_sort(int a[], size_t size) {
  // if (size == 1) {
  //   return;
  // }
  int sorted;
  do {
    sorted = 0;
    int i = 0;
    while (i < size - 1) {
      if (a[i] > a[i + 1]) {
        int tmp = a[i];
        a[i] = a[i + 1];
        a[i + 1] = tmp;
        sorted++;
      }
      i++;
    }
  } while (sorted != 0);
}

int perimeter(int l[]) { return (((l[0] + l[1]) * 2) + (l[0] * l[1] * l[2])); }

int main() {
  // 0 = l, 1 = w, 2 = h
  int sides[3];
  char *input = malloc(20);
  int total = 0;
  while ((fgets(input, 10, stdin)) != NULL) {
    sscanf(input, "%i%*[x]%i%*[x]%i", &sides[0], &sides[1], &sides[2]);
    bubble_sort(sides, 3);
    total = total + perimeter(sides);
  }
  printf("%i", total);
  free(input);
}
