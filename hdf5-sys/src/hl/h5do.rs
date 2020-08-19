use crate::internal_prelude::*;

extern "C" {
    pub fn H5DOappend(
        dset_id: hid_t, dxpl_id: hid_t, axis: c_uint, extensions: size_t, memtype: hid_t,
        buf: *const c_void,
    ) -> herr_t;
    #[cfg_attr(hdf5_1_10_3, deprecated(note = "Deprecated in HDF5 1.10.3, use H5Dwrite_chunk()"))]
    pub fn H5DOwrite_chunk(
        dset_id: hid_t, dxpl_id: hid_t, filters: u32, offset: *const hsize_t, data_size: size_t,
        buf: *const c_void,
    ) -> herr_t;
    #[cfg_attr(hdf5_1_10_3, deprecated(note = "Deprecated in HDF5 1.10.3, use H5Dread_chunk()"))]
    pub fn H5DOread_chunk(
        dset_id: hid_t, dxpl_id: hid_t, offset: *const hsize_t, filters: *mut u32, buf: *mut c_void,
    ) -> herr_t;
}
