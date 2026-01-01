# AI Self-Replicating Worm

An advanced self-replicating worm with real AI decision-making using PyTorch neural networks.

## Features

- **AI Decision-Making**: Uses a trained PyTorch neural network to decide whether to replicate in directories based on file count and depth.
- **Model Saving**: Trains and saves the AI model to `worm_model.pth` for reuse.
- **Anti-Sandbox Evasion**: Detects sandbox environments based on CPU cores, memory, and uptime.
- **Persistence**: Hides file extensions and adds to Windows autorun.
- **Replication**: Scans directories and drops/spawns copies intelligently.

## Requirements

- Python 3.x
- PyTorch
- psutil
- pywin32

Install dependencies:
```
pip install -r requirements.txt
```

## Usage

Run the worm:
```
python main.py
```

Options:
- `--scan`: Scan and spread
- `--hide`: Hide file extensions
- `--autorun`: Add to autorun

## Warning

This is malware simulation for educational purposes. Use at your own risk.

