# zm 🦘
*cd* for lazy people who don't care where they are, or how to get where they're going

## What is this?

Like most developers, I'm a pretty lazy person, and remembering which folder I'm in (even though it prints out in my shell) is too much for my precious little head to handle.

*zm* is a command line tool intended as a convenience layer on top of *cd*.

Install it like this (you'll need cargo preinstalled):

```bash
git clone https://github.com/benrutter/zm
cd zm
cargo build --release
cargo install --path .
```

That installs a runnable called *zoom*, us it like this:

```bash
$_ zoom some_dir
> /home/you/Documents/here_is_some_dir
```

The *zoom* command will look for a directory that ends with the string you gave:
- In your current folder
- Recursively from your current folder
- Recursively from your home directory
Once it finds a match, it'll stop looking and return the path. If it can't find a match, it'll just pass back what you gave it.

That means if you add something like this to your **.zshrc**:

```zsh
zm() {
  dir=$1
  cd "$(zoom $dir)"
}
```

Then you'll have a shiny new tool you can use like "cd".

Typing "zm some_dir" will transport you to that directory based on the logic of zoom.

The code is minimal and will search quickly. This might not be a suitable tool for an *extremely large* filesystem, like some kinds of servers.

## Bonus! (moving up)

*zm* will resolve `..3` to `../../..` to navigate you up several directories.
