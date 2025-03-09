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

There are no pre-built binaries available, contributions to provide binaries are
welcome.

## Usage

### Basic usage

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
# prevent you doing this because filenames can contain spaces.
$ echo /usr/bin/cat /usr/bin/cut | path-parents
/usr
/usr/bin
/usr/bin/cat
/usr/bin/cat /usr
/usr/bin/cat /usr/bin
/usr/bin/cat /usr/bin/cut
```

### More typical usage

`path-parents` is mostly useful for expanding a path you're having trouble with
for debugging purposes. E.g. if access is denied or the file doesn't exist, you
can figure out where the problem is with:

```shell
ls -l -d $(path-parents /path/with/problems)
```

E.g.:

```shell
$ ls -l -d $(path-parents /opt/homebrew/Cellar/rustup/*/share/zsh/site-functions/_rustup)
drwxr-xr-x    3 root       wheel     96 10 Dec  2022 /opt/
drwxr-xr-x   33 johntobin  admin   1056 28 Feb 22:00 /opt/homebrew/
drwxrwxr-x  151 johntobin  admin   4832  8 Feb 15:36 /opt/homebrew/Cellar/
drwxr-xr-x@   3 johntobin  admin     96  8 Mar 14:59 /opt/homebrew/Cellar/rustup/
drwxr-xr-x@  14 johntobin  admin    448  8 Mar 14:59 /opt/homebrew/Cellar/rustup/1.28.1/
drwxr-xr-x@   4 johntobin  admin    128  5 Mar 11:33 /opt/homebrew/Cellar/rustup/1.28.1/share/
drwxr-xr-x@   3 johntobin  admin     96  5 Mar 11:33 /opt/homebrew/Cellar/rustup/1.28.1/share/zsh/
drwxr-xr-x@   3 johntobin  admin     96  5 Mar 11:33 /opt/homebrew/Cellar/rustup/1.28.1/share/zsh/site-functions/
-rw-r--r--@   1 johntobin  admin  55464  5 Mar 11:33 /opt/homebrew/Cellar/rustup/1.28.1/share/zsh/site-functions/_rustup
```

If there permission problems or missing files it will be easy to see where the
problem starts.

## Licence

Licensed under the Apache 2.0 licence, see the [`LICENSE`](LICENSE) file
accompanying the software.
