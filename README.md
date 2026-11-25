# ShakiraRED(dt) — VIctory V1

An experimental performance fork of [Trexies/VIwork](https://github.com/Trexies/victory-fork/tree/VIwork), based on [VIctory](https://github.com/Marvisak/VIctory) and community contributions. This is a work in progress, with incomplete rendering and no guarantee of stability. The source and Windows executable are provided without game files or assets.

## Status and testing

Tested on Windows with an NVIDIA RTX 4090 using the Series X profile. Numbers below are individual samples from different gameplay scenes, not a benchmark average; scene load and shader readiness affect results.

| Test | Observed result |
| --- | --- |
| Lighter gameplay scene | Peak 20.0 FPS at about 2,627 draw calls; five preceding samples ranged from 18.3 to 19.5 FPS |
| Another gameplay run | 17.6–18.8 FPS across five samples at about 2,700 draws; the peak sample also reported 462 dropped draws from missing host pipelines |
| Heavier gameplay scene | 13.7–15.0 FPS at roughly 5,500–5,700 draws |
| Separate V1 performance check | 12.8 FPS at approximately 5,508 draws |
| High-load spike | 1.8 FPS at about 10,262 draws; 567 ms per frame |

The gameplay launch used `VICTORY_AMBIENT=0.6`, `VICTORY_CARD_BUFFERS_MB=4096`, `VICTORY_CARD_WINDOWS_MB=4096` and `VICTORY_WIDE_MERGE=1`, with a populated shader cache. Samples are roughly one-second averages logged about every five seconds. The examples were not controlled camera-matched comparisons. Missing pipelines can reduce draw work, so FPS alone does not establish visual correctness. A separate run ended with `DXGI_ERROR_DEVICE_HUNG`; the exact GPU command responsible was not identified.

## Changes in V1

Compared with the Trexies `VIwork` baseline:

- Retains the Belobug ambient lighting pass.
- Adds incremental descriptor-heap scanning and texture/view handling changes.
- Adds an own-view copy-back path, read/write locks and batching for memory and view tracking, plus parallel wide-merge work.
- Ports asynchronous submit and fill-ahead paths, with configurable options, and adds submission/relay timing diagnostics.
- Adds guarded in-place and plain-copy paths and fixes for nested wide-run re-entrancy and texture detiling.
- Includes texture and descriptor stability fixes and GPU-side resource handling optimizations.

See the source and its comments for individual switches and attribution. Features and behavior can depend on environment settings; the results above are observations from the tested configuration.

## Build and run

Requires Windows x64, Rust as pinned in `rust-toolchain.toml`, a compatible Direct3D 12 GPU, and the user-supplied title files. No title files or cached title data are distributed here.

Build from source, or download the standalone `victory.exe` from Releases. No PowerShell launcher is required. In Command Prompt, run from the directory containing `victory.exe`:

```cmd
set "VICTORY_AMBIENT=0.6"
set "VICTORY_CARD_BUFFERS_MB=4096"
set "VICTORY_CARD_WINDOWS_MB=4096"
set "VICTORY_WIDE_MERGE=1"
victory.exe run "<path-to-your-title-executable>" --execute --console series-x
```

To build it yourself, run `cargo build --release -p victory`; the executable is written to `target\release\victory.exe`.

## Credits and license

Based on [Trexies/VIwork](https://github.com/Trexies/victory-fork/tree/VIwork), [VIctory](https://github.com/Marvisak/VIctory), and the community work credited in the upstream README and source. GPL-3.0-or-later; see [LICENSE](LICENSE).

---

## Upstream README

The following is the inherited upstream documentation. Its hardware, status and FPS observations describe upstream configurations, not the ShakiraRED V1 test above.

# VIctory fork (work in progress)

A GPL-3.0 fork of [VIctory](https://github.com/Marvisak/VIctory), a compatibility layer that runs an
Xbox Series title natively on Windows from files you supply. **No game files or assets are included
or required.**

This tree starts from [`fxceless/victory-fork`](https://github.com/fxceless/victory-fork) at
`310c6eb`, merges
[`ZSlayerHQ/victory-world-runtime`](https://github.com/ZSlayerHQ/victory-world-runtime) branch `next`
(`99f8834` — *"both public forks merged, with the walls fix confirmed on an RTX 5090"*), and adds
this fork's changes on top.

## Tested on this machine

| | |
|---|---|
| **CPU** | AMD Ryzen 7 9800X3D — 8 cores / 16 threads, 4.7 GHz |
| **GPU** | NVIDIA GeForce RTX 5080 — 16 GB (16,303 MiB), driver 621.24 |
| **RAM** | 32 GB (2 × 16 GB) Micron DDR5-6400 |
| **Board** | Gigabyte X870E AORUS ELITE WIFI7 |
| **Storage** | Samsung 980 PRO 1 TB + WD_BLACK SN7100 2 TB (NVMe) |
| **OS** | Windows 11 Home, 10.0.26300 |
| **Display** | 2560 × 1440 |

## What this fork adds

- **`VICTORY_GROUP_SYNC` defaults to `2`** (every barrier waits). `auto`/`3` opts into the
  automatic mode from #608; its default caused constant flicker on this GPU.
- **`victory translate <shaders.rpf> --warm`** — translate the whole shader pack into the cache a
  run reads. A second pass finds all 26k containers cached; a run then starts warm.
  (`victory warm` from the merged branch is also present and makes compute pipelines too.)
- **PRs #662** (view-heap limit via `D3D12_OPTIONS19`), **#654** (skipped pipelines not built
  early), **#608** (narrow-wave divergent barriers + host wave in the shader-cache key), **#615**
  (`ds_bpermute_b32` cross-half scratch).
- **Pipeline workers** — `VICTORY_PIPELINE_WORKERS` (default: half the logical CPUs, 2–8) and
  `VICTORY_PIPELINE_PRIORITY`.
- **Merged community PRs** — #5 (the host D3D12 device is made before the title runs, so a slow
  driver cache cannot trip the title's 30 s watchdog) and #6 (the texture catch-up allowance resets
  per scene once the backlog settles), both by Proudnoob4.
- **`VICTORY_FRAMETIME_DUMP`** — `=1` writes `<cache>\frametime.csv` (default
  `%LOCALAPPDATA%\victory\cache\frametime.csv`); any other value is a path. One CSV row per ~1 s
  summary interval, with the frame time and the buckets the runtime already measures (`gpu_ms`,
  `cpu_prepare_ms`, `copies_ms`, `textures_ms`, `copyback_ms`, `heap_ms`, `card_*`, …), plus the
  worst/average **single** frame. Release-buildable. Implementation:
  `crates/victory-gpu/src/frametime.rs`.

## Performance on this machine

Settled world on the merged build, 1440p, Series X profile: **~4.1–4.4 fps / 227–260 ms**, with
4,400–4,800 draws/frame. The frame budget measured by the dump:

| bucket | ms/frame |
|---|---:|
| `gpu_ms` (submitting thread waits for the card) | ~96–98 |
| `cpu_prepare_ms` | ~64–69 |
| `copies_ms` | ~37–41 |
| `copyback_ms` | ~27–28 |
| `heap_ms` | ~13–62 (median ~11–18) |
| `textures_ms` | ~3–4 |

The **1–2 s drops** line up with `heap_ms` (the by-index resource-table pass: walk buffer views,
resolve host copies, merge ranges, queue refreshes, pick textures) and `textures_ms`, while the GPU
is idle — i.e. asset binding/streaming, not GPU and not the file-read path. `copies_ms` +
`copyback_ms` are ~27% of a frame. Separately, 2.5–4.9 s gaps between presents show no heap/GPU
cost and are not attributed yet. Full sample writeup: `crates/victory-gpu/src/frametime.rs` and the
dump itself.

**Reading `gpu_ms`/`gpu_pct`:** with `VICTORY_RELAY` on they are the *submitting thread's wait for
the card*, not how busy the card is. The real figure is the log's
`the card's clock per frame: nothing of ours queued N ms (the card about P% busy by this count)`
line; dumps from issue #4 make the same point. The CSV does not carry that P% yet.

## Capture a frametime dump

### In short (person)
1. Merge/pull this repo and **build the exe**:
   ```sh
   cargo build --release -p victory
   ```
   Pulling the source is not a build. Use the exe you just built.
2. Run the title with the dump on:
   ```bat
   run-frametime.cmd "D:\game\game.exe"
   ```
   or, in the same `cmd` window:
   ```bat
   set VICTORY_FRAMETIME_DUMP=1
   target\release\victory.exe run "D:\game\game.exe" --execute
   ```
3. Play into the world for 2–3 minutes, then send
   `%LOCALAPPDATA%\victory\cache\frametime.csv` (see
   [Where the dump goes](#where-the-dump-goes)). Add it under `dumps/` or post it.

### Copy this to an agent
> Merge or clone `https://github.com/skornuta/victory-fork` (`main`), run
> `cargo build --release -p victory`, then run the title with `VICTORY_FRAMETIME_DUMP=1`
> (or `run-frametime.cmd "<title.exe>"`). Confirm the log prints `frametime dump path=...`;
> the CSV is at `%LOCALAPPDATA%\victory\cache\frametime.csv` (`victory config` prints
> `cache_dir`). Let it reach the world and run 2–3 minutes, then return the CSV. Do not modify
> or copy the game files.

### If no file appears
- `git pull` without `cargo build --release -p victory` → still the old exe, no dump.
- Running a `victory.exe` from another folder or profile → run `target\release\victory.exe`.
- No `frametime dump path=...` in the log → that build has no dump, or it never presented a frame.
- Pre-`646f5cf` builds wrote `=1` to `%TEMP%\victory-frametime.csv`.
- PowerShell does not expand `%LOCALAPPDATA%`; use `$env:LOCALAPPDATA` or the `.cmd`.
- The file appears on the first presented frame (~1 s), not at launch.

Speed and memory:
- Waits for memory values, write-watch queries and page queries no longer walk page tables per call.
- Large title buffers get a copy in card memory, with shader writes found by hashing on the card.
- Submissions can be sent ahead of the wait for the card (`VICTORY_LATE_WAIT`), with a relay thread that lands results as the card finishes (`VICTORY_RELAY`): in a first five-minute run the card's idle time behind the submitting thread fell from about 136 to 30 ms per frame.
- Independent host-copy refreshes run in parallel by default on systems with at least four logical CPUs (`VICTORY_WIDE_REFRESH=0` disables this; `=1` forces it). This targets the serial memory reads that previously cost about 45–50 ms per frame on the test run.
- Title memory can be opened as a graphics heap so that buffers are used in place instead of copied (`VICTORY_INPLACE`, first stage only; a probe confirmed the approach on the test GPU).
- The bindless texture queue no longer uploads stale views; the view table no longer grows without bound; the pipeline library is mapped instead of read into memory.
- An intermittent device loss was traced to the pipeline library file being replaced while mapped; fixed.

## Community dumps

Most of the newer speed changes sit behind environment switches that are off
by default (`VICTORY_LATE_WAIT`, `VICTORY_RELAY`, `VICTORY_WIDE_BACK`,
`VICTORY_CARD_BUFFERS_MIN_MB`, `VICTORY_INPLACE`, `VICTORY_BVH_BUILDER`, ...).
Independent host-copy refreshes are the exception: they use parallel workers
automatically on systems with at least four logical CPUs. Each switch is
described in a comment where it is read.

## Build and run

```sh
cargo build --release -p victory

# warm the shader cache once (same build and shader switches as the run)
victory translate "D:\game\shaders.rpf" --warm

victory run "D:\game\game.exe" --execute
```

Or use the checked-in helper (sets `VICTORY_FRAMETIME_DUMP=1`; build first):

```bat
run-frametime.cmd "D:\game\game.exe"
```

Environment used for the measurements:

```bat
set VICTORY_PIPELINE_WORKERS=8
set VICTORY_PIPELINE_PRIORITY=normal
set VICTORY_SHADER_LOOP_LIMIT=1024
set VICTORY_GROUP_SYNC=2
set VICTORY_RELAY=2
set VICTORY_LATE_WAIT=3
set VICTORY_WIDE_BACK=1
set VICTORY_CARD_BUFFERS_MIN_MB=1
set VICTORY_CARD_BUFFERS_MB=1024
set VICTORY_CARD_REHASH_RUNS=1
set VICTORY_JOIN_COPIES=1
set VICTORY_FRAMETIME_DUMP=1
```

| Variable | Effect |
|----------|--------|
| `VICTORY_LOG=<target>=debug,...` | debug and trace output |
| `VICTORY_STACKS=20,60` | every thread's stack at those seconds |
| `VICTORY_WATCH=<rva>,...` | logs a title u32 when it changes |
| `VICTORY_KEYS=30:Enter,35:X` | presses keys at those seconds, focus or not |
| `VICTORY_CRASH_SAVE=<rva>:<len>,...` | saves title memory to the cache folder on a crash |
| `VICTORY_CAPTURE=<dir>` | saves presented frames (powers of two, or `VICTORY_CAPTURE_FRAMES=<list>`) |
| `VICTORY_DUMP_TARGETS=1` | with the two above, saves every render target of those frames |
| `VICTORY_D3D12_DEBUG=1` | host debug layer |
| `VICTORY_D3D12_GBV=1` | with the above, GPU-based validation (bad descriptor use reported, not faulted; slow) |
| `VICTORY_D3D12_DRED=1` | on a GPU failure, the allocation hit and the commands in flight |
| `VICTORY_PIPELINE_WAIT_MS=24` | reduce the per-frame wait for background pipeline creation from the default of 500 ms. A draw can be omitted until its pipeline is ready |
| `VICTORY_SKIP_PIPELINES=<hash>,...` | omit draws and dispatches, including indirect ones, whose pipeline contains a listed 16-digit shader hash |
| `VICTORY_GPU_TIMING=1` | with the above, each list of a submission run and timed on its own; slow lists logged with their slowest commands |
| `VICTORY_CARD_HASH_WIDE=0` | the passes that find what shaders wrote to copies in the card's memory hash with one thread per 64 KiB, as before |
| `VICTORY_D3D12_WARP=1` | host device on the software adapter (slow; cannot reset the driver) |
| `VICTORY_SHADER_LOOP_LIMIT=<n>` | translated shader loops stop after `n` iterations per lane instead of 2^20 (own shader cache entries); if a lower `n` makes world frames much faster, loops hitting the limit are what makes them slow |
| `VICTORY_AUDIO_CAPTURE=<file.wav>` | records what the title plays (our process only) |
| `VICTORY_XMA_DUMP=<dir>` | saves the first 32 XMA voices' context and input |

| `VICTORY_FRAMETIME_DUMP` | File |
|---|---|
| unset or empty | no dump |
| `1` | `%LOCALAPPDATA%\victory\cache\frametime.csv` — e.g. `C:\Users\<you>\AppData\Local\victory\cache\frametime.csv` |
| any other value | exactly that path (its folder must already exist) |

Builds before `646f5cf` wrote `=1` to `%TEMP%\victory-frametime.csv` instead; if the cache path is
empty, check there. An explicit absolute path works on every build.

GPL-3.0-or-later. See [LICENSE](LICENSE).

