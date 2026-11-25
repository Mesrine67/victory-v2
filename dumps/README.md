# Community frametime dumps

These files were captured on earlier lines and are kept for comparison. The merged
Neptune tree in this fork carries the dump tool again (`VICTORY_FRAMETIME_DUMP=1`,
or `run-neptune.cmd -FrametimeDump`), so new samples can be added the same way.

CSV captures from `VICTORY_FRAMETIME_DUMP=1` on real machines, for comparing the frame buckets and
the 1–2 s drops across hardware. Each file is one run. When the `#` header is present it names the
CPU, GPU + VRAM, driver, RAM, OS, board, display and the `VICTORY_*` switches, so the run is reproducible.

| File | Build | Captured | Machine | Run |
|---|---|---|---|---|
| [`frametime_rtx5080_9800x3d_32gb_931bee3_2026-11-25_fold-world.csv`](frametime_rtx5080_9800x3d_32gb_931bee3_2026-11-25_fold-world.csv) | `931bee3` | 2026-11-25 06:37 | RTX 5080 / Ryzen 7 9800X3D / 32 GB DDR5-6400 / Win11 26300 / driver 621.24 | world with fold on, ~12 min, 628 intervals; settled 4.7 fps / 226 ms, 5,636 draws |
| [`frametime_rx9070xt_r7-7700_32gb_2026-11-25_world.csv`](frametime_rx9070xt_r7-7700_32gb_2026-11-25_world.csv) | not in header | 2026-11-25 | AMD RX 9070 XT / Ryzen 7 7700 / 32 GB / Win11 Home 25H2 26200 / driver 32.0.32029.2010 / 5120x1440 | world, 588 intervals, median 3,646 draws, 4.1 fps; `VICTORY_RT_RESOLVE=0`; paths redacted |
| [`frametime_rtx4080super_i9-14900k_32gb_2026-11-25_render-audit.csv`](frametime_rtx4080super_i9-14900k_32gb_2026-11-25_render-audit.csv) | not in header | 2026-11-25 | RTX 4080 SUPER / i9-14900K / 32 GB / Win11 Pro 26H2 26300 / driver 621.24 / 2560x1440 | shader render audit (`VICTORY_SHADER_OPT=0`, captures), 225 intervals, median 4,855 draws, 2.7 fps; paths redacted |
| [`frametime_rx6900xt_r5-3600_32gb_9292987_2026-11-25_world.csv`](frametime_rx6900xt_r5-3600_32gb_9292987_2026-11-25_world.csv) | `9292987` | 2026-11-25 09:16 | AMD RX 6900 XT / Ryzen 5 3600 / 32 GB / Win10 22H2 19045 / driver 32.0.21045.29003 | world scan window, 145 intervals (~175 s); from issue #2 |
| [`frametime_rtx3060_i9-14900k_32gb_2026-11-25_first-scene.csv`](frametime_rtx3060_i9-14900k_32gb_2026-11-25_first-scene.csv) | fxceless `9376fd6` + PRs | 2026-11-25 | RTX 3060 8 GB / i9-14900K / 32 GB / Win11 25H2 26200 / driver 610.88 | first gameplay scene, 199 intervals; from issue #4 |

## Adding a dump

1. Run with `VICTORY_FRAMETIME_DUMP=1` (writes `%LOCALAPPDATA%\victory\cache\frametime.csv`) or give
   a path.
2. Name it `frametime_<gpu>_<cpu>_<ram>gb_<build>_<YYYY-MM-DD>_<scene>.csv` — hardware, not the
   owner; credit the source in the table. Absolute paths in `# switches` are redacted to `<path>`.
3. Add a row above with the build, date, machine and what the run covers (menus, world, a specific
   area, how long).

The files contain no game assets: the `#` block is hardware and switches only.
