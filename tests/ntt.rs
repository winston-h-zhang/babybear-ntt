// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use sppark::NTTInputOutputOrder;

const DEFAULT_GPU: usize = 0;

#[test]
fn bb31_self_consistency() {
    use rand::random;

    fn random_fr() -> u32 {
        let fr: u32 = random();
        fr % 0x78000001
    }

    for lg_domain_size in 1..24 + 4 * !cfg!(debug_assertions) as i32 {
        let domain_size = 1usize << lg_domain_size;

        let v: Vec<u32> = (0..domain_size).map(|_| random_fr()).collect();

        let mut vtest1 = v.clone();
        let mut vtest2 = v.clone();

        babybear_ntt::NTT(DEFAULT_GPU, &mut vtest1, NTTInputOutputOrder::NN);

        babybear_ntt::NTT(DEFAULT_GPU, &mut vtest2, NTTInputOutputOrder::RR);
        assert!(vtest1 == vtest2);

        babybear_ntt::iNTT(DEFAULT_GPU, &mut vtest1, NTTInputOutputOrder::NN);

        babybear_ntt::iNTT(DEFAULT_GPU, &mut vtest2, NTTInputOutputOrder::RR);
        assert!(v == vtest1);
        assert!(vtest1 == vtest2);

        babybear_ntt::NTT(DEFAULT_GPU, &mut vtest1, NTTInputOutputOrder::NR);

        babybear_ntt::iNTT(DEFAULT_GPU, &mut vtest1, NTTInputOutputOrder::RN);
        assert!(v == vtest1);
    }
}