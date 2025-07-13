# space - open multiple apps with the CLI

<!-- markdownlint-disable MD040 -->

`space` is a tool that opens multiple apps with a simple command. By creating a
space, you can add your apps to it and launch them all apps in it at the same
time.

## Install

> [!WARNING]
> This software is still in development. Official versions may be incompatible.

You can install `space` with
[cargo](https://doc.rust-lang.org/stable/cargo/getting-started/installation.html).

```shell
cargo install --git https://github.com/calejvaldez/space
```

## Usage

`space` opens multiple apps in a space. A space is a collection of apps.

### Create a space

```
space init <name>
```

### Add an app to a space

> [!NOTE]
> This opens the file picker! Choose the app you want to open.
>
> Optionally, add the path at the end.

```
space add <space> <app-label>
```

### Open all apps in a space

```
space open <space>
```

### List all spaces and apps

```
space ls
```

## Contributing

Feel free to contribute through PRs!

## License

This is free and open source software licensed under the GNU General Public
License v3. For more information, see the license file.
