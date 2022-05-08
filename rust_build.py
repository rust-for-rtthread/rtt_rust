import toml
import os
import subprocess

TEMPLATE = {
    "package": {"name": "rust_dummy", "version": "0.0.0", "edition": "2021"},
    "lib": {"name": "rust", "crate-type": ["staticlib"]},
    "dependencies": {},
}

RUSTC_FLAG = {
    "f1": "-C opt-level=z",
    # "-C panic=abort",
    "f2": "-C relocation-model=static",
    # "-C lto",
    # "-C codegen-units=1",
    # "-C debuginfo=2",
    # for main path
    "remap_main": " --remap-path-prefix=%s=",
    # for apps
    "remap_apps": " --remap-path-prefix=%s=apps",
    # for core
    "remap_core": " --remap-path-prefix=%s=core",
    # for alloc
    "remap_alloc": " --remap-path-prefix=%s=alloc",
}


CARGO_CMD = {
    "f1": "cargo rustc",
    "f2": "-Z build-std=core,alloc,compiler_builtins",
    "f3": "--target",
    "target-arch": "%s",
    "f4": "--release",
    "out-path": "--target-dir=%s",
    "f5": "--",
}


RUSTC_CORE_PATH = "lib/rustlib/src/rust/library/core"
RUSTC_ALLOC_PATH = "lib/rustlib/src/rust/library/alloc"

TARGET_ARCH = {
    "cortex-m3": "thumbv7em-none-eabihf",
    "cortex-m4": "thumbv7em-none-eabihf",
    "cortex-m7": "thumbv7em-none-eabihf",
    "cortex-a": "armv7a-none-eabi",
}

DUMMY_FIX = """
#[no_mangle]
pub extern "C" fn _sbrk() {}

#[no_mangle]
pub extern "C" fn _write() {}

#[no_mangle]
pub extern "C" fn _close() {}

#[no_mangle]
pub extern "C" fn _lseek() {}

#[no_mangle]
pub extern "C" fn _read() {}

#[no_mangle]
pub extern "C" fn _fstat() {}

#[no_mangle]
pub extern "C" fn _isatty() {}

#[no_mangle]
pub extern "C" fn _exit() {}

#[no_mangle]
pub extern "C" fn _open() {}

#[no_mangle]
pub extern "C" fn _kill() {}

#[no_mangle]
pub extern "C" fn _getpid() {}
"""


FEATURE_FILE_PATH = ""


def ClearFeature(cwd):
    path = os.path.join(cwd, "rtt_rs2")
    os.system("cd %s; git restore Cargo.toml" % path)


def PrepareSetFeature(cur_pkg_dir):
    global FEATURE_FILE_PATH
    path = os.path.join(cur_pkg_dir, "rtt_rs2")
    FEATURE_FILE_PATH = os.path.join(path, "Cargo.toml")
    os.system("cd %s; git restore Cargo.toml" % path)


def SeleceFeature(feature):
    if FEATURE_FILE_PATH == "":
        print("Rust build: Please call PrepareSetFeature first")
        return
    meta = toml.load(FEATURE_FILE_PATH)
    meta["features"]["default"] += [feature]
    with open(FEATURE_FILE_PATH, "w") as file:
        toml.dump(meta, file)


def PrebuildRust(cur_pkg_dir, arch, rtt_path, app_dir):
    rust_app_proj = []
    rust_app_proj_name = []

    subdir_in_app = os.listdir(app_dir)
    for apps in subdir_in_app:
        if os.path.exists(
            os.path.join(app_dir, apps, "Cargo.toml")
        ) and not os.path.exists(os.path.join(app_dir, apps, ".ignore")):
            rust_app_proj.append(os.path.join(app_dir, apps))

    if len(rust_app_proj) == 0:
        return False

    try:
        arch = TARGET_ARCH[arch]
    except:
        print("Rust build: Not support this ARCH %s" % arch)
        return False

    try:
        # fetch cargo package name
        for proj in rust_app_proj:
            meta = toml.load(os.path.join(proj, "Cargo.toml"))
            rust_app_proj_name.append(meta["package"]["name"])
    except:
        print("Rust build: Error cargo directory")
        return False

    # create statliclib rust-dummy
    if not os.path.exists(os.path.join(cur_pkg_dir, "rust_dummy", "Cargo.toml")):
        if 0 != os.system("cd %s; cargo new --lib rust_dummy" % cur_pkg_dir):
            print("Rust build: Create dummy project failed")
            return False

    # add depend: apps rtt_rt2
    for (name, path) in zip(rust_app_proj_name, rust_app_proj):
        print("Rust add package: %s [%s]" % (name, path))
        TEMPLATE["dependencies"][name] = {"path": path}
        TEMPLATE["dependencies"]["rtt_rs2"] = {
            "path": os.path.join(cur_pkg_dir, "rtt_rs2")
        }

    try:
        # use dependencies
        with open(os.path.join(cur_pkg_dir, "rust_dummy", "src/lib.rs"), "w") as flibrs:
            flibrs.write("#![no_std]\n")
            flibrs.write("\n")
            flibrs.write("extern crate rtt_rs2;\n")
            flibrs.write("pub use rtt_rs2::*;\n")
            for i in rust_app_proj_name:
                flibrs.write("pub use %s::*;\n" % i)
            flibrs.write("\n\n")
            flibrs.write(DUMMY_FIX)
            flibrs.write("\n\n")

        # generate Cargo.toml
        with open(os.path.join(cur_pkg_dir, "rust_dummy", "Cargo.toml"), "w") as ftoml:
            toml.dump(TEMPLATE, ftoml)
    except:
        print("Rust build: Generate dummy file failed")
        return False

    print("Rust build: Success import apps.")

    # fix cargo flag
    try:
        rustc_path = subprocess.check_output("rustc --print sysroot", shell=True)
        rustc_path = str(rustc_path[0:-1], "UTF-8")
    except:
        print("Rust build: rust toolchains error")
        return False

    check = False
    try:
        if os.path.exists(rustc_path):
            check = True
    except:
        pass
    if not check:
        return False

    RUSTC_FLAG["remap_core"] = RUSTC_FLAG["remap_core"] % os.path.join(
        rustc_path, RUSTC_CORE_PATH
    )
    RUSTC_FLAG["remap_alloc"] = RUSTC_FLAG["remap_alloc"] % os.path.join(
        rustc_path, RUSTC_ALLOC_PATH
    )
    RUSTC_FLAG["remap_apps"] = RUSTC_FLAG["remap_apps"] % os.path.abspath(app_dir)
    RUSTC_FLAG["remap_main"] = RUSTC_FLAG["remap_main"] % os.path.abspath(cur_pkg_dir)
    CARGO_CMD["out-path"] = CARGO_CMD["out-path"] % os.path.join(
        cur_pkg_dir, "rust_out"
    )
    CARGO_CMD["target-arch"] = CARGO_CMD["target-arch"] % arch

    print("Rust build: Building rust...")

    # prepare build
    all_rust_flag = ""
    all_cargo_cmd = ""

    for i in RUSTC_FLAG:
        all_rust_flag += " " + RUSTC_FLAG[i]
    for i in CARGO_CMD:
        all_cargo_cmd += " " + CARGO_CMD[i]

    build_path = os.path.join(cur_pkg_dir, "rust_dummy")
    cmd = 'cd %s; RTT_PATH=%s RUSTFLAGS="%s" %s' % (
        build_path,
        # TODO fix build.rs
        rtt_path + "/../",
        all_rust_flag,
        all_cargo_cmd,
    )
    print(cmd)
    if os.system(cmd) != 0:
        print("Prebuild RUST failed.")
        return False
    if (
        os.system(
            "cp %s %s"
            % (
                os.path.join(cur_pkg_dir, ("rust_out/%s/release/librust.a" % arch)),
                os.path.join(cur_pkg_dir, "rust_out"),
            )
        )
        != 0
    ):
        print("Prebuild RUST failed.")
        return False

    return True
