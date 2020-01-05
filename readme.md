# One-shot window tiling/aligning program

So I wanted nice window layouts without having to bother with tiling WMs (I love my Openbox setup way too much).
Zentile just wasn't right for me (it stays on as a background process all the time,
makes you move your tiling-dedicated keyboard shortcuts into a separate file).

Tiler is a "one-shot" script type - you bind it into a keyboard shortcut,
then activate it when you want to align your windows.

## Usage

Tiler will automatically align all windows in your workspace.

`tiler` - standard settings

`tiler big` - bigger windows, smaller gaps


## Installation
* Clone the repo
* `cargo build --release`
* Copy the result binary from `target/release/tiler` into some convenient place
* Bind the binary execution to your keyboard shortcut of choice
