// 最小化测试 generators 模块功能
// 这个测试文件可以直接在 Cargo 环境中运行

use bulletproofs::generators::{PedersenGens, BulletproofGens};
use rand_core::OsRng;

fn main() {
    println!("Testing generators module with k256 backend...");
    
    // 测试 PedersenGens
    let pedersen_gens = PedersenGens::default();
    println!("✓ PedersenGens created successfully");
    
    // 测试承诺生成
    let mut rng = OsRng;
    let value = bulletproofs::secp256k1_impl::random_scalar(&mut rng);
    let blinding = bulletproofs::secp256k1_impl::random_scalar(&mut rng);
    
    let commitment = pedersen_gens.commit(value, blinding);
    println!("✓ Pedersen commitment created: {:?}", commitment);
    
    // 测试 BulletproofGens
    let bp_gens = BulletproofGens::new(64, 1);
    println!("✓ BulletproofGens created with capacity 64");
    
    // 测试生成器访问
    let g_generator = bp_gens.G(1, 1).next().unwrap();
    let h_generator = bp_gens.H(1, 1).next().unwrap();
    println!("✓ G generator: {:?}", g_generator);
    println!("✓ H generator: {:?}", h_generator);
    
    println!("\nAll basic generator tests passed!");
    println!("The migration foundation is working correctly.");
}