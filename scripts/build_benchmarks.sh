#!/bin/bash
# crazy parallel bash build script :0

EXCLUDE_GPU=false

usage(){
    echo "./scripts/build_benchmarks.sh [FLAGS]"
    echo "    [--no-gpu] Exclude GPU benchmark from compilation. Shaves off about several of compile time"
    echo "    [--help] print this menu"
}

build_and_cp_exe(){
    CUR="$1"

    echo "cur $CUR"

    # we need to pass the gpu flag for conditional compilation
    FEATURES="--features gpu"
    [[ "$1" == "--no-gpu" ]] && FEATURES=""


    # just not compiling the binary with the executable is not enough
    # bc the wgpu is in the dependency tree for the cargo toml
    if [[ "$CUR" == "gpu" && "$EXCLUDE_GPU" == "true" ]]; then
        echo "Skipping GPU benchmark compilation..."
        return 0
    fi

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
    # cargo build -j $(nproc) --release --target-dir "build-tmp/${TMP_TARGET}" --bin "${CUR}"
    if [[ "$CUR" == "gpu" ]]; then
        cargo build -j $(nproc) --release --target-dir "build-tmp/${TMP_TARGET}" --bin "${CUR}" --features gpu
    else
        cargo build -j $(nproc) --release --target-dir "build-tmp/${TMP_TARGET}" --bin "${CUR}"
    fi


    mv "build-tmp/${TMP_TARGET}/release/${CUR}" bin


    rm -rf "build-tmp/${TMP_TARGET}"
}

if [[ "$1" == "--help" ]]; then
    usage
    exit 0
fi


if [[ "$1" == "--no-gpu" ]]; then
    EXCLUDE_GPU=true
fi

# make if not exist and clean build dirs
# but do NOT clean up gpu bin
mkdir -p bin
find bin -mindepth 1 ! -name 'gpu' -delete

mkdir -p build-tmp
rm -rf build-tmp/*
# cargo build, even with release flag, puts a bunch of random shit in the build dir
# I just want the executable, so i just put them here and mv them over
# think I can just use cargo clean on that build dir later too


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
