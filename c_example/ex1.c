#include <stdio.h>
#include "wordcutw.h"

int
main()
{
  Wordcut *wordcut = wordcut_new_with_dict_from_default_dir("data/thai.txt");
  size_t range_count = 0;
  TextRange* text_ranges = wordcut_into_text_ranges(wordcut, "ลากา", &range_count);
  printf("COUNT = %zu\n", range_count);
  printf("R0 %zu_%zu\n", text_ranges[0].s, text_ranges[0].e);
  printf("R1 %zu_%zu\n", text_ranges[1].s, text_ranges[1].e);
  delete_text_ranges(text_ranges, range_count);
  delete_wordcut(wordcut);
  return 0;
}
