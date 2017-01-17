# (WIP) Japanese Command-line Dictionary

Basic usage (not functional yet):

```shell
# convert text to kana and add it to the dictionary with the meaning after "-m"
jcd add -r hiragana -m Cursive syllabary used in the japanese language
jcd add -h hiragana -m Cursive syllabary used in the japanese language
# messed up case also works
jcd add -r KATakANA -m Angualar syllabary used in the japanese language
jcd add -k KATakANA -m Angualar syllabary used in the japanese language
# search the dictionary for a word
jcd search -r ushi # auto detects kana to use
jcd search -h ushi # converts to hiragana (ignores case)
jcd search -k ushi # converts to katakana (ignores case)
# convert arguments to kana and print to stdout
jcd convert -r KATAKANA ga wakaru # auto detects
jcd convert -h KATAKANA ga wakaru # converts to hiragana
jcd convert -k KATAKANA ga wakaru # converts to katakana
```

See [main.rs][main] for more details.

## Convertion method

Convertion uses the [Modified Hepburn system][0], except for
syllables with the [macron][1] or [circumflex][2], which is
not implemented yet.
Hyphen (`-`) is always converted to the [chouonpu][3] (`ー`).
`m` (or `M`) is never converted to `ん` (or `ン`).

### Some examples:

| Romaji                    | Kana                       |
| :-----------------------: | :------------------------: |
| `hiragana`                | `ひらがな`                 |
| `KATakANA` and `KATAKANA` | `カタカナ`                 |
| `CHOKORE-TO`              | `チョコレート`             |
| `nippon`                  | `にっぽん`                 |
| `kan'i`                   | `かんい`                   |
| `kani`                    | `かに`                     |
| `shin'you`                | `しんよう`                 |
| `shinyou`                 | `しにょう`                 |
| `maccha` (not `matcha`)   | `まっちゃ`                 |
| `matcha`                  | `まtちゃ`                  |
| `kka ssa tta hha rra`     | `っか っさ った っは っら` |

## Notes about the project and Rust

I'm using this project as an exercise to learn [Rust][4] but
I don't plan to drop it at least until it has all the
functionality I want.

As I'm pretty new to [Rust][4] you'll probably find lots of
things that could be improved. As I learn more about it and
how to use it hopefully they'll be changed. If not, don't
let that put you off!

[0]: https://en.wikipedia.org/wiki/Hepburn_romanization#Variants_of_Hepburn_romanization
[1]: https://en.wikipedia.org/wiki/Macron
[2]: https://en.wikipedia.org/wiki/Circumflex
[3]: https://en.wikipedia.org/wiki/Ch%C5%8Donpu
[4]: https://www.rust-lang.org
[main]: src/main.rs
