redirect
========

Redirect stdin lines matching a certain prefix to a file.

If you pipe some output to redirect, it will redirect lines that match a given
pattern to a file. For example, if you had a program called "program1" which
generates the following output:

```
Hello
> This text will be redirected
This text will not be redirected
```

If you piped the output of this program into redirect, like so:

```
program1 | redirect
```

The following would be printed to stdout:

```
Hello
This text will not be redirected
```

The following would be written to the file "redirected":

```
> This text will be redirected
```

# Compiling from source

You'll need git to download the source and rust and cargo to compile. On Arch:

```
$ sudo pacman -S git rust cargo
```

On Debian/Ubuntu/Mint/etc:

```
$ sudo apt-get install git rust cargo
```

Download the source repository:

```
$ git clone https://github.com/briansteffens/redirect
$ cd redirect
```

Compile and install:

```
# make install
```
