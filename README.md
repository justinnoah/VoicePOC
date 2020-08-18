# STT POC
This untitled project is a simple Speech-to-Text proof of concept. The project
uses SDL2 to get a microphone, then dumps the data into DeepSpeech for
recognoition. Using methods on Audio devices in SDL2, I was able to have SDL2
provide a data stream as raw wave i16s for DeepSpeech 0.7.4.

### Dependencies
- SDL2, SDL2_ttf >= 2.0.5
- DeepSpeech 0.7.4
- Rust 1.45.0 (older versions may work, but untested)


#### To build
- Rust stable
- DeepSpeech Library in your path
  - I used `pip install --user deepspeech==0.7.4` then added 
   `~/.local/lib/python3.8/site-packages/deepspeech/lib` to my `LD_LIBRARY_PATH`,
   which is needed for linking
- SDL2 and SDL2_ttf are needed in the library path as well, usually provided
  by your distribution of choice (apt, dnf, emerge, chocolatey, etc).
- Download the deepspeech model as [deepspeech.pbmm](https://github.com/mozilla/DeepSpeech/releases/download/v0.7.4/deepspeech-0.7.4-models.pbmm) and the scorer as [deepspeech.scorer](https://github.com/mozilla/DeepSpeech/releases/download/v0.7.4/deepspeech-0.7.4-models.scorer). and place  them in the top level of the checkout, e.g. /path/to/cloned/repo/.

#### To use
- > cargo run
- spacebar
- say something
- spacebar
- See what DeepSpeech heard
- Escape key to exit

### Roadmap, In order from top -> bottom
- Move to DeepSpeech 0.8.x
- Code re-org
  - Need to get most everything out of main().
- Streaming audio instead of listen then interpret.
- VAD: Trigger words for listening instead of spacebar
- Settings
  - Font
  - Color/Style
  - Size
- Availability, produce binaries for:
  - Android
  - Linux
  - *BSD
  - MacOSX
  - Windows
  - iOS (maybe)
