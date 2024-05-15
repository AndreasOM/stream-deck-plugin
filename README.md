# Stream Deck - Plugin

*Warning:* This is in very early development!

A quick wrapper to make it easier to develop stream deck plugins in rust.

## Platforms
Only macOS has been tested, since I don't have a windows system,
but a crossbuild should be easy to do,
and you can override the binary per platform in the manifest.


## Implemented Responses (from stream deck to plugin)
- [x] WillAppear
- [x] KeyDown

## Implemented Requests (from plugin to stream deck)
- [x] registerPlugin
- [x] setState
- [x] setTitle
- [x] setImage


Sorry, no documentation yet.
Look at [streamdeck-plugin-template](https://github.com/AndreasOM/stream-deck-plugin-template) to get started.
