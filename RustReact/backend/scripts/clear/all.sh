clean_cargo_dir() {
    cd "$1"
    cargo clean
    cd -
}

clean_cargo_dir server/app
