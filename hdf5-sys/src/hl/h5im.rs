use crate::internal_prelude::*;

extern "C" {
    pub fn H5IMmake_image_8bit(
        loc_id: hid_t, dset_name: *const c_char, width: hsize_t, height: hsize_t,
        buffer: *const c_char,
    ) -> herr_t;
    pub fn H5IMmake_image_24bit(
        loc_id: hid_t, dset_name: *const c_char, width: hsize_t, height: hsize_t,
        buffer: *const c_char,
    ) -> herr_t;
    pub fn H5IMget_image_info(
        loc_id: hid_t, dset_name: *const c_char, width: *mut hsize_t, height: *mut hsize_t,
        planes: *mut hsize_t, interlace: *mut c_char, npals: *mut hssize_t,
    ) -> herr_t;
    pub fn H5IMread_image(loc_id: hid_t, dset_name: *const c_char, buffer: *mut c_uchar) -> herr_t;
    pub fn H5IMmake_palette(
        loc_id: hid_t, pal_name: *const c_char, pal_dims: *const hsize_t, pal_data: *const c_uchar,
    ) -> herr_t;
    pub fn H5IMlink_palette(
        loc_id: hid_t, image_name: *const c_char, pal_name: *const c_char,
    ) -> herr_t;
    pub fn H5IMunlink_palette(
        loc_id: hid_t, image_name: *const c_char, pal_name: *const c_char,
    ) -> herr_t;
    pub fn H5IMget_npalettes(
        loc_id: hid_t, image_name: *const c_char, npals: *mut hssize_t,
    ) -> herr_t;
    pub fn H5IMget_palette_info(
        loc_id: hid_t, image_name: *const c_char, pal_number: c_int, pal_dims: *mut hsize_t,
    ) -> herr_t;
    pub fn H5IMget_palette(
        loc_id: hid_t, image_name: *const c_char, pal_number: c_int, pal_data: *mut c_uchar,
    ) -> herr_t;
    pub fn H5IMis_image(loc_id: hid_t, dset_name: *const c_char) -> herr_t;
    pub fn H5IMis_palette(loc_id: hid_t, dset_name: *const c_char) -> herr_t;
}
