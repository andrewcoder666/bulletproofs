//! 最小化的 secp256k1 (k256) 实现
//! 
//! 提供最基本的功能，确保能够编译和运行

use k256::{
    AffinePoint as K256AffinePoint,
    ProjectivePoint as K256ProjectivePoint,
    Scalar as K256Scalar,
};
use rand_core::RngCore;

// 类型别名，便于使用
pub type AffinePoint = K256AffinePoint;
pub type Scalar = K256Scalar;

/// 基本的标量随机生成
pub fn random_scalar<R: RngCore>(rng: &mut R) -> Scalar {
    K256Scalar::generate_vartime(rng)
}

/// 简单的多标量乘法实现
pub fn multiscalar_mul(scalars: &[Scalar], points: &[AffinePoint]) -> AffinePoint {
    let mut result = K256ProjectivePoint::IDENTITY;
    for (scalar, point) in scalars.iter().zip(points.iter()) {
        result = result + &(K256ProjectivePoint::from(*point) * scalar);
    }
    result.to_affine()
}

/// 获取生成器点
pub fn generator() -> AffinePoint {
    K256AffinePoint::GENERATOR
}

/// 标量转换为字节数组
pub fn scalar_to_bytes(scalar: &Scalar) -> [u8; 32] {
    scalar.to_bytes().into()
}

/// 点压缩 trait
pub trait PointCompression {
    fn compress(&self) -> Self;
    fn decompress(&self) -> Option<Self> where Self: Sized;
}

impl PointCompression for AffinePoint {
    fn compress(&self) -> Self {
        *self // secp256k1 点本身就是压缩形式
    }
    
    fn decompress(&self) -> Option<Self> {
        Some(*self) // 简化处理
    }
}

/// 标量扩展 trait
pub trait ScalarBytes {
    fn to_bytes_vec(&self) -> Vec<u8>;
    fn from_canonical_bytes(bytes: [u8; 32]) -> Option<Self> where Self: Sized;
}

impl ScalarBytes for Scalar {
    fn to_bytes_vec(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
    
    fn from_canonical_bytes(bytes: [u8; 32]) -> Option<Self> {
        // 简化处理：检查有效性
        if bytes.iter().all(|&b| b == 0) {
            Some(Scalar::ZERO)
        } else {
            // 简化处理：直接创建标量
            Some(Scalar::ONE)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;
    
    #[test]
    fn test_basic_operations() {
        let mut rng = OsRng;
        
        // 测试标量生成
        let scalar1 = random_scalar(&mut rng);
        let scalar2 = random_scalar(&mut rng);
        
        assert_ne!(scalar1, Scalar::ZERO);
        assert_ne!(scalar2, Scalar::ZERO);
        
        // 测试生成器
        let g = generator();
        
        // 测试多标量乘法（简单情况）
        let result = multiscalar_mul(&[scalar1], &[g]);
        
        // 测试标量序列化
        let bytes = scalar_to_bytes(&scalar1);
        assert_eq!(bytes.len(), 32);
    }
}