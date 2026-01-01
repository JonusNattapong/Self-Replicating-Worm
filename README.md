# AI Self-Replicating Worm

This is an advanced AI-powered self-replicating worm implemented in Rust. It uses simple AI heuristics to decide where to replicate, making it "intelligent" in its spread. Designed for educational purposes only to demonstrate malware techniques.

## Features

- **Real AI-Powered Self-replication (Dropper)**: Uses a trained machine learning decision tree model to intelligently decide where to drop executables based on directory features (file count, depth), and spawn processes.
- **File Extensions Hiding**: Modifies Windows Registry to hide file extensions in Explorer.
- **Autorun Persistence**: Adds itself to Windows autorun registry for persistence.
- **Anti-Sandbox Evasion**: Detects sandbox environments based on CPU cores, RAM, and system uptime.
- **CLI Interface**: Command-line options for selective execution.

## Warning

**This is malware.** Do not run on real systems or production environments. Use only in isolated, controlled virtual machines for educational purposes. The author is not responsible for any damage or misuse.

## Build

```bash
cargo build --release
```

## Usage

Run the executable:

```bash
./target/release/ai-self-replicating-worm.exe
```

Or use options:

```bash
./target/release/ai-self-replicating-worm.exe --help
./target/release/ai-self-replicating-worm.exe --scan
./target/release/ai-self-replicating-worm.exe --hide
./target/release/ai-self-replicating-worm.exe --autorun
```

## Disclaimer

This project is for educational and research purposes only. Understanding malware techniques helps in developing better defenses. Do not distribute or use maliciously.

