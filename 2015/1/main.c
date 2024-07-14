#import <stdio.h>

int main() {
  int ch = 0, val = 0, i = 0;
  while ((ch = getchar()) != EOF) {
    if (val < 0) {
      printf("%i\n", i);
      return 0;
    }
    if (ch == '(') {
      val++;
    } else if (ch == ')') {
      val--;
    }
    i++;
  }
  return 1;
}
