// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use std::env;

fn main() {
    if cfg!(target_os = "windows") && !cfg!(target_env = "msvc") {
        panic!("unsupported compiler");
    }

    let fr = "FEATURE_BABY_BEAR";

    let mut nvcc = cc::Build::new();
    nvcc.cuda(true);
    nvcc.flag("-arch=sm_70");
    #[cfg(not(target_env = "msvc"))]
    nvcc.flag("-Xcompiler").flag("-Wno-unused-function");
    nvcc.define("TAKE_RESPONSIBILITY_FOR_ERROR_MESSAGE", None);
    nvcc.define(fr, None);
    if let Some(include) = env::var_os("DEP_BLST_C_SRC") {
        nvcc.include(include);
    }
    if let Some(include) = env::var_os("DEP_SPPARK_ROOT") {
        nvcc.include(include);
    }
    nvcc.file("cuda/ntt_api.cu").compile("babybear_ntt");

    println!("cargo:rerun-if-changed=cuda");
    println!("cargo:rerun-if-env-changed=CXXFLAGS");
}
