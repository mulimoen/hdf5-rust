use crate::internal_prelude::*;

pub const DIMENSION_SCALE_CLASS: &[u8] = b"DIMENSION_SCALE\0";
pub const DIMENSION_LIST: &[u8] = b"DIMENSION_LIST\0";
pub const REFERENCE_LIST: &[u8] = b"REFERENCE_LIST\0";
pub const DIMENSION_LABELS: &[u8] = b"DIMENSION_LABLES\0";

pub type H5DS_iterate_t = Option<
    extern "C" fn(dset: hid_t, dim: c_uint, scale: hid_t, visitor_data: *mut c_void) -> herr_t,
>;

extern "C" {
    pub fn H5DSattach_scale(did: hid_t, dsid: hid_t, idx: c_uint) -> herr_t;
    pub fn H5DSdetach_scale(did: hid_t, dsid: hid_t, idx: c_uint) -> herr_t;
    pub fn H5DSset_scale(dsid: hid_t, dimname: *const c_char) -> herr_t;
    pub fn H5DSget_num_scales(did: hid_t, dim: c_uint) -> c_int;
    pub fn H5DSset_label(did: hid_t, idx: c_uint, label: *const c_char) -> herr_t;
    pub fn H5DSget_label(did: hid_t, idx: c_uint, label: *mut c_char, size: size_t) -> ssize_t;
    pub fn H5DSget_scale_name(did: hid_t, name: *mut c_char) -> ssize_t;
    pub fn H5DSis_scale(did: hid_t) -> htri_t;
    pub fn H5DSiterate_scales(
        did: hid_t, dim: c_uint, idx: *mut c_int, visitor: H5DS_iterate_t,
        visitor_data: *mut c_void,
    ) -> herr_t;
    pub fn H5DSis_attached(did: hid_t, dsid: hid_t, idx: c_uint) -> htri_t;
}
