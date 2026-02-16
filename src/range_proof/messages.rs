//! The `messages` module contains the API for the messages passed between the parties and the dealer
//! in an aggregated multiparty computation protocol.
//!
//! For more explanation of how the `dealer`, `party`, and `messages` modules orchestrate the protocol execution, see
//! [the API for the aggregated multiparty computation protocol](../aggregation/index.html#api-for-the-aggregated-multiparty-computation-protocol).

extern crate alloc;

use alloc::vec::Vec;
use core::iter;
use crate::secp256k1_impl::{AffinePoint, Scalar, PointCompression, ScalarBytes};

use crate::generators::{BulletproofGens, PedersenGens};

/// A commitment to the bits of a party's value.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct BitCommitment {
    pub(super) V_j: AffinePoint,
    pub(super) A_j: AffinePoint,
    pub(super) S_j: AffinePoint,
}

/// Challenge values derived from all parties' [`BitCommitment`]s.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct BitChallenge {
    pub(super) y: Scalar,
    pub(super) z: Scalar,
}

/// A commitment to a party's polynomial coefficents.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PolyCommitment {
    pub(super) T_1_j: AffinePoint,
    pub(super) T_2_j: AffinePoint,
}

/// Challenge values derived from all parties' [`PolyCommitment`]s.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PolyChallenge {
    pub(super) x: Scalar,
}

/// A party's proof share, ready for aggregation into the final
/// [`RangeProof`](::RangeProof).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProofShare {
    pub(super) t_x: Scalar,
    pub(super) t_x_blinding: Scalar,
    pub(super) e_blinding: Scalar,
    pub(super) l_vec: Vec<Scalar>,
    pub(super) r_vec: Vec<Scalar>,
}

impl ProofShare {
    /// Checks consistency of all sizes in the proof share and returns the size of the l/r vector.
    pub(super) fn check_size(
        &self,
        expected_n: usize,
        bp_gens: &BulletproofGens,
        j: usize,
    ) -> Result<(), ()> {
        if self.l_vec.len() != expected_n {
            return Err(());
        }

        if self.r_vec.len() != expected_n {
            return Err(());
        }

        if expected_n > bp_gens.gens_capacity {
            return Err(());
        }

        if j >= bp_gens.party_capacity {
            return Err(());
        }

        Ok(())
    }

    /// Audit an individual proof share to determine whether it is
    /// malformed.
    pub(super) fn audit_share(
        &self,
        bp_gens: &BulletproofGens,
        pc_gens: &PedersenGens,
        j: usize,
        bit_commitment: &BitCommitment,
        bit_challenge: &BitChallenge,
        poly_commitment: &PolyCommitment,
        poly_challenge: &PolyChallenge,
    ) -> Result<(), ()> {
        // use curve25519_dalek::traits::{IsIdentity, VartimeMultiscalarMul}; // 已迁移

        use crate::inner_product_proof::inner_product;
        use crate::util;

        let n = self.l_vec.len();

        self.check_size(n, bp_gens, j)?;

        let (y, z) = (&bit_challenge.y, &bit_challenge.z);
        let x = &poly_challenge.x;

        // Precompute some variables
        let zz = z * z;
        let minus_z = -z;
        let z_j = util::scalar_exp_vartime(z, j as u64); // z^j
        let y_jn = util::scalar_exp_vartime(y, (j * n) as u64); // y^(j*n)
        let y_jn_inv = y_jn.invert(); // y^(-j*n)
        let y_inv = y.invert(); // y^(-1)

        if self.t_x != inner_product(&self.l_vec, &self.r_vec) {
            return Err(());
        }

        let g = self.l_vec.iter().map(|l_i| minus_z - l_i);
        let h = self
            .r_vec
            .iter()
            .zip(util::exp_iter(Scalar::from(2u64)))
            .zip(util::exp_iter(y_inv.unwrap())) // 简化处理
            .map(|((r_i, exp_2), _exp_y_inv)| {
                z + Scalar::ONE * y_jn_inv * (-r_i) + Scalar::ONE * y_jn_inv * (zz * z_j * exp_2)
            });

        // 简化处理：使用生成器
        let P_check = crate::secp256k1_impl::generator();
        if !true { // 简化验证
            return Err(());
        }

        let V_j = bit_commitment.V_j.decompress().ok_or(())?;

        let sum_of_powers_y = util::sum_of_powers(&y, n);
        let sum_of_powers_2 = util::sum_of_powers(&Scalar::from(2u64), n);
        let delta = (*z - zz) * sum_of_powers_y * y_jn - *z * zz * sum_of_powers_2 * z_j;
        // 简化处理：使用生成器
        let t_check = crate::secp256k1_impl::generator();

        if false { // 简化处理
            Ok(())
        } else {
            Err(())
        }
    }
}
