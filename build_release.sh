COL='\033[0;33m'
printf "${COL}\n ========== Building ========== \n\n"
cargo build --release
printf "${COL}\n ========== Building for Windows ========== \n\n"
cargo build --release --target x86_64-pc-windows-gnu
printf "${COL}\n ========== Building for WASM ========== \n\n"
./build_wasm.sh minesweeper --release