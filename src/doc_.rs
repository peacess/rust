//!Just doc sample for the mod mylib::doc_
//!
//!
//! # Samples
//! ```
//! use mylib::doc_::Data;
//!
//! let modData = Data::new();
//! ```
//!


/// Just doc sample for the mylib::doc_::Data
///
///
/// # Samples
///
/// ```rust,editable
/// use mylib::doc_::Data;
/// let data = Data::new();
/// ```
/// use the [`Data::new`]
///
/// use 2 the [crate::doc_::Data]
///
/// see the [`Vec`]
///
/// see1 the [Vec]
///
/// see2 the [std::vec::Vec]
///
pub struct Data {
    /// data's name
    /// see [Data::new]
    pub name: String,
}

impl Data {
    /// new struct [Data]
    /// see the field [Data::name]
    pub fn new() -> Self {
        Data {
            name: "".to_owned(),
        }
    }
}