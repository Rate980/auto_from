use from_macro::union_enum;
// macro_rules! _UnionEnum {
//     ($name:ident; $($types:ty: $names:ident),* ) => {
//         enum $name{
//             $($names($types)),*
//         }
//         $(impl From<$types> for $name{
//             fn from(value: $types) -> Self{
//                 Self::$names(value)
//             }
//         })*
//     };
// }
// union_enum! {Data; [String; 3], (u8, u16): B,}
union_enum! {Data<'a, T: Copy>; &'a T: A,}
enum A<'a, T>
where
    T: Copy,
{
    T(&'a T),
}
impl<'a, T: Copy> From<&'a T> for A<'a, T> {
    fn from(value: &'a T) -> Self {
        Self::T(value)
    }
}
fn main() {
    // let a: Data = 2.into();
}
