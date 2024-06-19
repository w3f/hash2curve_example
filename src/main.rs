use ark_ec:: hashing::{
    map_to_curve_hasher::MapToCurveBasedHasher,
    HashToCurve,
};

use ark_ff::field_hashers::DefaultFieldHasher;

use ark_ec::hashing::curve_maps::{
	elligator2::Elligator2Map,
	    wb::WBMap};


use sha2::Sha256;

use ark_ec::bls12::Bls12Config;
use ark_bls12_381::{Config as Bls12_381, G1Projective};

use ark_ed_on_bls12_381_bandersnatch::{BandersnatchConfig, EdwardsProjective as BanderSnatchProjective};

fn main() {
    let h2c_hasher_bls12_381 = MapToCurveBasedHasher::<
        G1Projective,
        DefaultFieldHasher<Sha256, 128>,
        WBMap<<Bls12_381 as Bls12Config>::G1Config>,
    >::new(&[1])
    .unwrap();

    let hash_result = h2c_hasher_bls12_381.hash(b"if you stick a Babel fish in your ear you can instantly understand anything said to you in any form of language.").unwrap();

    println!("hashed to bls12-381 {}", hash_result);

    assert!(
        hash_result.is_on_curve(),
        "hash results into a point off the curve"
    );

    let h2c_hasher_bandersnatch = MapToCurveBasedHasher::<
            BanderSnatchProjective,
            DefaultFieldHasher<Sha512, 128>,
            Elligator2Map<BandersnatchConfig>,
        >::new(&[1])
        .unwrap();

        let hash_result = h2c_hasher_bandersnatch.hash(b"if you stick a Babel fish in your ear you can instantly understand anything said to you in any form of language.").expect("fail to hash the string to curve");

    println!("hashed to bandersnatch {}", hash_result);

    assert!(
            hash_result.is_on_curve(),
            "hash results into a point off the curve"
        );

}
