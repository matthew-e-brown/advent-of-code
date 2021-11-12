# Day 5: challenge mode

I wanted to challenge myself, so as much as this day is literally two `grep` one
liners:

```console
$ cat puzzle-input.txt | grep -P '(.)\1' | grep ... | wc -l
$ cat puzzle-input.txt | grep -P '(..).*\1' | grep ... | wc -l
```

I really wanted to try doing it with a custom parser. I may have gone a bit
overboard.