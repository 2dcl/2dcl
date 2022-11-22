rd /s /q build
cargo build --release 
mkdir build/windows
copy target/release/2dcl build/windows
xcopy 2dcl/assets build/windows /e
cd build/windows