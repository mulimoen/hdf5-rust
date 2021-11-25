use std::env;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;

use bzip2::read::BzDecoder;
use cfg_if::cfg_if;
use tar::Archive;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let uri = format!("{}/{}", DOWNLOAD_URL, DOWNLOAD_BINARY);
    let archive_path = out_dir.join("hdf5-conda.tar.bz2");
    if archive_path.exists() {
        println!("Using existing archive");
    } else {
        println!("Download archive");
        download(&uri, "hdf5-conda.tar.bz2", &out_dir);
        extract(&archive_path, &out_dir);
    }

    let inc_dir = out_dir.join(INC_PATH);
    let root_dir = out_dir.join(LIB_PATH);

    println!("cargo:library={}", LIB_NAME);
    println!("cargo:root={}", root_dir.display());
    println!("cargo:include={}", inc_dir.display());
}

pub const DOWNLOAD_URL: &str = "https://anaconda.org/conda-forge/hdf5/1.12.1/download";

const INC_PATH: &str = {
    cfg_if! {
        if #[cfg(any(target_os = "linux", target_os = "macos"))] {
            "include"
        } else if #[cfg(target_os = "windows")] {
            "Library\\include"
        } else {
            compile_error!("This crate can not be used on this platform");
        }
    }
};

const LIB_NAME: &str = {
    cfg_if! {
        if #[cfg(any(target_os = "linux", target_os = "macos"))] {
            "hdf5"
        } else if #[cfg(target_os = "windows")] {
            "libhdf5"
        } else {
            compile_error!("This crate can not be used on this platform");
        }
    }
};

const LIB_PATH: &str = {
    cfg_if! {
        if #[cfg(any(target_os = "linux", target_os = "macos"))] {
            "lib"
        } else if #[cfg(target_os = "windows")] {
            "Library\\lib"
        } else {
            compile_error!("This crate can not be used on this platform");
        }
    }
};

const DOWNLOAD_BINARY: &str = {
    cfg_if! {
        if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
            "linux-64/hdf5-1.12.1-nompi_h2750804_102.tar.bz2"
        } else if #[cfg(all(target_os = "linux", target_arch = "aarch64"))] {
            "linux-aarch64/hdf5-1.12.1-nompi_h774d4d8_102.tar.bz2"
        } else if #[cfg(any(target_os = "osx", target_arch = "x86_64"))] {
            "osx-64/hdf5-1.12.1-nompi_hd9e8a45_102.tar.bz2"
        } else if #[cfg(any(target_os = "windows", target_arch = "x86_64"))] {
            "hdf5-1.12.1-nompi_h2a0e4a3_102.tar.bz2"
        } else {
            compile_error!("This package can not be used on this arch");
        }
    }
};

fn download(uri: &str, filename: &str, out_dir: &Path) {
    let out = PathBuf::from(out_dir.join(filename));

    let f = File::create(&out).unwrap();
    let writer = io::BufWriter::new(f);

    let req = attohttpc::get(uri).read_timeout(Duration::new(90, 0));

    let response = req.send().unwrap();

    if !response.is_success() {
        panic!("Unexpected response code {:?} for {}", response.status(), uri);
    }

    response.write_to(writer).unwrap();
}

fn extract<P: AsRef<Path>, P2: AsRef<Path>>(archive_path: P, extract_to: P2) {
    let file = File::open(archive_path).unwrap();
    let unzipped = BzDecoder::new(file);
    let mut a = Archive::new(unzipped);
    a.unpack(extract_to).unwrap();
}
