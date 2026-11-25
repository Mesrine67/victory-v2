# Community frametime dumps

To capture one, see [Capture a frametime dump](../README.md#capture-a-frametime-dump).

CSV captures from `VICTORY_FRAMETIME_DUMP=1` on real machines, for comparing the frame buckets and
the 1–2 s drops across hardware. Each file is one run. When the `#` header is present it names the
CPU, GPU + VRAM, driver, RAM, OS, board, display and the `VICTORY_*` switches, so the run is reproducible.

| File | Build | Captured | Machine | Run |
|---|---|---|---|---|
| [`frametime_skornuta_cd86c4d_2026-11-25_world.csv`](frametime_skornuta_cd86c4d_2026-11-25_world.csv) | `cd86c4d` | 2026-11-25 00:04 | not recorded — capture predates the `#` header; presumed skornuta's RTX 5080 / 9800X3D | reaches the world; max 8,428 draws/frame, last 3.4 fps, `heap_ms` peaks ~1.4 s |
| [`frametime_skornuta_5080_9800x3d_931bee3_2026-11-25_fold-world.csv`](frametime_skornuta_5080_9800x3d_931bee3_2026-11-25_fold-world.csv) | `931bee3` | 2026-11-25 06:37 | RTX 5080 / Ryzen 7 9800X3D / 32 GB DDR5-6400 / Win11 26300 / driver 621.24 | world with fold on, ~12 min, 628 intervals; settled 4.7 fps / 226 ms, 5,636 draws |

## Adding a dump

1. Run with `VICTORY_FRAMETIME_DUMP=1` (writes `%LOCALAPPDATA%\victory\cache\frametime.csv`) or give
   a path.
2. Name it `frametime_<owner>_<gpu>_<build>_<YYYY-MM-DD>_<scene>.csv` when the machine is known, or
   `frametime_<owner>_<build>_<YYYY-MM-DD>_<scene>.csv` when it is not.
3. Add a row above with the build, date, machine and what the run covers (menus, world, a specific
   area, how long).

The files contain no game assets: the `#` block is hardware and switches only.
