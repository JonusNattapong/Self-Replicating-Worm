# AI Self-Replicating Worm (Dropper)

A highly advanced malware prototype featuring AI-driven decision making, polymorphic encryption, PE packing, and self-decrypting executables.

## ⚠️ WARNING
This is **MALWARE**. It is designed to replicate itself across systems. Use only in controlled environments for research purposes.

## Features

### Core Functionality
- **AI Decision Making**: Heuristic-based directory selection for optimal spreading
- **Sandbox Detection**: Evades analysis environments (CPU cores, memory, processes, uptime)
- **Registry Manipulation**: Hides file extensions and adds to autorun
- **Self-Replication**: Intelligent file dropping with random names

### Advanced Evasion Techniques
- **Polymorphic Encryption**: XOR encryption with random keys per copy
- **PE Packing**: Custom executable packing with metadata
- **Self-Decrypting Executables**: Assembly stub that decrypts payload at runtime
- **Process Injection Framework**: Ready for stealthy execution (placeholder)

## Architecture

```
Final Executable = [Assembly Stub] + [Signature] + [Size] + [Key] + [Encrypted Payload]
                   └─ Runs immediately   └─ Metadata ─┘  └─ Decrypted at runtime ─┘
```

1. **Builder (Rust)**: Creates packed executable with encrypted payload
2. **Stub (Assembly)**: Scans for signature, decrypts payload, executes worm
3. **Runtime**: Each copy has unique encryption, evading signature detection

## Compilation

### 1. Compile Assembly Stub
The worm requires a compiled assembly decryption stub to be fully functional.

#### Using NASM (Recommended)
```bash
nasm -f bin stub.asm -o stub.exe
```

#### Using MASM (Alternative)
```bash
ml /c /coff stub.asm
link /subsystem:windows stub.obj
```

### 2. Compile Rust Worm
```bash
cargo build --release
```

## Usage

### Basic Usage
```bash
# Run all features (scan, hide extensions, add to autorun)
./target/release/ai-self-replicating-worm

# Individual features
./target/release/ai-self-replicating-worm --scan
./target/release/ai-self-replicating-worm --hide
./target/release/ai-self-replicating-worm --autorun

# Verbose output
./target/release/ai-self-replicating-worm --verbose
```

### Command Line Options
- `--scan`: Scan and spread the worm
- `--hide`: Hide file extensions in Explorer
- `--autorun`: Add to Windows autorun registry
- `--verbose`: Enable detailed logging

## Technical Details

### Assembly Stub (`stub.asm`)
- **Self-locating**: Uses `call/pop` technique to find its own base address
- **Signature scanning**: Searches memory for "PE_PACK" signature
- **XOR decryption**: Decrypts payload using polymorphic key
- **Memory allocation**: Uses `VirtualAlloc` for executable memory
- **Direct execution**: Jumps to decrypted code in allocated memory

### Polymorphic Encryption
- **Key storage**: First byte of payload contains XOR key
- **Stub preservation**: Only payload is encrypted, stub remains executable
- **Random keys**: Each copy uses different encryption key

### AI Decision Making
- **Heuristics**: Directory depth > 2 AND file count > 10
- **Performance**: Simple calculations for real-time operation
- **Evasion**: Avoids system directories (Windows, System, __pycache__)

## Files

- `src/main.rs`: Main Rust worm implementation
- `stub.asm`: Assembly decryption stub
- `Cargo.toml`: Rust dependencies
- `README.md`: This documentation

## Dependencies

### Rust Crates
- `clap`: Command line argument parsing
- `rand`: Random number generation for encryption
- `walkdir`: Directory traversal
- `winreg`: Windows registry manipulation
- `sysinfo`: System information for sandbox detection

### External Tools
- **NASM** or **MASM**: Assembly compilation
- **Cargo**: Rust build system

## Security Analysis

### Evasion Techniques
1. **Signature Evasion**: Polymorphic encryption creates unique signatures
2. **Behavioral Evasion**: AI decision making avoids predictable patterns
3. **Environmental Evasion**: Sandbox detection prevents analysis
4. **Persistence**: Registry manipulation ensures survival

### Detection Challenges
- **No static signatures**: Each copy is cryptographically unique
- **Dynamic execution**: Payload decrypted only at runtime
- **Stealthy spreading**: Intelligent directory selection
- **Anti-analysis**: Sandbox detection and environmental checks

## Future Enhancements

### Process Injection
- Implement Windows API calls for `VirtualAllocEx`, `WriteProcessMemory`, `CreateRemoteThread`
- Inject into legitimate processes (explorer.exe, notepad.exe)

### Advanced Packing
- Compress payload before encryption
- Multiple encryption layers
- Anti-debugging techniques

### Network Spreading
- SMB/CIFS exploitation
- Email attachment generation
- USB drive infection

### Rootkit Features
- Kernel-mode components
- File system hiding
- Process concealment

## Legal Notice

This code is provided for **educational and research purposes only**. Creating, distributing, or using malware is illegal in most jurisdictions. The author assumes no responsibility for misuse of this code.

## Architecture Diagram

```
[Original Worm Binary]
        │
        ▼
[PE Packer] ──► [Assembly Stub] + [Encrypted Payload]
        │                    │
        │                    ▼
        │          [XOR Decryption in Memory]
        │                    │
        ▼                    ▼
[Registry Setup] ◄─── [Execute Decrypted Worm]
[Extension Hiding]
[Autorun Addition]
[Directory Scanning]
[Intelligent Spreading]
```

This represents the cutting edge of malware development, combining AI intelligence with professional-grade evasion techniques.
