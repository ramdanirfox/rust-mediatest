rustc version: rustc 1.67.1 (d5a82bbd2 2023-02-07)

For openmpt compat, i have to put libopenmpt.lib inside e
and rename it to openmpt.lib
For openmpt compat, i have to put libopenmpt.lib inside %USERPROFILE%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib
and rename it to openmpt.lib

UPDATES:
no need to copy libopenmpt.lib manually, its handled within build script now

Linux Only to minify binary
strip file-kita