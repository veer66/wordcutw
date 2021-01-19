#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct TextRange {
  uintptr_t s;
  uintptr_t e;
} TextRange;

const void *wordcut_new_with_dict(const char *path);

const void *wordcut_new_with_dict_from_default_dir(const char *path);

void delete_wordcut(const void *wordcut);

void delete_text_ranges(struct TextRange *text_ranges, uintptr_t range_count);

struct TextRange *wordcut_into_text_ranges(const void *wordcut,
                                           const char *text,
                                           uintptr_t *range_count);
