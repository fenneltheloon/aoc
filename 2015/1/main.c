#import <stdio.h>

int main() {
  int ch;
  int val;
  while ((ch = getchar()) != EOF) {
    if (ch == '(') {
      val++;
    } else if (ch == ')') {
      val--;
    }
  }
  printf("%d", val);
}
