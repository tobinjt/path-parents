# path-parents

`path-parents` prints all the parents of each path given, optionally skipping
parents up to the length specified by `--skip`.

## Installation

`path-parents` is written in Rust and so needs a Rust toolchain for
installation. See <https://www.rust-lang.org/tools/install> for how to install
Rust. When Rust is installed `path-parents` can be installed with:

```shell
# This will not work until the crate is published.
cargo install path-parents
```

## Usage

```shell
$ path-parents /usr/bin/less /usr/bin/more
/usr
/usr/bin
/usr/bin/less
/usr
/usr/bin
/usr/bin/more

$ path-parents -s 1 /usr/bin/less /usr/bin/more
/usr/bin
/usr/bin/less
/usr/bin
/usr/bin/more

$ (echo /usr/bin/cat; echo /usr/bin/cut ) | path-parents
/usr
/usr/bin
/usr/bin/cat
/usr
/usr/bin
/usr/bin/cut

# Note that this will not work as you hope, and path-parents doesn't try to
# prevent you doing this.
$ echo /usr/bin/cat /usr/bin/cut | path-parents
/usr
/usr/bin
/usr/bin/cat
/usr/bin/cat /usr
/usr/bin/cat /usr/bin
/usr/bin/cat /usr/bin/cut
```

## Licence

Licensed under the Apache 2.0 licence, see the LICENSE file accompanying the
software.
