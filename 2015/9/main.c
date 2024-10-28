#import <stdio.h>
#import <stdlib.h>
#import <string.h>

struct Edge {
  char *from;
  char *to;
  uint8_t distance;
};

int main(int argc, char *argv[]) {
  size_t buf_size = 48;
  char *buffer = malloc(sizeof(char[buf_size]));

  while (-1 != (getline(&buffer, &buf_size, stdin))) {
    char *from = strsep(&buffer, " ");
    strsep(&buffer, " ");
    char *to = strsep(&buffer, " ");
    strsep(&buffer, " ");
    char *dist = strsep(&buffer, " ");
  }
  return EXIT_SUCCESS;
}
