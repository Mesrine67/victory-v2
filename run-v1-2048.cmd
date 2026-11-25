@echo off
rem Experimental: give 4 GB of card-side buffers back to textures.
rem Measured during a loading scene only; compare at your own draw count.
setlocal
if "%~1"=="" (
    echo usage: run-v1-2048.cmd "<path to the title executable>"
    exit /b 1
)
cd /d "%~dp0"
set VICTORY_AMBIENT=0.6
set VICTORY_CARD_BUFFERS_MB=2048
set VICTORY_CARD_WINDOWS_MB=2048
set VICTORY_WIDE_MERGE=1
set VICTORY_PIPELINE_WORKERS=8
set VICTORY_GROUP_SYNC=2
set VICTORY_SHADER_LOOP_LIMIT=1024
set VICTORY_RELAY=2
set VICTORY_LATE_WAIT=3
set VICTORY_WIDE_BACK=1
set VICTORY_CARD_BUFFERS_MIN_MB=1
set VICTORY_CARD_REHASH_RUNS=1
set VICTORY_JOIN_COPIES=1
set VICTORY_LOG=info
"target\release\victory.exe" run "%~1" --execute --console series-x
endlocal
