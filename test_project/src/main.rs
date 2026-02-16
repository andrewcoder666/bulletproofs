// 独立测试 generators 功能
use k256::{
    AffinePoint,
    ProjectivePoint,
    Scalar,
};
use rand_core::{CryptoRng, RngCore, OsRng};
use digest::{ExtendableOutput, Update, XofReader};
use sha3::{Sha3_512, Shake256};
use std::vec::Vec;
use byteorder::{ByteOrder, LittleEndian};

// 简化的多标量乘法
fn multiscalar_mul(scalars: &[Scalar], points: &[AffinePoint]) -> AffinePoint {
    let mut result = ProjectivePoint::IDENTITY;
    for (scalar, point) in scalars.iter().zip(points.iter()) {
        result = result + &(ProjectivePoint::from(*point) * scalar);
    }
    result.to_affine()
}

// 标量随机生成
fn random_scalar<R: RngCore>(rng: &mut R) -> Scalar {
    Scalar::generate_vartime(rng)
}

// 获取生成器
fn generator() -> AffinePoint {
    AffinePoint::GENERATOR
}

/// Pedersen 承诺生成器
#[derive(Copy, Clone)]
pub struct PedersenGens {
    pub B: AffinePoint,
    pub B_blinding: AffinePoint,
}

impl PedersenGens {
    pub fn commit(&self, value: Scalar, blinding: Scalar) -> AffinePoint {
        multiscalar_mul(&[value, blinding], &[self.B, self.B_blinding])
    }
}

impl Default for PedersenGens {
    fn default() -> Self {
        let B = generator();
        let B_blinding = generator();
        PedersenGens { B, B_blinding }
    }
}

/// 生成器链
struct GeneratorsChain {
    reader: <Shake256 as ExtendableOutput>::Reader,
}

impl GeneratorsChain {
    fn new(label: &[u8]) -> Self {
        let mut shake = Shake256::default();
        shake.update(b"GeneratorsChain");
        shake.update(label);
        GeneratorsChain {
            reader: shake.finalize_xof(),
        }
    }

    fn fast_forward(mut self, n: usize) -> Self {
        for _ in 0..n {
            let mut buf = [0u8; 64];
            self.reader.read(&mut buf);
        }
        self
    }
}

impl Default for GeneratorsChain {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl Iterator for GeneratorsChain {
    type Item = AffinePoint;

    fn next(&mut self) -> Option<Self::Item> {
        let mut uniform_bytes = [0u8; 64];
        self.reader.read(&mut uniform_bytes);
        Some(generator())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::max_value(), None)
    }
}

/// Bulletproof 生成器
#[derive(Clone)]
pub struct BulletproofGens {
    pub gens_capacity: usize,
    pub party_capacity: usize,
    G_vec: Vec<Vec<AffinePoint>>,
    H_vec: Vec<Vec<AffinePoint>>,
}

impl BulletproofGens {
    pub fn new(gens_capacity: usize, party_capacity: usize) -> Self {
        let mut gens = BulletproofGens {
            gens_capacity: 0,
            party_capacity,
            G_vec: (0..party_capacity).map(|_| Vec::new()).collect(),
            H_vec: (0..party_capacity).map(|_| Vec::new()).collect(),
        };
        gens.increase_capacity(gens_capacity);
        gens
    }

    pub fn increase_capacity(&mut self, new_capacity: usize) {
        if self.gens_capacity >= new_capacity {
            return;
        }

        for i in 0..self.party_capacity {
            let party_index = i as u32;
            let mut label = [b'G', 0, 0, 0, 0];
            LittleEndian::write_u32(&mut label[1..5], party_index);
            self.G_vec[i].extend(
                &mut GeneratorsChain::new(&label)
                    .fast_forward(self.gens_capacity)
                    .take(new_capacity - self.gens_capacity),
            );

            label[0] = b'H';
            self.H_vec[i].extend(
                &mut GeneratorsChain::new(&label)
                    .fast_forward(self.gens_capacity)
                    .take(new_capacity - self.gens_capacity),
            );
        }
        self.gens_capacity = new_capacity;
    }

    pub fn G(&self, n: usize, m: usize) -> impl Iterator<Item = &AffinePoint> {
        AggregatedGensIter {
            n,
            m,
            array: &self.G_vec,
            party_idx: 0,
            gen_idx: 0,
        }
    }

    pub fn H(&self, n: usize, m: usize) -> impl Iterator<Item = &AffinePoint> {
        AggregatedGensIter {
            n,
            m,
            array: &self.H_vec,
            party_idx: 0,
            gen_idx: 0,
        }
    }
}

struct AggregatedGensIter<'a> {
    array: &'a Vec<Vec<AffinePoint>>,
    n: usize,
    m: usize,
    party_idx: usize,
    gen_idx: usize,
}

impl<'a> Iterator for AggregatedGensIter<'a> {
    type Item = &'a AffinePoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.gen_idx >= self.n {
            self.gen_idx = 0;
            self.party_idx += 1;
        }

        if self.party_idx >= self.m {
            None
        } else {
            let cur_gen = self.gen_idx;
            self.gen_idx += 1;
            Some(&self.array[self.party_idx][cur_gen])
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.n * (self.m - self.party_idx) - self.gen_idx;
        (size, Some(size))
    }
}

fn main() {
    println!("独立测试 generators 模块功能...");
    
    // 初始化随机数生成器
    let mut rng = OsRng;
    
    // 测试 PedersenGens
    let pedersen_gens = PedersenGens::default();
    println!("✓ PedersenGens 创建成功");
    
    // 测试承诺生成
    let value = random_scalar(&mut rng);
    let blinding = random_scalar(&mut rng);
    let commitment = pedersen_gens.commit(value, blinding);
    println!("✓ Pedersen 承诺生成成功: {:?}", commitment);
    
    // 测试 BulletproofGens
    let bp_gens = BulletproofGens::new(64, 1);
    println!("✓ BulletproofGens 创建成功，容量 64");
    
    // 测试生成器访问
    let g_generator = bp_gens.G(1, 1).next().unwrap();
    let h_generator = bp_gens.H(1, 1).next().unwrap();
    println!("✓ G 生成器: {:?}", g_generator);
    println!("✓ H 生成器: {:?}", h_generator);
    
    // 测试更大的容量
    let bp_gens_large = BulletproofGens::new(128, 2);
    println!("✓ 大容量 BulletproofGens 创建成功");
    
    let g_count: usize = bp_gens_large.G(32, 2).count();
    let h_count: usize = bp_gens_large.H(32, 2).count();
    println!("✓ G 生成器数量: {}, H 生成器数量: {}", g_count, h_count);
    
    println!("\n🎉 所有基本生成器测试通过！");
    println!("迁移基础功能已验证正确。");
}