pub(crate) trait BitMap {
    const DIM: usize;
    const N_WIDTH: usize;
    fn set_bit(&mut self, row: usize, col: usize);
    fn idx(row: usize, col: usize) -> (usize, u32); // (array index, bit index)
    fn count_bits(&self) -> usize;
}

macro_rules! impl_bm {
    ($tp:ty, $dim:expr) => (
        impl BitMap for [$tp; $dim * ($dim / (std::mem::size_of::<$tp>() * 8))] {
            const N_WIDTH: usize = std::mem::size_of::<$tp>() * 8;
            const DIM: usize = $dim;
            
            fn set_bit(&mut self, row: usize, col: usize) {
                // do nothing on out of bounds index
                if row >= Self::DIM || col >= Self::DIM {
                    return;
                }
                // println!("{}, {}", Self::N_WIDTH, Self::DIM);
                let (arr_idx, bit_idx) = Self::idx(row, col);
                let n = self.get_mut(arr_idx).unwrap();
                let bit = (2 as $tp).pow(bit_idx);
                *n |= bit;
            }
        
            fn idx(row: usize, col: usize) -> (usize, u32) {
                let idx = row * Self::DIM + col;
                (idx / Self::N_WIDTH, (Self::N_WIDTH - (idx % Self::N_WIDTH) - 1) as u32)
            }   
        
            fn count_bits(&self) -> usize {
                let mut count = 0;
                for mut n in *self {
                    while n > 0 {
                        if let 1 = n % 2 {
                            count += 1;
                        }
                        n >>= 1;
                    }
                }
                count
            } 
        }
    )
}

// assumes quadratic bitmap
// impl<const S: usize> BitMap for [u128; S] {
//     const DIM: usize = pow_two_int_sqrt(S * 128);

//     fn set_bit(&mut self, row: usize, col: usize) {
//         // do nothing on out of bounds index
//         if row >= Self::DIM || col >= Self::DIM {
//             return;
//         }
//         println!("{}", Self::DIM);
//         let (arr_idx, bit_idx) = Self::idx(row, col);
//         let n = self.get_mut(arr_idx).unwrap();
//         let bit = 2_u128.pow(bit_idx);
//         *n |= bit;
//     }

//     fn idx(row: usize, col: usize) -> (usize, u32) {
//         let idx = row * Self::DIM + col;
//         (idx / 128, (128 - (idx % 128) - 1) as u32)
//     }   

//     fn count_bits(&self) -> usize {
//         let mut count = 0;
//         for mut n in *self {
//             while n > 0 {
//                 if let 1 = n % 2 {
//                     count += 1;
//                 }
//                 n >>= 1;
//             }
//         }
//         count
//     }
// }

#[test]
fn test_set_bit() {
    let mut bm = [0_u128; 512];
    bm.set_bit(0, 0);
    assert_eq!(bm[0], 2_u128.pow(127));
    bm.set_bit(0, 1);
    assert_eq!(bm[0], 2_u128.pow(127) + 2_u128.pow(126));
    bm.set_bit(1, 200);
    assert_eq!(bm[3], 2_u128.pow(55));

    assert_eq!(bm.count_bits(), 3);
}

const fn _pow_two_int_sqrt(n: usize) -> usize {
    n >> (n.trailing_zeros() / 2)
}

#[test]
fn test_pow_two_int_sqrt() {
    assert_eq!(_pow_two_int_sqrt(1), 1);
    assert_eq!(_pow_two_int_sqrt(4), 2);
    assert_eq!(_pow_two_int_sqrt(16), 4);
    assert_eq!(_pow_two_int_sqrt(64), 8);
}



impl_bm!(u8, 8);
impl_bm!(u16, 16);
impl_bm!(u32, 32);
impl_bm!(u64, 64);
impl_bm!(u128, 128);
impl_bm!(u128, 256);
impl_bm!(u128, 384);
impl_bm!(u128, 512);
impl_bm!(u128, 640);
impl_bm!(u128, 768);
impl_bm!(u128, 896);
impl_bm!(u128, 1024);
impl_bm!(u128, 2048);
impl_bm!(u128, 4096);
impl_bm!(u128, 8192);