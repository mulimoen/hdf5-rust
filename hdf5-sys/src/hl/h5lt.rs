use crate::internal_prelude::*;

use crate::h5t::H5T_class_t;

pub const H5LT_FILE_IMAGE_OPEN_RW: usize = 0x001;
pub const H5LT_FILE_IMAGE_DONT_COPY: usize = 0x002;
pub const H5LT_FILE_IMAGE_DONT_RELEASE: usize = 0x004;
pub const H5LT_FILE_IMAGE_ALL: usize = 0x0007;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum H5LT_lang_t {
    H5LT_LANG_ERR = -1,
    H5LT_DDL = 0,
    H5LT_C = 1,
    H5LT_FORTRAN = 2,
    H5LT_NO_LANG = 3,
}

extern "C" {
    pub fn H5LTmake_dataset(
        loc_id: hid_t, dset_name: *const c_char, rank: c_int, dims: *const hsize_t, type_id: hid_t,
        buffer: *const c_void,
    ) -> herr_t;
    pub fn H5LTmake_dataset_char(
        loc_id: hid_t, dset_name: *const c_char, rank: c_int, dims: *const hsize_t,
        buffer: *const c_char,
    ) -> herr_t;
    pub fn H5LTmake_dataset_short(
        loc_id: hid_t, dset_name: *const c_char, rank: c_int, dims: *const hsize_t,
        buffer: *const c_short,
    ) -> herr_t;
    pub fn H5LTmake_dataset_int(
        loc_id: hid_t, dset_name: *const c_char, rank: c_int, dims: *const hsize_t,
        buffer: *const c_int,
    ) -> herr_t;
    pub fn H5LTmake_dataset_long(
        loc_id: hid_t, dset_name: *const c_char, rank: c_int, dims: *const hsize_t,
        buffer: *const c_long,
    ) -> herr_t;
    pub fn H5LTmake_dataset_float(
        loc_id: hid_t, dset_name: *const c_char, rank: c_int, dims: *const hsize_t,
        buffer: *const c_float,
    ) -> herr_t;
    pub fn H5LTmake_dataset_double(
        loc_id: hid_t, dset_name: *const c_char, rank: c_int, dims: *const hsize_t,
        buffer: *const c_double,
    ) -> herr_t;
    pub fn H5LTmake_dataset_string(
        loc_id: hid_t, dset_name: *const c_char, buf: *const c_char,
    ) -> herr_t;

    pub fn H5LTread_dataset(
        loc_id: hid_t, dset_name: *const c_char, type_id: hid_t, buffer: *mut c_void,
    ) -> herr_t;
    pub fn H5LTread_dataset_char(
        loc_id: hid_t, dset_name: *const c_char, buffer: *mut c_char,
    ) -> herr_t;
    pub fn H5LTread_dataset_short(
        loc_id: hid_t, dset_name: *const c_char, buffer: *mut c_short,
    ) -> herr_t;
    pub fn H5LTread_dataset_int(
        loc_id: hid_t, dset_name: *const c_char, buffer: *mut c_int,
    ) -> herr_t;
    pub fn H5LTread_dataset_long(
        loc_id: hid_t, dset_name: *const c_char, buffer: *mut c_long,
    ) -> herr_t;
    pub fn H5LTread_dataset_float(
        loc_id: hid_t, dset_name: *const c_char, buffer: *mut c_float,
    ) -> herr_t;
    pub fn H5LTread_dataset_double(
        loc_id: hid_t, dset_name: *const c_char, buffer: *mut c_double,
    ) -> herr_t;
    pub fn H5LTread_dataset_string(
        loc_id: hid_t, dset_name: *const c_char, bug: *mut c_char,
    ) -> herr_t;

    pub fn H5LTget_dataset_ndims(
        loc_id: hid_t, dset_name: *const c_char, rank: *mut c_int,
    ) -> herr_t;
    pub fn H5Ltget_dataset_info(
        loc_id: hid_t, dset_name: *const c_char, dims: *mut hsize_t, type_class: *mut H5T_class_t,
        type_size: *mut size_t,
    ) -> herr_t;
    pub fn H5LTfind_dataset(loc_id: hid_t, name: *const c_char) -> herr_t;

    pub fn H5LTset_attribute_string(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, attr_data: *const c_char,
    ) -> herr_t;
    pub fn H5LTset_attribute_char(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, buffer: *const c_char,
        size: size_t,
    ) -> herr_t;
    pub fn H5LTset_attribute_uchar(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, buffer: *const c_uchar,
        size: size_t,
    ) -> herr_t;
    pub fn H5LTset_attribute_short(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, buffer: *const c_short,
        size: size_t,
    ) -> herr_t;
    pub fn H5LTset_attribute_ushort(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, buffer: *const c_ushort,
        size: size_t,
    ) -> herr_t;
    pub fn H5LTset_attribute_int(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, buffer: *const c_int,
        size: size_t,
    ) -> herr_t;
    pub fn H5LTset_attribute_uint(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, buffer: *const c_uint,
        size: size_t,
    ) -> herr_t;
    pub fn H5LTset_attribute_long(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, buffer: *const c_long,
        size: size_t,
    ) -> herr_t;
    pub fn H5LTset_attribute_longlong(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char,
        buffer: *const c_longlong, size: size_t,
    ) -> herr_t;
    pub fn H5LTset_attribute_ulong(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, buffer: *const c_ulong,
        size: size_t,
    ) -> herr_t;
    pub fn H5LTset_attribute_float(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, buffer: *const c_float,
        size: size_t,
    ) -> herr_t;
    pub fn H5LTset_attribute_double(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, buffer: *const c_double,
        size: size_t,
    ) -> herr_t;

    pub fn H5LTget_attribute(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, mem_type_id: hid_t,
        data: *mut c_void,
    ) -> herr_t;
    pub fn H5LTget_attribute_string(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_char,
    ) -> herr_t;
    pub fn H5LTget_attribute_char(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_char,
    ) -> herr_t;
    pub fn H5LTget_attribute_uchar(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_uchar,
    ) -> herr_t;
    pub fn H5LTget_attribute_short(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_short,
    ) -> herr_t;
    pub fn H5LTget_attribute_ushort(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_ushort,
    ) -> herr_t;
    pub fn H5LTget_attribute_int(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_int,
    ) -> herr_t;
    pub fn H5LTget_attribute_uint(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_uint,
    ) -> herr_t;
    pub fn H5LTget_attribute_long(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_long,
    ) -> herr_t;
    pub fn H5LTget_attribute_long_long(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_longlong,
    ) -> herr_t;
    pub fn H5LTget_attribute_ulong(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_ulong,
    ) -> herr_t;
    pub fn H5LTget_attribute_float(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_float,
    ) -> herr_t;
    pub fn H5LTget_attribute_double(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, data: *mut c_double,
    ) -> herr_t;

    pub fn H5LTget_attribute_ndims(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, rank: *mut c_int,
    ) -> herr_t;
    pub fn H5LTget_attribute_info(
        loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char, dims: *mut hsize_t,
        type_class: *mut H5T_class_t, type_size: *mut size_t,
    ) -> herr_t;

    pub fn H5LTtext_to_dtype(text: *const c_char, lang_type: H5LT_lang_t) -> hid_t;
    pub fn H5LTdtype_to_text(
        dtype: hid_t, str_: *mut c_char, lang_type: H5LT_lang_t, len: *mut size_t,
    ) -> herr_t;

    pub fn H5LTfind_attribute(loc_id: hid_t, name: *const c_char) -> herr_t;
    pub fn H5LTpath_valid(
        loc_id: hid_t, path: *const c_char, check_object_valid: hbool_t,
    ) -> htri_t;

    pub fn H5LTopen_file_image(buf_ptr: *mut c_void, buf_size: size_t, flags: c_uint) -> hid_t;
}
