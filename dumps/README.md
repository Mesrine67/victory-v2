# Community frametime dumps

To capture one, see [Capture a frametime dump](../README.md#capture-a-frametime-dump).

CSV captures from `VICTORY_FRAMETIME_DUMP=1` on real machines, for comparing the frame buckets and
the 1–2 s drops across hardware. Each file is one run. When the `#` header is present it names the
CPU, GPU + VRAM, driver, RAM, OS, board, display and the `VICTORY_*` switches, so the run is reproducible.

| File | Build | Captured | Machine | Run |
|---|---|---|---|---|
| [`frametime_skornuta_5080_9800x3d_931bee3_2026-11-25_fold-world.csv`](frametime_skornuta_5080_9800x3d_931bee3_2026-11-25_fold-world.csv) | `931bee3` | 2026-11-25 06:37 | RTX 5080 / Ryzen 7 9800X3D / 32 GB DDR5-6400 / Win11 26300 / driver 621.24 | world with fold on, ~12 min, 628 intervals; settled 4.7 fps / 226 ms, 5,636 draws |
| [`frametime_blackbu7let_rx6900xt_r5-3600_9292987_2026-11-25_world.csv`](frametime_blackbu7let_rx6900xt_r5-3600_9292987_2026-11-25_world.csv) | `9292987` | 2026-11-25 09:16 | AMD RX 6900 XT / Ryzen 5 3600 / 32 GB / Win10 22H2 19045 / driver 32.0.21045.29003 | world scan window, 145 intervals (~175 s); from issue #2 |
| [`frametime_woreksami_rtx3060_i9-14900k_2026-11-25_first-scene.csv`](frametime_woreksami_rtx3060_i9-14900k_2026-11-25_first-scene.csv) | fxceless `9376fd6` + PRs | 2026-11-25 | RTX 3060 8 GB / i9-14900K / 32 GB / Win11 25H2 26200 / driver 610.88 | first gameplay scene, 199 intervals; from issue #4 |

## Adding a dump

1. Run with `VICTORY_FRAMETIME_DUMP=1` (writes `%LOCALAPPDATA%\victory\cache\frametime.csv`) or give
   a path.
2. Name it `frametime_<owner>_<gpu>_<build>_<YYYY-MM-DD>_<scene>.csv` when the machine is known, or
   `frametime_<owner>_<build>_<YYYY-MM-DD>_<scene>.csv` when it is not.
3. Add a row above with the build, date, machine and what the run covers (menus, world, a specific
   area, how long).

The files contain no game assets: the `#` block is hardware and switches only.
