# DIFFUSER v0.1.0
_**Categories:** reverb, meta-plugin_

## Installation
_**Disclaimer:** this plugin will only work on 64-bit windows machines!_ \
Download the `.dll` file in the `bin/` directory and place it into your DAW's VST folder.
Previous versions of the plugin are also available, in case you need them.

## Compiling The Source Code
_**Note:** you don't need to compile the source code if you just want to use the plugin, just download the `.dll`._ \
Make sure you have Cargo installed on your computer (the Rust compiler). Then in the root of the repository run `cargo build`. Once Cargo is done building, there should be a `DIFFUSER_v0_1_0.dll` file in the newly created `debug/` directory. Place this file into your DAW's VST folder.

# What is DIFFUSER?
DIFFUSER is a TDL (tapped delay line) diffuser which is a component in the making of digital reverbs. This plugin is a proof-of-concept to show off some of the technology I'm developing for an upcoming plugin (a room reverb mixed with an extended Karplus-Strong resonator).

Essentially a diffuser is the simplest form of reverberation possible: a bank of dozens of delays, with prime lengths, as to minimize resonances. Here each stereo channel has different taps, and additionally, the left and right channels can cross-over between each other. There is also feedback control, which turns the diffuser into an exponentially decaying basic reverb.

Note that the feedback is very unstable and might run off, in that case, to prevent damage, there is a soft-clipper in the feedback path. This sof-clipper can be abused intentionally to create harsh distorted high-pitched tones.

## Controls Explained
+ Length: length of the diffusion (controls the number of delay taps)
+ Crossover: amount of crossover between left and right channels
+ Feedback: amount of feedback around the diffuser
+ Dry/wet: controls the mix between the unprocessed (dry) and processed (wet) signals.
