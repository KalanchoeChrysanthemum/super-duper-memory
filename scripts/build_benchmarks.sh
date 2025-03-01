#!/bin/bash
# crazy bash build script :0

# make if not exist and clean build dirs
mkdir -p bin
rm -rf bin/*

mkdir -p build-tmp
rm -rf build-tmp/*
# cargo build, even with release flag, puts a bunch of random shit in the build dir
# I just want the executable, so i just put them here and mv them over
# think I can just use cargo clean on that build dir later too


build_and_cp_exe(){
    CUR="$1"
    cargo build --release --target-dir build-tmp --bin "${CUR}"
    mv "build-tmp/release/${CUR}" bin
    cargo clean --target-dir build-tmp

}

for file in src/bin/benchmarks/*; do

    filename=$(basename "$file")
    
    exe_name="${filename%.*}"

    echo "Found executable: $exe_name"
    build_and_cp_exe $exe_name
done



rm -rf build-tmp

