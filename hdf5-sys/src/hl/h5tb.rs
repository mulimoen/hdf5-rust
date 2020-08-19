use crate::internal_prelude::*;

extern "C" {
    pub fn H5TBmake_table(
        table_title: *const c_char, loc_id: hid_t, dset_name: *const c_char, nfields: hsize_t,
        nrecords: hsize_t, type_size: size_t, field_names: *const *const c_char,
        field_offset: *const size_t, field_types: *const hid_t, chunk_size: hsize_t,
        fill_data: *mut c_void, compress: c_int, buf: *const c_void,
    ) -> herr_t;

    pub fn H5TBappend_records(
        loc_id: hid_t, dset_name: *const c_char, nrecords: hsize_t, type_size: size_t,
        field_offset: *const size_t, dst_sizes: *const size_t, buf: *const c_void,
    ) -> herr_t;
    pub fn H5TBwrite_records(
        loc_id: hid_t, start: hsize_t, dset_name: *const c_char, nrecords: hsize_t,
        type_size: size_t, field_offset: *const size_t, dst_sizes: *const size_t,
        buf: *const c_void,
    ) -> herr_t;
    pub fn H5TBwrite_fields_name(
        loc_id: hid_t, dset_name: *const c_char, field_names: *const c_char, start: hsize_t,
        nrecords: hsize_t, type_size: size_t, field_offset: *const size_t,
        dst_sizes: *const size_t, buf: *const c_void,
    ) -> herr_t;
    pub fn H5TBwrite_fields_index(
        loc_id: hid_t, dset_name: *const c_char, nfields: hsize_t, field_index: *const c_int,
        start: hsize_t, nrecords: hsize_t, type_size: size_t, field_offset: *const size_t,
        dst_sizes: *const size_t, buf: *const c_void,
    ) -> herr_t;

    pub fn H5TBread_table(
        loc_id: hid_t, dset_name: *const c_char, dst_size: size_t, dst_offset: *const size_t,
        dst_sizes: *const size_t, dst_buf: *mut c_void,
    ) -> herr_t;
    pub fn H5TBread_fields_name(
        loc_id: hid_t, dset_name: *const c_char, field_names: *const c_char, start: hsize_t,
        nrecords: hsize_t, type_size: size_t, field_offset: *const size_t,
        dst_sizes: *const size_t, buf: *mut c_void,
    ) -> herr_t;
    pub fn H5TBread_fields_index(
        loc_id: hid_t, dset_name: *const c_char, nfields: hsize_t, field_index: *const c_int,
        start: hsize_t, nrecords: hsize_t, type_size: size_t, field_offset: *const size_t,
        dst_sizes: *const size_t, buf: *mut c_void,
    ) -> herr_t;
    pub fn H5TBread_records(
        loc_id: hid_t, dset_name: *const c_char, start: hsize_t, nrecords: hsize_t,
        type_size: size_t, dst_offset: *const size_t, dst_sizes: *const size_t, buf: *mut c_void,
    ) -> herr_t;

    pub fn H5TBget_table_info(
        loc_id: hid_t, dset_name: *const c_char, nfields: *mut hsize_t, nrecords: *mut hsize_t,
    ) -> herr_t;
    pub fn H5TBget_field_info(
        loc_id: hid_t, dset_naem: *const c_char, fields_names: *mut *mut c_char,
        fields_sizes: *mut size_t, fields_offsets: *mut size_t, type_size: *mut size_t,
    ) -> herr_t;

    pub fn H5TBdelete_record(
        loc_id: hid_t, dset_name: *const c_char, start: hsize_t, nrecords: hsize_t,
    ) -> herr_t;
    pub fn H5TBinsert_record(
        loc_id: hid_t, dset_name: *const c_char, start: hsize_t, nrecords: hsize_t,
        dst_size: size_t, dst_offset: *const size_t, dst_sizes: *const size_t, buf: *mut c_void,
    ) -> herr_t;
    pub fn H5TBadd_records_from(
        loc_id: hid_t, dset_name1: *const c_char, start1: hsize_t, nrecords: hsize_t,
        dset_name2: *const c_char, start2: hsize_t,
    ) -> herr_t;
    pub fn H5TBcombine_tables(
        loc_id1: hid_t, dset_name1: *const c_char, loc_id2: hid_t, dset_name2: *const c_char,
        dset_name3: *const c_char,
    ) -> herr_t;
    pub fn H5TBinsert_field(
        loc_id: hid_t, dset_name: *const c_char, field_name: *const c_char, field_type: hid_t,
        position: hsize_t, fill_data: *const c_void, buf: *const c_void,
    ) -> herr_t;
    pub fn H5TBdelete_field(
        loc_id: hid_t, dset_name: *const c_char, field_name: *const c_char,
    ) -> herr_t;

    pub fn H5TBAget_title(loc_id: hid_t, table_title: *mut c_char) -> herr_t;
    pub fn H5TBAget_fill(
        loc_id: hid_t, dset_name: *const c_char, dset_id: hid_t, dst_buf: *mut c_uchar,
    ) -> htri_t;
}
