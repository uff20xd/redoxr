# Redoxr
Redoxr is a build system library based on nob.h[https://github.com/tsoding/nob.h] by Tsoding and the Zig[https://ziglang.org/] build system.

**You probably shouldnt use this in production as it is currently not stable and not made for anything serious!!**
Also my code is a mess, so its performance is probably subpar, even when compared to cargo.

## Usage

```build.rs
mod redoxr;
use redoxr::redoxr::*;

fn main() -> MainResult {
    //For configuring and rebuilding the script
    let _redoxr = Redoxr::new(
        "--cfg", "quiet",
        "--cfg", "run",
    );
}

```

## Refrences

The Zig Build System - https://ziglang.org/learn/build-system/

nob.h - https://github.com/tsoding/nob.h

Nix Derivations - https://nix.dev/manual/nix/2.22/language/derivations

Nix Flakes - https://nix.dev/manual/nix/2.28/command-ref/new-cli/nix3-flake
