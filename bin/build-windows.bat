rd /s /q build
cargo build --release 
mkdir build\windows\assets
copy target\release\2dcl.exe build\windows
xcopy 2dcl\assets build\windows\assets /e
7z -tzip -sdel a build\windows\2dcl-windows-0.1.0.zip .\build\windows\*