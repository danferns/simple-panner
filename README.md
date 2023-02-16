# Simple Panner

Simple Panner is a stereo panning plugin that I built as a way to learn plugin development using the amazing [NIH-plug](https://github.com/robbert-vdh/nih-plug) framework and the [VIZIA](https://github.com/vizia/vizia) UI library.

![Plugin Screenshot](./Screenshot.png)

## How to use

Simple Panner has two parameters, `Pan` and `Channel Mix`.

`Pan` controls the gain of the left and right channels.

`Channel Mix` controls the mixing of the two channels _before_ panning the sound.

-   When set to `0%`, audio from one channel will not be mixed into the other channel. Panning will purely control the gain of each channel. This will sound wider, but still allow you to control the stereo balance of the track.

-   At `100%`, both channels are mixed into each other evenly. This is equalivalent to converting the audio to mono and then panning it, offerring a more localized sound.

## Building

After installing [Rust](https://rustup.rs/), you can compile Simple Panner as follows:

```shell
cargo xtask bundle simple_panner --release
```
