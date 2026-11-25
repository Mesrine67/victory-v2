# victory-fork — Neptune (VIctory Edit) build

**Public try-build while the community mega fork is being finished.** The same
lines are merged here, so you can run and measure today. See
[`PREVIEW.md`](PREVIEW.md) for what is already in, what is still landing, and
what to report.

This fork now carries **Neptune**, traffic & zm0kie's build of the VIctory runtime,
on top of the myokex / fxceless / JoshuaO22 line. It replaces the earlier
ShakiraRED V1 tree, because Neptune already includes the two things that tree was
missing:

- **the soldsawn translator** (`soldsawn/victory-nvidia-8gb`): 64-lane console wave
  programs are translated into a paired form for 32-lane NVIDIA cards. The refusal
  classes the V1 tree still dropped — whole-wave sections, first-lane reads,
  `v_permlanex16_b32`, lane masks read as numbers — are handled by
  `crates/victory-shader/src/emit/idiom.rs`, and descriptors read as data by
  `emit/descriptor_data.rs`. The author's note: *"no pipeline is refused any more."*
- **levels on demand** (`VICTORY_LEVELS_ON_DEMAND=1`): a texture takes card memory
  only for the mip levels the game actually holds.

It also keeps V1's ambient pass (Belobug / ShakiraRED-dt), our
`VICTORY_FRAMETIME_DUMP` CSV, and adds the descriptor-heap byte-copy fix, the
pipeline wait cap (`VICTORY_PIPELINE_WAIT_MS=2500`), window uploads, D3D12
background off, Discord Rich Presence (opt-out with `VICTORY_DISCORD=0`) and a
mouse/keyboard pad.

No game files or assets are included or required. Shaders are translated on your
machine from your own copy.

## Tested on this machine

| | |
|---|---|
| **CPU** | AMD Ryzen 7 9800X3D — 8 cores / 16 threads |
| **GPU** | NVIDIA GeForce RTX 5080 — 16 GB, driver 621.24 |
| **RAM** | 32 GB DDR5-6400 |
| **OS** | Windows 11 Home, 10.0.26300 |
| **Display** | 2560 × 1440, Series X profile |

## Performance

Neptune's own numbers (RTX 5070 Ti, i7-14700, 64 GB, Series S 720p → 1080p):

| scene | fps |
|---|---:|
| open world, 6,500–9,000 draws | 9–11 (less while streaming) |
| inside the house | ~12 |
| first room, Series S + FSR | 16–17 |
| Series X (4K), first room | ~14 |

On this 5080 / 9800X3D, with the warm shader and pipeline caches, `-SeriesX`:

| | |
|---|---:|
| fps | **TBD** |
| ms/frame | **TBD** |
| draws | **TBD** |

For comparison, the earlier ShakiraRED V1 tree measured 17.8 fps median / 56 ms at
~6,679 draws, with ~350–450 draws per frame dropped as refused compute shaders.
Neptune's translator is what removes those drops.

## Build

The toolchain is pinned (`rust-toolchain.toml`, 1.98.1) and installed by rustup on
the first build.

```sh
cargo build --release -p victory
```

Translate the shaders once (about 5–10 minutes; the run reuses the cache):

```bat
set VICTORY_GAME_DIR=<game directory>
target\release\victory.exe warm --jobs 16
```

Then run:

```bat
run-neptune.cmd -Warm        :: first time: warm, then start
run-neptune.cmd              :: every time after
```

`run-neptune.cmd` calls the checked-in `play.ps1`, which sets the tested switches
and picks Series S + FSR when a PC FidelityFX DLL is at hand, otherwise Series X.
You can also call it directly:

```powershell
.\play.ps1 -Game 'D:\path\to\game_scarlett_master.exe' -Warm
.\play.ps1 -Game 'D:\path\to\game_scarlett_master.exe' -SeriesX -FrametimeDump
```

### The tested switch set

`play.ps1` carries the settled set; the important ones:

```bat
set VICTORY_LEVELS_ON_DEMAND=1        :: mip levels on demand
set VICTORY_CARD_WINDOWS_MB=4096
set VICTORY_CARD_BUFFERS_MB=1536
set VICTORY_WINDOW_KB=1024
set VICTORY_WIDE_MERGE=1
set VICTORY_WIDE_THREADS=12
set VICTORY_RELAY_OWN_QUEUE=1
set VICTORY_RELAY=2
set VICTORY_TEXTURE_NO_WAIT=1
set VICTORY_TEXTURE_STAGE_MB=512
set VICTORY_JOIN_COPIES=1
set VICTORY_COPY_CHANGED=1
set VICTORY_QUIET_BACK=1
set VICTORY_INDIRECT_RAW=2
set VICTORY_DESC_DATA=1               :: shaders that read a descriptor as data
set VICTORY_DROP_DXIL=1
set VICTORY_PIPELINE_WAIT_MS=2500     :: no 12 s shader stalls
set VICTORY_VRAM_IDLE_MS=15000,2500,1000
```

## Frametime dump

`-FrametimeDump` (or `VICTORY_FRAMETIME_DUMP=1`) writes one row a second to
`%LOCALAPPDATA%\victory\cache\frametime.csv`, with a `#` header naming the
hardware, driver and switches. It is the same tool this fork has always carried;
see [`dumps/`](dumps/README.md) for the community captures and the naming
convention.

## Known issues

- Objects and clothes can turn black for a moment while the game swaps textures.
- Parts of the world are missing behind some doors and on some roofs.
- Facial animation does not play.
- Frame time jumps while the game streams.
- Light indoors is a flat stand-in (`VICTORY_AMBIENT`); ceilings can stay black
  and shiny surfaces sparkle.
- Planar reflections are the game's own `mirror_*` pass and are being worked on
  separately.

## Credits

This is a merge, not new work on the runtime's core. It stands on:

- **Neptune (traffic & zm0kie)** — the merge base, switch set, Discord presence,
  window uploads, memory lines.
- **soldsawn/victory-nvidia-8gb** — the paired-wave translator.
- **myokex/victory-amd**, **fxceless/victory-fork** — the base, levels on demand,
  descriptor-heap sync.
- **Belobug**, **reddxd-byte/ShakiraRED-dt** — the ambient light pass.
- **ZSlayerHQ/victory-world-runtime** — writes seen across views, kernel reads,
  dispatch in parts.
- **JoshuuDev/victory-amd** — remapping lock by range, blend and descriptor fixes.
- **Trexies** — folded small buffers.
- **Marvisak/VIctory** — the original runtime.
- **skornuta/victory-fork** — the frametime dump and this fork's packaging.
- The FidelityFX redirect patch received for DexNom Runtime.

GPL-3.0-or-later, following upstream. See [LICENSE](LICENSE).
