# Sort the $PATH to be something reasonable

I cannot believe I have to write this utility. MacOS rearranges the `$PATH`
variable for interactive shells. This becomes a big problem when using
[Nix](https://nixos.org) because we want to keep a pretty consistent environment
between our interactive and non-interactive shells. At the same time the utility
that does the `$PATH` fuckery is `/usr/libexec/path_helper`. From what I can
tell it has no options other than to output either `sh` or `csh` compatible
syntax with the `-s` and `-c` flags respectively. It is invoked by
`/etc/zprofile` and `/etc/profile`. I do not want to change these because I do
not know what gui apps, for example, rely on the system versions of python.


## Sort order

1. Paths in the `$HOME` folder
2. `/local` paths
3. `/usr` paths
4. Root `/` paths

Within each of these we want `*/bin/` to be searched before `*/sbin/`.


## Usage

* `-a`: append a path to its relative section
* `-p`: prepend a path to its relative section
* `-A`: Append a path to the end. (Not sorted)
* `-P`: Prepend a path to the beginning. (Not sorted)
* `-D`: do not deduplicate paths
* `-S`: do not sort paths


### Examples

Sort the existing `$PATH` variable
```shell
export PATH="$(pathtool)"
```

Append `/foo/bin` to the end `$PATH` variable
```shell
export PATH="$(pathtool -A /foo/bin)"
```

Add `/bar/bin` to the sorted paths
```shell
export PATH="$(pathtool -a /bar/bin)"
```
