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

# Drawbacks and plans

Obviously, this was just a quick test project for now.  
I might end up updating it to include common spelling mistakes and such.

A major problem with `spell` currently, is the `nessecary` and `necessary` problem.  
Look at it this way.

As a human, you would think the difference between the two words are 3 characters.
```
nessecary
necessary
  ^ ^^
```

However, as a computer, `spell` thinks it's actually 4 character.

```
nessecary
necessary
  ^^^^
```

It does not handle incorrect ordering orders very well.  
One of the most simpilest approaches to handle this could be to favor the beginning of words, and assume the first few characters are always correct.  
That is usually the case, and might filter away stuff like `jessica` that pop up.
