//!Just doc sample for the mod rust::doc_
//!
//!
//! # Samples
//! ```
//! use rust::doc_::Data;
//!
//! let modData = Data::new();
//! ```
//!

/// Just doc sample for the rust::doc_::Data
///
///
/// # Samples
///
/// ```rust,editable
/// use rust::doc_::Data;
/// let data = Data::new();
/// ```
/// use the [`Data::new`]
///
/// use 2 the [rust::doc_::Data]
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
        Data { name: "".to_owned() }
    }
}
