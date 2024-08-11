#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const char *sep = " ";
const int ARRAY_SIZE = 25000;
enum Order { On = 1, Off = 0, Toggle = -1 };

typedef struct Rectangle {
  int x1;
  int y1;
  int x2;
  int y2;
  enum Order state;
} Rectangle;

void assign(Rectangle *rec, int x1, int y1, int x2, int y2, enum Order state) {
  rec->x1 = x1;
  rec->y1 = y1;
  rec->x2 = x2;
  rec->y2 = y2;
  rec->state = state;
}

enum Order apply_new(enum Order current, enum Order new) {
  if (new == Toggle) {
    switch (current) {
    case On:
      return Off;
    case Off:
      return On;
    case Toggle:
      fprintf(stderr, "Detected current square containing toggle state, not "
                      "allowed. Terminating...");

      exit(1);
    }
  }
  return new;
}

int assimilate(Rectangle *current_list, Rectangle *new_list, Rectangle new,
               int current_list_size) {
  int new_list_index = 0;
  for (int current_list_index = 0; current_list_index < current_list_size;
       current_list_index++) {
    Rectangle current = current_list[current_list_index];

    if (current.y1 > new.y2 || new.y1 > current.y2 || current.x1 > new.x2 ||
        new.x1 > current.x2) {
      assign(&new_list[new_list_index], current.x1, current.y1, current.x2,
             current.y2, current.state);
      new_list_index++;
      continue;
    }

    char config = 0;
    if (current.y1 >= new.y1) {
      config++;
    }
    config = config << 1;
    if (new.y2 >= current.y2) {
      config++;
    }
    config = config << 1;
    if (current.x1 >= new.x1) {
      config++;
    }
    config = config << 1;
    if (new.x2 >= current.x2) {
      config++;
    }

    switch (config) {
    case 0:
      assign(&new_list[new_list_index], current.x1, current.y1, current.x2,
             new.y1 - 1, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y1, new.x1 - 1, new.y2,
             current.state);
      new_list_index++;
      assign(&new_list[new_list_index], new.x1, new.y1, new.x2, new.y2,
             apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], new.x2 + 1, new.y1, current.x2, new.y2,
             current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y2 + 1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 1:
      assign(&new_list[new_list_index], current.x1, current.y1, current.x2,
             new.y1 - 1, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y1, new.x1 - 1, new.y2,
             current.state);
      new_list_index++;
      assign(&new_list[new_list_index], new.x1, new.y1, current.x2, new.y2,
             apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y2 + 1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 2:
      assign(&new_list[new_list_index], current.x1, current.y1, current.x2,
             new.y1 - 1, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y1, new.x2, new.y2,
             apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], new.x2 + 1, new.y1, current.x2, new.y2,
             current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y2 + 1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 3:
      assign(&new_list[new_list_index], current.x1, current.y1, current.x2,
             new.y1 - 1, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y1, current.x2, new.y2,
             apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y2 + 1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 4:
      assign(&new_list[new_list_index], current.x1, current.y1, current.x2,
             new.y1 - 1, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y1, new.x1 - 1,
             current.y2, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], new.x1, new.y1, new.x2, current.y2,
             apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], new.x2 + 1, new.y1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 5:
      assign(&new_list[new_list_index], current.x1, current.y1, current.x2,
             new.y1 - 1, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y1, new.x1 - 1,
             current.y2, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], new.x1, new.y1, current.x2, current.y2,
             apply_new(current.state, new.state));
      new_list_index++;
      break;
    case 6:
      assign(&new_list[new_list_index], current.x1, current.y1, current.x2,
             new.y1 - 1, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y1, new.x2, current.y2,
             apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], new.x2 + 1, new.y1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 7:
      assign(&new_list[new_list_index], current.x1, current.y1, current.x2,
             new.y1 - 1, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y1, current.x2,
             current.y2, apply_new(current.state, new.state));
      new_list_index++;
      break;
    case 8:
      assign(&new_list[new_list_index], current.x1, current.y1, new.x1 - 1,
             new.y2, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], new.x1, current.y1, new.x2, new.y2,
             apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], new.x1 + 1, current.y1, current.x2,
             new.y2, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y2 + 1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 9:
      assign(&new_list[new_list_index], current.x1, current.y1, new.x1 - 1,
             new.y2, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], new.x1, current.y1, current.x2, new.y2,
             apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y2 + 1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 10:
      assign(&new_list[new_list_index], current.x1, current.y1, new.x2, new.y2,
             apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], new.x2 + 1, current.y1, current.x2,
             new.y2, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y2 + 1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 11:
      assign(&new_list[new_list_index], current.x1, current.y1, current.x2,
             new.y2, apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], current.x1, new.y2 + 1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 12:
      assign(&new_list[new_list_index], current.x1, current.y1, new.x1 - 1,
             current.y2, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], new.x1, current.y1, new.x2, current.y2,
             apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], new.x2 + 1, current.y1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 13:
      assign(&new_list[new_list_index], current.x1, current.y1, new.x1 - 1,
             current.y2, current.state);
      new_list_index++;
      assign(&new_list[new_list_index], new.x1, current.y1, current.x2,
             current.y2, apply_new(current.state, new.state));
      new_list_index++;
      break;
    case 14:
      assign(&new_list[new_list_index], current.x1, current.y1, new.x2,
             current.y2, apply_new(current.state, new.state));
      new_list_index++;
      assign(&new_list[new_list_index], new.x2 + 1, current.y1, current.x2,
             current.y2, current.state);
      new_list_index++;
      break;
    case 15:
      assign(&new_list[new_list_index], current.x1, current.y1, current.x2,
             current.y2, apply_new(current.state, new.state));
      new_list_index++;
      break;
    }
    if (new_list_index > ARRAY_SIZE) {
      fprintf(stderr, "Array size exceeded. Exiting...\n");
      exit(1);
    }
  }
  return new_list_index;
}

int main() {
  size_t line_size = 48;
  char *line = malloc(line_size);

  // Initialize empty map with initial square
  Rectangle *list_a = malloc(ARRAY_SIZE * sizeof(Rectangle));
  Rectangle *list_b = malloc(ARRAY_SIZE * sizeof(Rectangle));

  for (int i = 0; i < ARRAY_SIZE; i++) {
    assign(&list_a[i], 0, 0, 0, 0, Off);
    assign(&list_b[i], 0, 0, 0, 0, Off);
  }

  assign(&list_a[0], 0, 0, 999, 999, Off);
  int list_a_index = 1;
  int line_num = 0;

  while ((getline(&line, &line_size, stdin) != -1)) {
    printf("running line %i\n", ++line_num);
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

    Rectangle new_rect;
    word = strtok(NULL, sep); // XXX,XXX
    char *next;
    new_rect.x1 = (int)strtol(word, &next, 0);
    new_rect.y1 = (int)strtol(&(*(next + 1)), &next, 0);

    word = strtok(NULL, sep); // through
    word = strtok(NULL, sep); // XXX,XXX
    new_rect.x2 = (int)strtol(word, &next, 0);
    new_rect.y2 = (int)strtol(&(*(next + 1)), &next, 0);

    new_rect.state = state;

    // Evaluate the grid given the new square and swap the lists
    list_a_index = assimilate(list_a, list_b, new_rect, list_a_index);
    Rectangle *tmp = list_a;
    list_a = list_b;
    list_b = tmp;
  }

  // Iterate through the squares and calculate how many illuminated
  int total = 0;
  for (int i = 0; i < list_a_index; i++) {
    if (list_a[i].state == On) {
      total = total + ((list_a[i].x2 - list_a[i].x1 + 1) *
                       (list_a[i].y2 - list_a[i].y1 + 1));
    }
  }

  printf("%i\n", total);

  free(line);
  free(list_a);
  free(list_b);
}
