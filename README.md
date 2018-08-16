# subconv
Simple, but totally usable, project written to try coding in Rust.

Converts subtitles from MicroDVD (.txt) to SubRip (.srt). Subtitles are encoded in `utf-8`.


## Sample usage
```
find /Volumes/EXT_DISK/Awesome\ Series/*.txt -print0 | xargs -0 cargo run -- --encoding windows-1250
```
