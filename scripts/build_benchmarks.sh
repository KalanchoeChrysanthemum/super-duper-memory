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

# 22 seconds with 5 bins pre optimization
# 14 seconds with 5 bins post optimization
build_and_cp_exe(){
    CUR="$1"
    # this target build dir has to be unique for this to run in parallel
    TMP_TARGET="${CUR}-build"
    mkdir -p "build-tmp/${TMP_TARGET}"

    # this does *not* compile them all mains in parallel, but compiles 
    # each main itself (mostly its dependencies) in parallel, so it
    # should be proportional to the number of mains we have 
    # NUM_WORKERS=$(ls src/bin/benchmarks | wc -l) 
    # NUM_WORKERS=$(nproc)

    echo "Compiling ${CUR}..."
    # parallel build doesn't seem to help honestly, but running this func in par. does
    cargo build -j $(nproc) --release --target-dir "build-tmp/${TMP_TARGET}" --bin "${CUR}"
    
    
    mv "build-tmp/${TMP_TARGET}/release/${CUR}" bin


    rm -rf "build-tmp/${TMP_TARGET}"  
}


for file in src/bin/benchmarks/*; do

    filename=$(basename "$file")
    
    exe_name="${filename%.*}"

    echo "Found executable: $exe_name"
    # run in child
    build_and_cp_exe $exe_name &
done

# need children to finish before this we cleanup
wait

rm -rf build-tmp

