# victory-fork — V1 performance build (RTX 5080)

A GPL-3.0 fork of [VIctory](https://github.com/Marvisak/VIctory), now tracking the
**ShakiraRED V1** performance line (a fork of [Trexies/VIwork](https://github.com/Trexies/victory-fork/tree/VIwork)),
plus this fork's dump/data tooling. No game files or assets are included or required.

## Tested on this machine

| | |
|---|---|
| **CPU** | AMD Ryzen 7 9800X3D — 8 cores / 16 threads |
| **GPU** | NVIDIA GeForce RTX 5080 — 16 GB, driver 621.24 |
| **RAM** | 32 GB DDR5-6400 |
| **OS** | Windows 11 Home, 10.0.26300 |
| **Display** | 2560 × 1440, Series X profile |

## Performance

Measured on the V1 core, `VICTORY_AMBIENT=0.6`, `CARD_BUFFERS_MB=4096`,
`CARD_WINDOWS_MB=4096`, `WIDE_MERGE=1`, warm shader/pipeline caches:

| | |
|---|---:|
| fps | **17.8 median** (17.6 mean) |
| ms/frame | **56.3 median** |
| draws | 6,679 median |
| GPU wait | 14.7 ms |
| copy-back | 19.9 ms |
| preparing | ~28 ms |
| copies | ~8 ms |

Last samples: 22.6 fps @3,014 draws, 18.9 @3,426, 19.5 @3,601. The card runs
75–80% busy; the remaining limit is translated-shader cost, not CPU. For
comparison, the previous line (Trex + our patches) measured 4.7 fps / 226 ms at
5,636 draws.

## Build

```sh
cargo build --release -p victory
```

Warm the caches once (the run reuses them):

```bat
set VICTORY_HOST_WAVE=32
set VICTORY_VIEW_LIMIT=999999
target\release\victory.exe warm "<game directory>" --jobs 16
```

`VICTORY_HOST_WAVE`/`VICTORY_VIEW_LIMIT` must match the run's host: 32-lane
NVIDIA cards use 32 and 999999 (see the run log's `host wave lanes` and
`view heaps hold` lines). Then run:

```bat
run-v1.cmd "<game>\game_scarlett_master.exe"
```

The checked-in `run-v1.cmd` carries the tested settings. `run-v1-2048.cmd` is an
experiment that gives 4 GB of card-side buffers back to textures (useful if
textures are missing; the default 4096 measured better in the settled scene).

## What is in this tree

ShakiraRED V1 on the victory-world/Trex line:
- `ambient.rs` — a constant replacement for the missing indirect light
  (`VICTORY_AMBIENT=0.6`; `0` leaves the title's output).
- `heap_pass.rs` — incremental descriptor-heap scanning.
- `plaincopy.rs` + `watch.rs` — own-view copy-back, read/write locks, batching.
- `host.rs` / `queue.rs` — async submit, fill-ahead, relay/copy-queue timing.
- Card buffers, indirect windows, wide merge, copy queue, fold buffers.
- Our `dumps/` captures and the warm-cache key fix for NVIDIA hosts.

## Known issues

- **~350–450 draws/frame are dropped** by translator refusals: “whole-wave
  section step”, “first-lane read where not every lane runs”,
  `v_permlanex16_b32`, “descriptor address/format read as data”, “lane mask
  exchanged”, and several `E_INVALIDARG` compute pipelines. These are visual
  gaps (missing effects), not cache misses.
- The flat ambient is a workaround for the missing ray-traced indirect light;
  shadowed areas are lit uniformly rather than truly.
- One 4090 run in the V1 author's tests ended with `DXGI_ERROR_DEVICE_HUNG`.
- Planar reflections are the game's own `mirror_*` pass and are being worked on
  separately.

## Credits

ShakiraRED V1 by its contributors, on [Trexies/VIwork](https://github.com/Trexies/victory-fork/tree/VIwork),
[VIctory](https://github.com/Marvisak/VIctory) and the community forks
(myokex/victory-amd, fxceless/victory-fork, ZSlayerHQ/victory-world-runtime).
GPL-3.0-or-later, following upstream.
