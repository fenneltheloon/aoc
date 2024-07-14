#include <stdio.h>

int min(int a, int b) { return a < b ? a : b; }

int surface_area(int l, int w, int h) {
  int lw = l * w;
  int lh = l * h;
  int wh = w * h;
  return ((lw + lh + wh) << 1) + min(min(lw, lh), wh);
}

int main() {
  int l, w, h;
  int total = 0;
  while ((scanf("%i%*[x]%i%*[x]%i", &l, &w, &h)) == 3) {
    total = total + surface_area(l, w, h);
  }
  printf("%i", total);
}