use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // 1. 设置路径
    // 获取项目根目录 (Cargo.toml 所在目录)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get manifest dir");
    let root_dir = Path::new(&manifest_dir);

    let scripts_dir = root_dir.join("scripts");
    let bin_dir = root_dir.join("bin");
    let debug_dir = root_dir.join("debug");

    // 2. 确保目标文件夹存在
    fs::create_dir_all(&bin_dir).expect("Failed to create bin dir");
    fs::create_dir_all(&debug_dir).expect("Failed to create debug dir");

    println!("cargo:rerun-if-changed=scripts");

    // 3. 遍历 scripts 目录下的所有 .rs 文件
    let entries = fs::read_dir(&scripts_dir).expect("Failed to read scripts dir");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            compile_and_organize(&path, &bin_dir, &debug_dir);
        }
    }
}

fn compile_and_organize(source_path: &Path, bin_dir: &Path, debug_dir: &Path) {
    let file_name = source_path.file_stem().unwrap().to_str().unwrap();
    let exe_name = format!("{}.exe", file_name);
    let target_exe = bin_dir.join(&exe_name);

    // 检查是否需要重新编译
    if target_exe.exists() {
        if let (Ok(src_meta), Ok(exe_meta)) = (fs::metadata(source_path), fs::metadata(&target_exe)) {
            if let (Ok(src_time), Ok(exe_time)) = (src_meta.modified(), exe_meta.modified()) {
                if src_time <= exe_time {
                    // 源文件没有比目标文件新，跳过编译
                    return;
                }
            }
        }
    }

    println!("cargo:warning=Compiling {}...", file_name);

    // 4. 调用 rustc 进行编译
    // 我们直接输出到 bin 目录
    // 注意：rustc 默认会在当前目录生成 .pdb，或者在输出目录生成
    // Windows 上 rustc -o bin/xxx.exe 会在 bin/ 下生成 xxx.exe 和 xxx.pdb
    
    let status = Command::new("rustc")
        .arg(source_path)
        .arg("--out-dir")
        .arg(bin_dir) // 指定输出目录为 bin
        .arg("-g")    // 生成调试信息
        .status()
        .expect("Failed to execute rustc");

    if !status.success() {
        println!("cargo:warning=Failed to compile {}", file_name);
        return;
    }

    // 5. 移动生成的 .pdb 文件到 debug 文件夹
    // rustc 在 Windows 上会生成 .pdb 文件，通常和 .exe 在同一个目录（这里是 bin_dir）
    let pdb_name = format!("{}.pdb", file_name);
    let generated_pdb = bin_dir.join(&pdb_name);
    let target_pdb = debug_dir.join(&pdb_name);

    if generated_pdb.exists() {
        // 如果目标存在，先删除，防止移动失败（Windows 特性）
        if target_pdb.exists() {
            fs::remove_file(&target_pdb).unwrap_or_else(|e| {
                println!("cargo:warning=Failed to remove existing pdb: {}", e);
            });
        }

        // 移动文件
        match fs::rename(&generated_pdb, &target_pdb) {
            Ok(_) => println!("cargo:warning=Moved {} to debug folder", pdb_name),
            Err(_) => {
                // 如果跨驱动器移动失败，尝试复制后删除
                fs::copy(&generated_pdb, &target_pdb).expect("Failed to copy pdb");
                fs::remove_file(&generated_pdb).expect("Failed to remove original pdb");
            }
        }
    } else {
        println!("cargo:warning=No PDB file found for {}", file_name);
    }
}
