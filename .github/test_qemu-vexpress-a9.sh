echo "Runing build"
echo "RTT_PATH=${RTT_PATH}"
echo "RTT_LIB_CPU_PATH=${RTT_LIB_CPU_PATH}"
rustup default nightly
rustup component add rust-src
git clone --depth=1 https://github.com/RT-Thread/rt-thread.git /opt/rtt/rt-thread
cp .github/rttconfig-qemu-vexpress-a9 /opt/rtt/rtconfig.h
cargo build --manifest-path rtt_rs2/Cargo.toml -Z build-std=core,alloc,compiler_builtins --target armv7a-none-eabi
