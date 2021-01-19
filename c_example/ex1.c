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

  size_t string_count = 0;
  char **tokenized_strings = wordcut_into_strings(wordcut, "ลากา", &string_count);
  size_t i;
  for (i = 0; i < string_count; i++)
    {
      printf("String #%zu: %s\n", i, tokenized_strings[i]);
    }
  delete_strings(tokenized_strings, string_count);
  
  char *tokenized_text = wordcut_put_delimiters(wordcut, "ลากา", "---");
  printf("Tokenized text = %s\n", tokenized_text);
  free(tokenized_text);
  delete_wordcut(wordcut);
  return 0;
}
