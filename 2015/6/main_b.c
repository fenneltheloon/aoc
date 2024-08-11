#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum Order { On = 1, Off = 0, Toggle = -1 };

const char *sep = " ";

int main() {

  size_t line_size = 48;
  char *line = malloc(line_size * sizeof(char));

  // Initialize grid
  bool grid[1000][1000];
  for (short i = 0; i < 1000; i++) {
    for (short j = 0; j < 1000; j++) {
      grid[i][j] = false;
    }
  }

  // Loop input
  while ((getline(&line, &line_size, stdin)) != -1) {
    enum Order state;
    char *word;

    word = strtok(line, sep);
    if (strcmp(word, "turn") == 0) {
      word = strtok(NULL, sep);
      if (strcmp(word, "on") == 0) {
        state = On;
      } else {
        state = Off;
      }
    } else {
      state = Toggle;
    }

    word = strtok(NULL, sep); // XXX,XXX
    char *num;
    short x1 = (short)strtol(word, &num, 0);
    short y1 = (short)strtol(num + 1, &num, 0);

    word = strtok(NULL, sep); // through
    word = strtok(NULL, sep); // XXX,XXX
    short x2 = (short)strtol(word, &num, 0);
    short y2 = (short)strtol(num + 1, &num, 0);

    switch (state) {
    case On:
      for (short i = x1; i <= x2; i++) {
        for (short j = y1; j <= y2; j++) {
          grid[i][j] = true;
        }
      }
      break;
    case Off:
      for (short i = x1; i <= x2; i++) {
        for (short j = y1; j <= y2; j++) {
          grid[i][j] = false;
        }
      }
      break;
    case Toggle:
      for (short i = x1; i <= x2; i++) {
        for (short j = y1; j <= y2; j++) {
          grid[i][j] = !grid[i][j];
        }
      }
      break;
    }
  }

  int total = 0;
  for (short i = 0; i < 1000; i++) {
    for (short j = 0; j < 1000; j++) {
      if (grid[i][j]) {
        total++;
      }
    }
  }

  printf("%i\n", total);
  free(line);
}
