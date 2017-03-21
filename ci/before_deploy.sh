# `before_deploy` phase: here we package the build artifacts

set -ex

# create a "staging" directory
mkdir staging

# TODO update this part to copy the artifacts that make sense for your project
# NOTE All Cargo build artifacts will be under the 'target/$TARGET/{debug,release}'
cp target/$TARGET/release/crowbook staging

cd staging

# release tarball will look like 'rust-everywhere-v1.2.3-x86_64-unknown-linux-gnu.tar.gz'
tar czf ../${PROJECT_NAME}_${TRAVIS_TAG}_${TARGET}.tar.gz *





