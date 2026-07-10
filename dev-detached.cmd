@echo off
rem Launch the Tauri dev environment OUTSIDE any MSIX/AppContainer context.
rem (When launched from a sandboxed shell, %APPDATA% writes are virtualized and
rem the asset protocol 403s — see README dev notes.)
cd /d A:\Claude\pg-model-viewer
set WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223
npm run tauri dev > A:\Claude\pg-model-viewer\dev-detached.log 2>&1
