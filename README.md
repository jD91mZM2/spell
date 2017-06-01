# spell

Ever used `look` because you genually don't know how something is spelled?  
Getting tired of having to nail the spelling every time?

`look` no further... Because this is `spell`.

## How to use it:

```
./spell hwllo
hello
./spell amazin
amazing
amazon
amazon
amazons
amazons
```

Usage: `spell [-v] [-n%] [word] [file ...]`

More advanced examples:

```
./spell -50% nessecary
bessemer
jessica
jessica's
secretary
centenary
mercenary
necessary
secretary
bessemer's
nesselrode
./spell -v hallo
(83.33333% match):	hallow
(80% match):	gallo
(80% match):	hall
(80% match):	hall
(80% match):	halls
(80% match):	hello
(71.42857% match):	hallows
```

## Installing

Download [rust](https://rust-lang.org/) and compile it yourself using  
```
cargo build --release
```
