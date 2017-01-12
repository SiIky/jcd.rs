# (WIP) Japanese Command-line Dictionary

Basic idea of use (not functional yet):

```shell
# convert a word to kana and add it to the dictionary with the meaning after "-m"
jcd add hiragana -m Cursive syllabary used in the japanese language
# messed up case also work
jcd add KATakANA -m Angualar syllabary used in the japanese language
# search the dictionary for a word
jcd search ushi
# convert arguments to kana and print to stdout
jcd r2k KATAKANA ga wakaru
```

## Convertion method

Convertion uses the [Modified Hepburn system][0], except for
syllables with the [macron][1] or [circumflex][2], which is
not implemented yet.
Hyphen (`-`) is always converted to the [chouonpu][3] (`ー`).
`m` (or `M`) is never converted to `ん` (or `ン`).

Some examples:

| Romaji                            | Kana                                |
| :-------------------------------: | :---------------------------------: |
| **hiragana**                      | **ひらがな**                        |
| **KATakANA** and **KATAKANA**     | **カタカナ**                        |
| **CHOKORE-TO**                    | **チョコレート**                    |
| **nippon**                        | **にっぽん**                        |
| **kan'i**                         | **かんい**                          |
| **kani**                          | **かに**                            |
| **shin'you**                      | **しんよう**                        |
| **shinyou**                       | **しにょう**                        |
| **rarirurero** and **lalilulelo** | **らりるれろ** (Same with katakana) |
| **maccha** (not **matcha**)       | **まっちゃ**                        |
| **matcha**                        | **まtちゃ**                         |
| **kka ssa tta hha rra**           | **っか っさ った っは っら**        |

[0]: https://en.wikipedia.org/wiki/Hepburn_romanization#Variants_of_Hepburn_romanization
[1]: https://en.wikipedia.org/wiki/Macron
[2]: https://en.wikipedia.org/wiki/Circumflex
[3]: https://en.wikipedia.org/wiki/Ch%C5%8Donpu
