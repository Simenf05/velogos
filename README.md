# velogos
A command line program to help you learn to type faster written in rust.

## How it works

The program uses a [DAWG](https://pages.pathcom.com/~vadco/dawg.html) to generate words. The words can be inputed from a file (if you are bad at some words) or the 1000 most common words can be used. All the words will be used in building a tree of all the letters. From there we pick each letter at random, and get words from the tree.
