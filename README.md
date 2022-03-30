### WIP PCM and beyond encoder
```
cargo run
```
to play result file:
```
vlc --demux=rawaud --rawaud-channels 1 --rawaud-samplerate 44100 test.pcm
```
TODO:
* Refactor - code splitting, quantizer casting, etc.
* Write to WAV instead of raw PCM
* Add support for other encoders
* Read notes from some kind of sheet
* Add some common synthesizer features