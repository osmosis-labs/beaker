#[derive(Default, Debug, PartialEq, Eq)]
pub struct StructDoc {
    pub ident: String,
    pub desc: Vec<String>,
    pub sub_docs: Vec<StructDoc>,
}

impl StructDoc {
    pub fn new(ident: String, desc: Vec<String>, sub_docs: Vec<StructDoc>) -> Self {
        Self {
            ident,
            desc,
            sub_docs,
        }
    }
}

pub trait GetDocs {
    fn get_struct_docs() -> Vec<StructDoc>;
}

macro_rules! impl_get_docs {
    ($ty:ty) => {
        impl GetDocs for $ty {
            fn get_struct_docs() -> Vec<StructDoc> {
                vec![]
            }
        }
    };
}

impl_get_docs!(bool);
impl_get_docs!(char);
impl_get_docs!(i8);
impl_get_docs!(i16);
impl_get_docs!(i32);
impl_get_docs!(i64);
impl_get_docs!(isize);
impl_get_docs!(u8);
impl_get_docs!(u16);
impl_get_docs!(u32);
impl_get_docs!(u64);
impl_get_docs!(usize);
impl_get_docs!(f32);
impl_get_docs!(f64);
impl_get_docs!(str);
impl_get_docs!(String);

// tuple, array and slice are purposefully omitted until there is a need for it
// impl_get_docs!(tuple);
// impl_get_docs!(array);
// impl_get_docs!(slice);

impl<K, V> GetDocs for std::collections::HashMap<K, V> {
    fn get_struct_docs() -> Vec<StructDoc> {
        vec![]
    }
}

impl<T> GetDocs for Vec<T> {
    fn get_struct_docs() -> Vec<StructDoc> {
        vec![]
    }
}
