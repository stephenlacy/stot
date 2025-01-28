# stot
> like stat but more human


### Usage:
```
stot ./src/main.rs
file 100644 (rw-r--r--) 3.47 kB "2025-01-27 23:26" - ./src/main.rs


ls | stot
file 100644 (rw-r--r--) 6.47 kB "2025-01-27 23:01" - Cargo.lock
file 100644 (rw-r--r--) 186 B "2025-01-27 23:01" - Cargo.toml
file 100644 (rw-r--r--) 1.07 kB "2025-01-27 23:30" - LICENSE
file 100644 (rw-r--r--) 214 B "2025-01-27 23:29" - README.md
directory 40755 (rwxr-xr-x) 96 B "2025-01-27 16:42" - src
directory 40755 (rwxr-xr-x) 192 B "2025-01-27 22:59" - target
```

Where `stat` gives machine numbers, `stot` gives human readable values.

### Installation

```
cargo install --git https://github.com/stephenlacy/stot

```
