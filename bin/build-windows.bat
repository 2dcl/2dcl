rd /s /q build
cargo build --release 
mkdir build\windows\assets
xcopy target\release build\windows /e
xcopy 2dcl\assets build\windows\assets /e
7z -ttar a dummy build\windows\* -so | 7z -si -tgzip a build\windows\2dcl-windows-0.1.0.tgz