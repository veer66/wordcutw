# Moved to https://codeberg.org/mekong-lang/wordcutw

# wordcutw

A C-interface wrapper for Wordcut - a Lao/Thai word segmentation/breaking library

## Install

```
git clone git@github.com:veer66/wordcutw.git
cd wordcutw
cargo build --release
sudo cp target/release/libwordcutw.so /usr/local/lib
```

## Example

```C
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
```
