//! 椭圆曲线抽象 trait，为 curve25519-dalek 到 k256 的迁移提供统一接口
//! 
//! 这个模块定义了通用的椭圆曲线操作 trait，使得可以在不同曲线实现之间切换

use core::ops::{Add, Mul};
use rand_core::RngCore;

/// 椭圆曲线标量类型 trait
pub trait EcScalar: 
    Clone + Copy + PartialEq + Eq +
    Add<Output = Self> + Mul<Output = Self> +
    Sized
{
    /// 零元素
    const ZERO: Self;
    /// 一元素
    const ONE: Self;
    
    /// 从字节创建标量
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
    
    /// 转换为字节数组
    fn to_bytes(&self) -> [u8; 32];
    
    /// 随机生成标量
    fn random<R: RngCore + ?Sized>(rng: &mut R) -> Self;
    
    /// 计算逆元
    fn invert(&self) -> Self;
    
    /// 检查是否为零
    fn is_zero(&self) -> bool;
}

/// 椭圆曲线点 trait
pub trait EcPoint:
    Clone + Copy + PartialEq + Eq +
    Add<Output = Self> + Mul<Self::Scalar, Output = Self> +
    Sized
{
    /// 对应的标量类型
    type Scalar: EcScalar;
    
    /// 压缩点类型
    type Compressed: EcCompressedPoint<Point = Self>;
    
    /// 无穷远点
    const IDENTITY: Self;
    
    /// 基点
    const GENERATOR: Self;
    
    /// 多标量乘法
    fn multiscalar_mul<I, J>(scalars: I, points: J) -> Self
    where
        I: IntoIterator,
        I::Item: core::borrow::Borrow<Self::Scalar>,
        J: IntoIterator,
        J::Item: core::borrow::Borrow<Self>;

    /// 可变时间多标量乘法（性能优化版本）
    fn vartime_multiscalar_mul<I, J>(scalars: I, points: J) -> Self
    where
        I: IntoIterator,
        I::Item: core::borrow::Borrow<Self::Scalar>,
        J: IntoIterator,
        J::Item: core::borrow::Borrow<Self>;

    /// 压缩点
    fn compress(&self) -> Self::Compressed;
    
    /// 检查是否为无穷远点
    fn is_identity(&self) -> bool;
}

/// 压缩点 trait
pub trait EcCompressedPoint: 
    Clone + Copy + PartialEq + Eq + Sized
{
    /// 对应的点类型
    type Point: EcPoint<Compressed = Self>;
    
    /// 从字节创建压缩点
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
    
    /// 转换为字节数组
    fn to_bytes(&self) -> Vec<u8>;
    
    /// 解压缩为点
    fn decompress(&self) -> Option<Self::Point>;
}

/// 曲线参数 trait
pub trait CurveParams {
    /// 点类型
    type Point: EcPoint;
    /// 标量类型
    type Scalar: EcScalar;
    /// 压缩点类型
    type CompressedPoint: EcCompressedPoint<Point = Self::Point>;
    
    /// 曲线名称
    const NAME: &'static str;
    /// 标量字段大小（字节）
    const SCALAR_SIZE: usize;
    /// 点压缩大小（字节）
    const COMPRESSED_SIZE: usize;
}