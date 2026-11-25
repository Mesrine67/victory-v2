# victory-fork — VIctory, RTX 5080 build

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
| `gpu_ms` | ~96–98 |
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

## Community dumps

`dumps/` holds frametime CSVs captured on real machines, with a short index in
[`dumps/README.md`](dumps/README.md). They are the raw input for the bucket comparison above; add
your own with `VICTORY_FRAMETIME_DUMP=1`.

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

### Where the dump goes

| `VICTORY_FRAMETIME_DUMP` | File |
|---|---|
| unset or empty | no dump |
| `1` | `%LOCALAPPDATA%\victory\cache\frametime.csv` — e.g. `C:\Users\<you>\AppData\Local\victory\cache\frametime.csv` |
| any other value | exactly that path (its folder must already exist) |

Builds before `646f5cf` wrote `=1` to `%TEMP%\victory-frametime.csv` instead; if the cache path is
empty, check there. An explicit absolute path works on every build.

`=1` follows Victory's `cache_dir`, so `VICTORY_CACHE_DIR` or a `victory.toml` moves it; `victory config`
prints the resolved `cache_dir`. The file is created on the **first presented frame** (~1 s after the
title starts presenting) and is **truncated at the start of every run**, so each run gets a fresh file.
The log also prints `frametime dump path=...` when it opens.

In PowerShell, use `$env:VICTORY_FRAMETIME_DUMP = '1'` — `%TEMP%`/`%LOCALAPPDATA%` are not expanded
there.

Every dump starts with `#` lines naming the machine (CPU, GPU + VRAM, RAM, OS, board, display) and
the `VICTORY_*` switches, then the CSV header and rows. `pandas.read_csv(path, comment='#')` reads
it directly.

## Known issues

- **Dropped draws:** ~350–550 draws/frame are skipped for lack of a host pipeline (mesh pipelines
  refused with `0x80070057` among them). This is the main correctness gap seen here.
- **Long present gaps:** 2.5–4.9 s gaps with no heap/GPU cost; the current buckets can't attribute
  them (loading? pipeline? page-in?).
- **Shadowed areas are black** and ray-traced light does not run — inherited from the merged tree.
- Memory in play is high (~14–15 GB working set on this machine).

## Credits and licence

VIctory is by [Marvisak](https://github.com/Marvisak/VIctory) and its contributors. The community
fixes merged here are the work of the authors of
[`ZSlayerHQ/victory-world-runtime`](https://github.com/ZSlayerHQ/victory-world-runtime),
[`fxceless/victory-fork`](https://github.com/fxceless/victory-fork),
[`myokex/victory-amd`](https://github.com/myokex/victory-amd),
[`Trexies/victory-fork`](https://github.com/Trexies/victory-fork) and
[`WillyboPeep/victory-all-forks`](https://github.com/WillyboPeep/victory-all-forks); see their
histories and READMEs. [XCL](https://github.com/ivall/xcl) was used as a reference for how the same
title behaves under another layer. GPL-3.0, following upstream.
