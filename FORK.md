# This fork

`main` is a merge of [ZSlayerHQ/victory-world-runtime](https://github.com/ZSlayerHQ/victory-world-runtime)
branch `next` (`99f8834`, *"both public forks merged, with the walls fix confirmed on an RTX 5090"*)
onto `310c6eb`, with this fork's changes on top. No game files or assets are included.

## This fork's changes on top of `next`

- `VICTORY_GROUP_SYNC` defaults to `2` (every barrier waits); `auto`/`3` is opt-in. #608's
  automatic mode is still available, but its default triggered flicker on this machine.
- `victory translate <shaders.rpf> --warm`: translates the whole shader pack into the cache a
  run reads. A second pass finds it all; the run starts with the cache warm.
- PRs #662 (view-heap limit via `D3D12_OPTIONS19`), #654 (skip early build), #608 (narrow-wave
  barriers + host wave in the cache key), #615 (`bpermute` cross-half scratch).
- Pipeline workers: `VICTORY_PIPELINE_WORKERS`, `VICTORY_PIPELINE_PRIORITY`.
- `VICTORY_FRAMETIME_DUMP=<path>`: a CSV of per-interval frame buckets plus the worst/average
  single frame. Standalone patch and samples:
  [skornuta/victory-frametime](https://github.com/skornuta/victory-frametime).

## Tested

RTX 5080 16 GB, Ryzen 7 9800X3D, 32 GB, Windows 11, 1440p, Series X profile.

Settled world on the merged build: ~4.1–4.4 fps / 227–260 ms, 4,400–4,800 draws/frame.
The frametime dump puts the 1–2 s drops on `heap_ms` (the by-index resource-table pass) and
`textures_ms`, not the GPU; `copies_ms` + `copyback_ms` are ~27% of a frame. See
`victory-frametime/samples/report.md` in that repo for the numbers.

## Build and run

```sh
cargo build --release -p victory
victory translate <game>\shaders.rpf --warm
victory run <game>\<title>.exe --execute
```

Env used for the measurements (see `victory-frametime` for the dump):

```
VICTORY_PIPELINE_WORKERS=8
VICTORY_PIPELINE_PRIORITY=normal
VICTORY_SHADER_LOOP_LIMIT=1024
VICTORY_GROUP_SYNC=2
VICTORY_RELAY=2
VICTORY_LATE_WAIT=3
VICTORY_WIDE_BACK=1
VICTORY_CARD_BUFFERS_MIN_MB=1
VICTORY_CARD_BUFFERS_MB=1024
VICTORY_CARD_REHASH_RUNS=1
VICTORY_JOIN_COPIES=1
VICTORY_FRAMETIME_DUMP=%TEMP%\victory-frametime.csv
```

## Credit

VIctory is by [Marvisak](https://github.com/Marvisak/VIctory) and contributors; the community
fixes merged here are the work of their authors. GPL-3.0, following upstream.
