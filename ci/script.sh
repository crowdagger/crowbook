# `script` phase: you usually build, test and generate docs in this phase

set -ex

# TODO modify this phase as you see fit
# PROTIP Always pass `--target $TARGET` to cargo commands, this makes cargo output build artifacts
# to target/$TARGET/{debug,release} which can reduce the number of needed conditionals in the
# `before_deploy`/packaging phase

case $TARGET in
  # use an emulator to run the cross compiled binaries
  arm-unknown-linux-gnueabihf)
    # build tests but don't run them
    cargo test --target $TARGET --no-run

    # run tests in emulator
    find target/$TARGET/debug -maxdepth 1 -executable -type f | \
      xargs qemu-arm -L /usr/arm-linux-gnueabihf

    # build the main executable
    cargo build --target $TARGET

    # run the main executable using the emulator
    qemu-arm -L /usr/arm-linux-gnueabihf target/$TARGET/debug/hello
    ;;
  *)
    cargo build --target $TARGET --verbose
    cargo run --target $TARGET
    cargo test --target $TARGET
    ;;
esac

cargo build --target $TARGET --release

# sanity check the file type
file target/$TARGET/release/crowbook
