# spell

Ever used `look` because you genually don't know how something is spelled?  
Getting tired of having to nail the spelling every time?

`look` no further... Because this is `spell`.

## How to use it:

```
./spell hwllo
hello
halloo
hallow
hellos
hollow
howell
wallop
./spell amazin
amazing
amazingly
amazonian
amazonian
dramatizing
automatizing
traumatizing
```

Usage: `spell [-v] [-n%] [word] [file ...]`

More advanced examples:

```
./spell -50% nessecary
undersecretary
undersecretary's
necessary
necessarily
necessary's
unnecessary
concessionary
./spell -v hallo
(100% match):	halloo
(100% match):	hallow
(100% match):	halloos
(100% match):	hallows
(100% match):	shallot
(100% match):	shallow
(100% match):	halloo's
./spell -a hwllo
*huge output here*
```

## Installing

Download [rust](https://rust-lang.org/) and compile it yourself using  
```
cargo build --release
```

## How does it work?

spell uses [rust-lcs](https://github.com/ucarion/rust-lcs) in order to compare words.
