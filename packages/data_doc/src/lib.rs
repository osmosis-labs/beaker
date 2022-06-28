use std::path::PathBuf;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct DataDoc {
    pub ident: String,
    pub ty: String,
    pub desc: Vec<String>,
    pub sub_docs: Vec<DataDoc>,
}

impl DataDoc {
    pub fn new(ident: String, ty: String, desc: Vec<String>, sub_docs: Vec<DataDoc>) -> Self {
        Self {
            ident,
            ty,
            desc,
            sub_docs,
        }
    }
}

pub trait GetDataDocs {
    fn get_data_docs() -> Vec<DataDoc>;
}

macro_rules! impl_get_docs {
    ($ty:ty) => {
        impl GetDataDocs for $ty {
            fn get_data_docs() -> Vec<DataDoc> {
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
impl_get_docs!(PathBuf);

// tuple, array and slice are purposefully omitted until there is a need for it
// impl_get_docs!(tuple);
// impl_get_docs!(array);
// impl_get_docs!(slice);

impl<K, V> GetDataDocs for std::collections::HashMap<K, V>
where
    V: GetDataDocs,
{
    fn get_data_docs() -> Vec<DataDoc> {
        V::get_data_docs()
    }
}

impl<K, V> GetDataDocs for indexmap::IndexMap<K, V>
where
    V: GetDataDocs,
{
    fn get_data_docs() -> Vec<DataDoc> {
        V::get_data_docs()
    }
}

impl<T> GetDataDocs for Vec<T> {
    fn get_data_docs() -> Vec<DataDoc> {
        vec![]
    }
}

impl<T> GetDataDocs for Option<T> {
    fn get_data_docs() -> Vec<DataDoc> {
        vec![]
    }
}
