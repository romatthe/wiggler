Wiggler
=======

`Wiggler` is a command-line application for periodically wiggling your mouse. Do your admins control your power settings, but don't want to dim your screen? Just wiggle your mouse with `Wiggler`.

![Wiggler!](https://www.mariowiki.com/images/thumb/7/7c/WigglerDS.png/250px-WigglerDS.png)

Get Wiggler
-----------

If you're a Rust Programmer, you can install `Wiggler` using Cargo:

```
$ cargo install --git https://github.com/romatthe/wiggler
```

Otherwise, you can find a precompiled binary [here](https://github.com/romatthe/wiggler/releases)

Usage
-----

```
$ wiggler --help
USAGE:
    wiggler --time <DURATION>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --time <DURATION>    Set the interval between each mouse wiggle
```

Example
-------

Move the mouse each 30 seconds

```
$ wiggler --time 30
```

License
-------

See [LICENSE](LICENSE).