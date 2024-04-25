mod u31;
pub use u31::*;

mod u31_ext;
pub use u31_ext::*;

pub fn unroll<F, T>(count: u32, mut closure: F) -> Vec<T>
where
    F: FnMut(u32) -> T,
    T: bitvm::treepp::pushable::Pushable,
{
    let mut result = vec![];

    for i in 0..count {
        result.push(closure(i))
    }
    result
}
