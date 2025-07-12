# Redoxr
Redoxr is a build system library based on [nob.h](https://github.com/tsoding/nob.h) by Tsoding and the [Zig](https://ziglang.org/) build system.

**You probably shouldnt use this in production as it is currently not stable and not made for anything serious!!**
Also my code is a mess, so its performance is probably subpar, even when compared to cargo.

## Usage

```Rust
mod redoxr;
use redoxr::redoxr::*;

fn main() -> MainResult {
    //For configuring and rebuilding the script
    let _redoxr = Redoxr::new(
        "--cfg", "quiet",
        "--cfg", "run",
    );

    let dep1 = RustCrate::from_cargo("serde", "./serde");
    let dep2 = RustCrate::new("settings_manager", "./settings_manager")
        .flags(&[
            "--edition", "2024",
            "--Coptlevel", "3",
        ])
        //Pass your dependencies as a mutable pointer
        .depend_on(&mut dep1)
        .stay();
    let main_crate = RustCrate::new("Some_project", ".")
        .flags(&[
            "--edition", "2024",
            "--Coptlevel", "3",
        ])
        .depend_on(&mut dep2)
        .make_output()
        .make_bin()
        .stay();

    compile!(dep1);
    compile!(dep2);
    compile!(main_crate);
    //only works if --cfg run is enabled
    run!(main_crate);
}

```

## Refrences

The Zig Build System - https://ziglang.org/learn/build-system/

nob.h - https://github.com/tsoding/nob.h

Nix Derivations - https://nix.dev/manual/nix/2.22/language/derivations

Nix Flakes - https://nix.dev/manual/nix/2.28/command-ref/new-cli/nix3-flake
