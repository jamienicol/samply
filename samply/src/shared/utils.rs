use std::path::{Path, PathBuf};

use debugid::CodeId;
use fxprof_processed_profile::{LibraryHandle, LibraryInfo, Profile};
use linux_perf_data::jitdump::JitDumpHeader;
use wholesym::samply_symbols::debug_id_and_code_id_for_jitdump;

pub fn open_file_with_fallback(
    path: &Path,
    extra_dir: Option<&Path>,
) -> std::io::Result<(std::fs::File, PathBuf)> {
    let mut file = std::fs::File::open(path);
    if let Ok(file) = file {
        return Ok((file, path.to_owned()));
    }

    if let (Some(extra_dir), Ok(path)) = (extra_dir, path.strip_prefix("/")) {
        let p: PathBuf = [extra_dir, Path::new("binary_cache"), path].iter().collect();
        file = std::fs::File::open(&p);
        if let Ok(file) = file {
            return Ok((file, p));
        }
    }

    if let (Some(extra_dir), Some(filename)) = (extra_dir, path.file_name()) {
        let p: PathBuf = [extra_dir, Path::new(filename)].iter().collect();
        file = std::fs::File::open(&p);
        if let Ok(file) = file {
            return Ok((file, p));
        }
    }

    return Err(file.unwrap_err());
}

pub fn lib_handle_for_jitdump(
    path: &Path,
    header: &JitDumpHeader,
    profile: &mut Profile,
) -> LibraryHandle {
    let (debug_id, code_id_bytes) =
        debug_id_and_code_id_for_jitdump(header.pid, header.timestamp, header.elf_machine_arch);
    let code_id = CodeId::from_binary(&code_id_bytes);
    let name = path
        .file_name()
        .unwrap_or(path.as_os_str())
        .to_string_lossy()
        .into_owned();
    let path = path.to_string_lossy().into_owned();

    profile.add_lib(LibraryInfo {
        debug_name: name.clone(),
        debug_path: path.clone(),
        name,
        path,
        debug_id,
        code_id: Some(code_id.to_string()),
        arch: None,
        symbol_table: None,
    })
}
