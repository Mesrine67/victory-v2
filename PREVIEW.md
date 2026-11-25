# Preview: the mega-fork merge, runnable today

The community mega fork (`byartesao/victory-giga-fork`) is still being assembled.
This branch is the closest runnable thing to it: the same lines, merged and
measured, with the pieces that are ready now.

If you are waiting on the mega fork, **test this build and post your numbers** —
that is what it is for. When the mega fork lands, we rebase on it and diff.

## What is already in this preview

| Line | What it brings |
|---|---|
| **traffic / zm0kie — Neptune** | merge base, tested switch set, window uploads, Discord presence, memory lines |
| **soldsawn/victory-nvidia-8gb** | paired-wave translator: 64-lane console programs on 32-lane NVIDIA cards; the whole-wave / first-lane / lane-mask refusal classes |
| **myokex/victory-amd** | levels on demand, the AMD-line base |
| **fxceless/victory-fork** | descriptor-heap sync, the byte-copy fix |
| **ZSlayerHQ/victory-world-runtime** | writes seen across views, kernel reads, dispatch in parts |
| **JoshuuDev/victory-amd** | remapping lock by range, blend and descriptor fixes |
| **Belobug / ShakiraRED-dt** | the ambient light pass |
| **Trexies** | folded small buffers |
| **skornuta/victory-fork** | the frametime dump, packaging, the community dumps |

## Still landing in the mega fork

- firme's final merge pass and switch set.
- myokex's remaining `victory-amd` work (the parts not in this tree yet).
- traffic/reddit assets and fixes.
- Planar reflections — the stall mirror and the `mirror_*` pass (im gone).
- DirectStorage / GDeflate asset streaming, only-load-what-is-needed
  (Ethan, frazer; the current finding is that the codecs are fine and the
  end-to-end pipeline is the bottleneck).
- SAM's memory work (asset capture, VRAM/RAM pressure).

## What to report

1. GPU, CPU, RAM, driver, resolution, Series S or X.
2. `run-neptune.cmd -FrametimeDump`, then the CSV from
   `%LOCALAPPDATA%\victory\cache\frametime.csv`.
3. fps, draws, and the count of dropped draws from the log.
4. Anything missing or black: a screenshot and where you were.

Naming for the CSV: `frametime_<gpu>_<cpu>_<ram>gb_<build>_<date>_<scene>.csv`
(hardware, not your name; absolute paths redacted). See [`dumps/`](../dumps/README.md).
