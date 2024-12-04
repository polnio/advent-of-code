#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

bool char_is_alpha(char c) {
  return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

struct char_grid {
  char *content;
  size_t width;
  size_t height;
};
typedef struct char_grid CharGrid;

CharGrid *char_grid_new(char *path) {
  CharGrid *grid = malloc(sizeof(CharGrid));
  if (grid == NULL) {
    fprintf(stderr, "Failed to allocate memory.\n");
    return NULL;
  }

  FILE *file = fopen(path, "r");
  if (file == NULL) {
    fprintf(stderr, "Failed to open file: %s\n", path);
    free(grid);
    return NULL;
  }

  size_t size;
  fseek(file, 0, SEEK_END);
  size = ftell(file);
  fseek(file, 0, SEEK_SET);

  size_t width = 0;
  while (getc(file) != '\n') {
    width++;
  }
  rewind(file);

  size_t height = size / (width + 1);

  char *content = malloc(size + 1);
  if (content == NULL) {
    fprintf(stderr, "Failed to allocate memory.\n");
    free(grid);
    return NULL;
  }

  size_t count = fread(content, 1, size, file);
  if (count != size) {
    fprintf(stderr,
            "Failed to read file: read %zu bytes but expected %zu bytes.\n",
            count, size);
    free(content);
    free(grid);
    return NULL;
  }
  rewind(file);

  grid->content = content;
  grid->width = width;
  grid->height = height;
  fclose(file);
  return grid;
}

void free_char_grid(CharGrid *grid) {
  assert(grid != NULL);
  free(grid->content);
  free(grid);
}

char get_char(CharGrid *grid, size_t i, size_t j) {
  assert(grid != NULL);
  assert(i < grid->height);
  assert(j < grid->width);
  return grid->content[i * (grid->width + 1) + j];
}

bool check_up_right(CharGrid *grid, size_t i, size_t j) {
  assert(grid != NULL);
  assert(i < grid->height);
  assert(j < grid->width);

  return i >= 3 && j <= grid->width - 4 && get_char(grid, i, j) == 'X' &&
         get_char(grid, i - 1, j + 1) == 'M' &&
         get_char(grid, i - 2, j + 2) == 'A' &&
         get_char(grid, i - 3, j + 3) == 'S';
}

bool check_right(CharGrid *grid, size_t i, size_t j) {
  assert(grid != NULL);
  assert(i < grid->height);
  assert(j < grid->width);

  return j <= grid->width - 4 && get_char(grid, i, j) == 'X' &&
         get_char(grid, i, j + 1) == 'M' && get_char(grid, i, j + 2) == 'A' &&
         get_char(grid, i, j + 3) == 'S';
}

bool check_down_right(CharGrid *grid, size_t i, size_t j) {
  assert(grid != NULL);
  assert(i < grid->height);
  assert(j < grid->width);

  return i <= grid->height - 4 && j <= grid->width - 4 &&
         get_char(grid, i, j) == 'X' && get_char(grid, i + 1, j + 1) == 'M' &&
         get_char(grid, i + 2, j + 2) == 'A' &&
         get_char(grid, i + 3, j + 3) == 'S';
}

bool check_down(CharGrid *grid, size_t i, size_t j) {
  assert(grid != NULL);
  assert(i < grid->height);
  assert(j < grid->width);

  return i <= grid->height - 4 && get_char(grid, i, j) == 'X' &&
         get_char(grid, i + 1, j) == 'M' && get_char(grid, i + 2, j) == 'A' &&
         get_char(grid, i + 3, j) == 'S';
}

bool check_down_left(CharGrid *grid, size_t i, size_t j) {
  assert(grid != NULL);
  assert(i < grid->height);
  assert(j < grid->width);

  return i <= grid->height - 4 && j >= 3 && get_char(grid, i, j) == 'X' &&
         get_char(grid, i + 1, j - 1) == 'M' &&
         get_char(grid, i + 2, j - 2) == 'A' &&
         get_char(grid, i + 3, j - 3) == 'S';
}

bool check_left(CharGrid *grid, size_t i, size_t j) {
  assert(grid != NULL);
  assert(i < grid->height);
  assert(j < grid->width);

  return j >= 3 && get_char(grid, i, j) == 'X' &&
         get_char(grid, i, j - 1) == 'M' && get_char(grid, i, j - 2) == 'A' &&
         get_char(grid, i, j - 3) == 'S';
}

bool check_up_left(CharGrid *grid, size_t i, size_t j) {
  assert(grid != NULL);
  assert(i < grid->height);
  assert(j < grid->width);

  return i >= 3 && j >= 3 && get_char(grid, i, j) == 'X' &&
         get_char(grid, i - 1, j - 1) == 'M' &&
         get_char(grid, i - 2, j - 2) == 'A' &&
         get_char(grid, i - 3, j - 3) == 'S';
}

bool check_up(CharGrid *grid, size_t i, size_t j) {
  assert(grid != NULL);
  assert(i < grid->height);
  assert(j < grid->width);

  return i >= 3 && get_char(grid, i, j) == 'X' &&
         get_char(grid, i - 1, j) == 'M' && get_char(grid, i - 2, j) == 'A' &&
         get_char(grid, i - 3, j) == 'S';
}

int main(int argc, char **argv) {
  if (argc < 2) {
    fprintf(stderr, "Usage: %s <path>\n", argv[0]);
    return 1;
  }

  CharGrid *grid = char_grid_new(argv[1]);
  if (grid == NULL) {
    return 1;
  }

  size_t count = 0;
  for (size_t i = 0; i < grid->height; i++) {
    for (size_t j = 0; j < grid->width; j++) {
      char c = get_char(grid, i, j);
      if (c == 'X') {
        if (check_up_right(grid, i, j)) {
          count++;
        }
        if (check_right(grid, i, j)) {
          count++;
        }
        if (check_down_right(grid, i, j)) {
          count++;
        }
        if (check_down(grid, i, j)) {
          count++;
        }
        if (check_down_left(grid, i, j)) {
          count++;
        }
        if (check_left(grid, i, j)) {
          count++;
        }
        if (check_up_left(grid, i, j)) {
          count++;
        }
        if (check_up(grid, i, j)) {
          count++;
        }
      }
    }
  }

  printf("%zu\n", count);

  free_char_grid(grid);
  return 0;
}
