@echo off
rem Runs the merged Neptune build with the tested switch set.
rem   run-neptune.cmd                 uses VICTORY_GAME_DIR, else ..\GTA6
rem   run-neptune.cmd -Warm           translate the shaders first (first time)
rem   run-neptune.cmd -SeriesX        the 4K path instead of Series S + FSR
rem   run-neptune.cmd -FrametimeDump  also write <cache>\frametime.csv
setlocal
if "%VICTORY_GAME_DIR%"=="" (
    if exist "%~dp0..\GTA6\game_scarlett_master.exe" set "VICTORY_GAME_DIR=%~dp0..\GTA6"
)
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0play.ps1" %*
endlocal
