# velogos
A command line program to help you learn to type faster written in rust.

## How it works

The program uses a [DAWG](https://pages.pathcom.com/~vadco/dawg.html) to generate words. The words can be inputed from a file (if you are bad at some words) or the most common words are provided in 1000-words. All the words will be used in building a tree of all the letters. From there we pick each letter at random, and get words from the tree.
Statistics is to be implemented as of today but in the works. 