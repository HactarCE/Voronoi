# Voronoi Cell Visualization

This is a simple program that draws [Voronoi cells](https://en.wikipedia.org/wiki/Voronoi_diagram) for a set of points. There are options to use different [Lp distance metrics](https://en.wikipedia.org/wiki/Lp_space) or display the [farthest-point Voronoi cells](https://en.wikipedia.org/wiki/Voronoi_diagram#Farthest-point_Voronoi_diagram) instead.

<img src="https://i.imgur.com/RfbhBno.png" alt="Voronoi diagram for a set of six points" />

## Building on Linux or macOS

1. Download/install Cargo.
2. Clone this project and build/run:

```sh
git clone https://github.com/HactarCE/Voronoi
cd Voronoi
cargo run --release
```


## Building on Windows

1. Download/install [Rustup](https://www.rust-lang.org/tools/install).
2. Run `rustup.exe toolchain install stable-x86_64-pc-windows-msvc` to install the MSVC toolchain.
3. Run `rustup.exe default stable-msvc` to select that toolchain as the default.
4. Download this project and extract it somewhere.
5. Open a terminal in the folder where you extracted the project (it should have `Cargo.toml` in it) and build it using `cargo build --release` or run it using `cargo run --release`.
