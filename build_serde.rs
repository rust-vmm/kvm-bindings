// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::fs::{self, DirEntry, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Component, Path, PathBuf};

const SER_HDR: &str = "serialization/header.rs.txt";
const SER_SRC: &str = "serialization/src/";
const SER_TPL: &str = "serialization/template.rs.txt";

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    add_serialization(&root);
}

fn add_serialization(root: &Path) {
    let srcdir = root.join("src");
    let archs = ["arm", "arm64", "x86"];

    for arch in archs.into_iter().map(|a| srcdir.join(a)) {
        visit_dirs(&arch, &add_serialization_to_srcfile).unwrap();
    }
}

fn visit_dirs(dir: &Path, bindings_visitor: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file()
                && path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with("bindings_")
            {
                bindings_visitor(&entry)
            }
        }
    }
    Ok(())
}

fn is_relevant(line: &str) -> bool {
    for prefix in vec!["pub struct", "pub union"].iter() {
        if line.starts_with(prefix) {
            for blacklisted in vec!["__BindgenBitfieldUnit", "__IncompleteArrayField"].iter() {
                if line.contains(blacklisted) {
                    return false;
                }
            }
            return true;
        }
    }
    return false;
}

fn add_serialization_to_srcfile(srcfile: &DirEntry) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = Path::new(&manifest_dir);

    let serialization_header_file = root.join(SER_HDR);
    let mut header = String::from("");
    let _ = File::open(serialization_header_file)
        .unwrap()
        .read_to_string(&mut header);

    let template_file = root.join(SER_TPL);
    let mut template = String::from("");
    let _ = File::open(template_file)
        .unwrap()
        .read_to_string(&mut template);

    let srcpath = srcfile.path();
    let mut components = srcpath.components().rev();

    if let Some(Component::Normal(filename)) = components.next() {
        if let Some(Component::Normal(arch)) = components.next() {
            let serializer_fname = format!(
                "serializers_{}",
                &filename.to_str().unwrap()["bindings_".len()..]
            );

            let arch_dir_str = format!("{}/{}/{}", env::var("OUT_DIR").unwrap(), SER_SRC, arch.to_str().unwrap());
            let arch_dir = Path::new(arch_dir_str.as_str());
            if !arch_dir.exists() {
                fs::create_dir_all(arch_dir).unwrap();
            } else if !arch_dir.is_dir() {
                panic!("{:?} exists and is not a directory", arch_dir);
            }

            let mut srcfile_out = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(arch_dir.join(serializer_fname))
                .unwrap();
            srcfile_out.write_all(header.as_bytes()).unwrap();

            let srcfile_in = File::open(srcfile.path()).unwrap();
            let reader = BufReader::new(srcfile_in);
            for line_res in reader.lines() {
                if let Ok(line) = line_res {
                    if is_relevant(&line) {
                        let datastruct_name = line.split_whitespace().collect::<Vec<&str>>()[2];
                        let impl_text = template.replace("TYPENAME", datastruct_name);
                        srcfile_out.write_all(impl_text.as_bytes()).unwrap();
                    }
                }
            }
        }
    }
}
