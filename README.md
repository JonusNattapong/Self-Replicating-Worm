# AI Self-Replicating Worm (C++ Implementation)

A highly advanced malware implementation featuring AI-driven decision making, polymorphic encryption, PE packing, and self-decrypting executables.

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
Final Executable = [Compiled Stub] + [PE_PACK] + [Size] + [Key] + [Encrypted Payload]
                   └─ Runs immediately  └─ Metadata ─┘  └─ Decrypted at runtime ─┘
```

### **Functional Self-Decrypting Executables**
1. **Builder**: Loads `stub.exe` + encrypts payload with polymorphic key
2. **Stub (Assembly)**: Scans memory for "PE_PACK", extracts key, decrypts payload
3. **Runtime**: Allocates executable memory, decrypts worm code, jumps to execution
4. **Result**: Each copy is a **runnable polymorphic executable** that spreads itself

### **Key Innovation: Proper Stub + Payload Separation**
- **Before**: Encrypted entire binary (non-functional)
- **After**: Stub remains executable, only payload encrypted
- **Result**: Dropped files actually run and decrypt themselves at runtime

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

### 2. Compile C++ Worm
```bash
# Using g++ (MinGW)
g++ src/main.cpp -o worm.exe -std=c++17 -lstdc++fs

# Using MSVC
cl src/main.cpp /EHsc /std:c++17 /Fe:worm.exe
```

## How to Use (Step-by-Step Guide)

### **Quick Start (3 Steps)**

#### **Step 1: Compile the Assembly Stub**
```bash
# Download and install NASM (Netwide Assembler)
# Then compile the decryption stub:
nasm -f bin stub.asm -o stub.exe
```

#### **Step 2: Compile the C++ Worm**
```bash
# Using MinGW (recommended for Windows)
g++ src/main.cpp -o worm.exe -std=c++17 -lstdc++fs

# Alternative: Using Microsoft Visual C++
cl src/main.cpp /EHsc /std=c++17 /Fe:worm.exe
```

#### **Step 3: Run the Worm**
```bash
# Basic execution (starts scanning and spreading)
./worm.exe

# Verbose mode (shows detailed logging)
./worm.exe --verbose
```

### **What Happens When You Run It**

#### **Initial Execution:**
```
AI Self-Replicating Worm (C++ Advanced)
Full-Featured Malware Implementation
WARNING: For educational purposes only

[*] Performing environmental analysis...
[+] Starting worm execution...
[*] Starting directory scan...
```

#### **During Operation:**
- **Sandbox Detection**: Checks system resources (RAM > 2GB, CPU cores > 1)
- **Self-Reading**: Dynamically reads its own binary using `GetModuleFileNameA()`
- **Directory Scanning**: Recursively scans directories, counts files
- **AI Decision Making**: Spreads to directories with > 5 files
- **Polymorphic Creation**: Creates `worm_NNNN.exe` files with unique encryption

#### **Sample Output:**
```
[*] Performing environmental analysis...
[*] Starting directory scan...
[SCAN] Documents (12 files)
[+] Dropped functional self-decrypting executable to: C:\Users\...\Documents\worm_1847.exe
[+] Architecture: Stub(512 bytes) + Payload(45056 bytes encrypted)
[+] Spawned process PID: 8473 (Stub will decrypt and execute payload)
```

### **Testing in Safe Environment**

#### **Recommended Test Setup:**
1. **Virtual Machine**: Use VMware/VirtualBox with Windows
2. **Isolated Network**: No internet connection during testing
3. **Backup Important Files**: Test directory with non-critical files
4. **Monitoring Tools**: Process Explorer, Registry Monitor

#### **Safe Testing Commands:**
```bash
# Create test directory with some files
mkdir C:\TestWorm
echo "test file 1" > C:\TestWorm\file1.txt
echo "test file 2" > C:\TestWorm\file2.txt
# ... create more files

# Run worm in test directory
cd C:\TestWorm
/path/to/worm.exe --verbose
```

### **Expected Behavior**

#### **Normal Operation:**
- Creates multiple `worm_*.exe` files in directories with >5 files
- Each copy has unique encryption (different file sizes/hashes)
- Attempts to execute spawned copies
- Modifies registry for persistence (autorun + extension hiding)

#### **In Sandbox/Analysis Environment:**
- Detects low resources (<2GB RAM or 1 CPU core)
- Terminates early with message: `[-] Sandbox detected. Exiting.`

### **Verification Commands**

#### **Check Created Files:**
```bash
# List worm files
dir /s worm_*.exe

# Check file properties (different sizes due to unique encryption)
for %f in (worm_*.exe) do @echo %f & fsutil file queryfilesize %f
```

#### **Check Registry Changes:**
```bash
# Check autorun (requires admin)
reg query "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Run"

# Check extension hiding
reg query "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v HideFileExt
```

#### **Monitor Processes:**
```bash
# Watch for worm processes
tasklist | findstr worm
```

### **Troubleshooting**

#### **Common Issues:**

**"stub.exe not found"**
```bash
# Make sure stub.exe is in the same directory as worm.exe
dir stub.exe
# If missing, recompile:
nasm -f bin stub.asm -o stub.exe
```

**"Cannot read own binary"**
- Run as administrator or check file permissions
- Ensure executable isn't already running

**"Scan error"**
- Some directories may be inaccessible (system directories are skipped)
- Check if running in sandbox/low-resource environment

**No files created**
- Check if current directory has subdirectories with >5 files
- Use `--verbose` to see scanning decisions

### **Advanced Usage**

#### **Custom Compilation:**
```bash
# Optimized release build
g++ src/main.cpp -o worm.exe -std=c++17 -lstdc++fs -O3 -s

# Debug build with symbols
g++ src/main.cpp -o worm_debug.exe -std=c++17 -lstdc++fs -g
```

#### **Analysis Mode:**
```bash
# Run in verbose mode to understand AI decisions
./worm.exe --verbose > worm_log.txt
# Analyze the log to see spreading patterns
```

### **Safety Precautions**

#### **⚠️ Critical Warnings:**
- **Never run on production systems**
- **Always use isolated virtual machines**
- **Backup important data before testing**
- **Monitor system behavior closely**
- **Have cleanup procedures ready**

#### **Cleanup Commands:**
```bash
# Remove created worm files
del /s worm_*.exe

# Reset registry (autorun)
reg delete "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Run" /v SelfReplicatingWorm /f

# Reset extension hiding
reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v HideFileExt /t REG_DWORD /d 0 /f
```

### **Command Line Options**
- `--verbose`: Enable detailed logging of scanning and creation process

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
- **Heuristics**: Directory with > 5 files triggers spreading
- **Performance**: Simple calculations for real-time operation
- **Evasion**: Avoids system directories (Windows, System32, Program Files)

## Files

- `src/main.cpp`: C++ worm implementation with self-decrypting executables
- `stub.asm`: Assembly decryption stub
- `README.md`: This documentation

## Dependencies

### C++ Standard Library Requirements
- **C++17**: Filesystem library, random number generation
- **Windows API**: Process manipulation, registry access

### External Tools
- **NASM** or **MASM**: Assembly compilation
- **g++** or **MSVC**: C++ compilation with Windows support

## Detection & Evasion Case Study of Self-Replicating Malware (Defensive Analysis)

### **Malware Overview**
This implementation demonstrates advanced self-replicating malware that combines multiple evasion techniques. From a defensive perspective, this represents a sophisticated threat that challenges traditional security controls.

### **Primary Evasion Techniques**

#### **1. Polymorphic Encryption (Anti-Signature)**
- **Technique**: Each copy uses unique XOR encryption keys
- **Defensive Challenge**: No static signatures possible - each sample is cryptographically unique
- **Detection Method**: Monitor for suspicious XOR operations or entropy analysis
- **Case Study**: Traditional AV signatures fail as `worm_1234.exe` and `worm_5678.exe` have completely different byte patterns despite identical functionality

#### **2. Runtime Decryption (Anti-Static Analysis)**
- **Technique**: Payload decrypted in memory only at execution time
- **Defensive Challenge**: Static analysis sees only encrypted blob, not malicious code
- **Detection Method**: Memory scanning, API hooking on `VirtualAlloc`, or behavioral analysis
- **Case Study**: On-disk file appears as random data, but `VirtualAlloc` + XOR decryption reveals worm code

#### **3. AI-Driven Spreading (Anti-Predictive Analysis)**
- **Technique**: Intelligent directory selection based on file count heuristics
- **Defensive Challenge**: Unpredictable propagation patterns avoid rule-based detection
- **Detection Method**: Anomaly detection in file creation patterns or entropy changes
- **Case Study**: Worm spreads to directories with >5 files, avoiding empty folders and creating seemingly legitimate file distributions

#### **4. Environmental Awareness (Anti-Sandbox)**
- **Technique**: Detects analysis environments via memory and CPU checks
- **Defensive Challenge**: Terminates execution in sandboxes before malicious behavior
- **Detection Method**: Monitor for system enumeration APIs or unusual early termination
- **Case Study**: Checks for <2GB RAM or single CPU core, common in virtual analysis environments

### **Advanced Evasion Layers**

#### **Stub + Payload Architecture**
```
Detection Challenge: Multi-stage execution
├── Stage 1: Legitimate-looking stub.exe (passes AV scans)
├── Stage 2: Runtime payload decryption in memory
└── Stage 3: Actual worm execution (never touches disk in cleartext)
```

#### **Self-Modification Techniques**
- **Dynamic Self-Reading**: Uses `GetModuleFileNameA()` for deployment flexibility
- **Random Naming**: Generated filenames avoid pattern matching
- **Registry Persistence**: Autorun integration for survival across reboots

### **Defensive Analysis Framework**

#### **Static Analysis Challenges**
```cpp
// What AV sees: Encrypted blob
unsigned char encrypted_payload[] = {0xA1, 0x3F, 0x8B, 0x2C...};

// What executes: Decrypted worm code
// VirtualAlloc + XOR decryption + execution
```

#### **Dynamic Analysis Challenges**
- **Sandbox Evasion**: Terminates before malicious actions in analysis environments
- **Time-Based Evasion**: May delay execution or require specific triggers
- **Process Injection Ready**: Framework for hiding in legitimate processes

#### **Memory Forensics Challenges**
- **No On-Disk Malware**: Payload exists only in memory
- **Encrypted Persistence**: Registry entries may be obfuscated
- **Process Camouflage**: Spawns copies that blend with normal system activity

### **Detection Strategies for Defenders**

#### **1. Behavioral Indicators**
- Monitor for recursive directory scanning with `fs::recursive_directory_iterator`
- Alert on suspicious `CreateProcessA` calls with random executable names
- Track entropy changes in file system (encrypted files appearing)

#### **2. Memory-Based Detection**
- Scan process memory for "PE_PACK" signatures
- Monitor `VirtualAlloc` calls followed by XOR decryption patterns
- Implement runtime memory encryption detection

#### **3. Network-Level Detection**
- Watch for SMB/CIFS connections (future network spreading)
- Monitor USB device enumeration (potential infection vectors)
- Alert on unusual file creation patterns across network shares

#### **4. System-Level Indicators**
- Registry monitoring for suspicious autorun entries
- File extension hiding detection (`HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced`)
- CPU/memory enumeration API calls from unexpected processes

### **Case Study: Full Infection Chain**

```
1. Initial Execution
   ├── Sandbox detection (memory/CPU checks)
   ├── Self-binary reading via GetModuleFileNameA()
   └── Directory scanning begins

2. Propagation Phase
   ├── AI decision: directory with >5 files = target
   ├── Load stub.exe + encrypt payload with random key
   ├── Create worm_NNNN.exe in target directory
   └── Attempt execution of new copy

3. Evasion Techniques Active
   ├── Each copy has unique encryption signature
   ├── Payload decrypted only in memory
   ├── Registry persistence established
   └── Extension hiding activated

4. Detection Evasion
   ├── No static signatures match
   ├── Behavioral patterns appear legitimate
   ├── Memory-only execution
   └── Sandbox termination prevents analysis
```

### **Lessons for Defensive Security**

#### **Key Takeaways:**
1. **Signature-Based Detection is Insufficient**: Polymorphic techniques render static signatures useless
2. **Memory Analysis is Critical**: Modern malware lives primarily in RAM
3. **Behavioral Analysis Must Evolve**: AI-driven malware requires AI-driven detection
4. **Environmental Awareness is Common**: Sandboxes are increasingly detected and evaded

#### **Recommended Defenses:**
- **Memory Scanning**: Real-time process memory analysis
- **API Hooking**: Monitor suspicious Windows API calls
- **Entropy Analysis**: Detect encrypted payloads on disk
- **Anomaly Detection**: ML-based behavioral analysis
- **Network Monitoring**: Watch for lateral movement patterns

This implementation serves as a comprehensive case study for understanding modern malware evasion techniques and developing effective defensive strategies.

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
