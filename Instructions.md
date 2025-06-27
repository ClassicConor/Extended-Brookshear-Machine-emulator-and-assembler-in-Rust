# Extended Brookshear Machine Assembler: Coding Guide

This guide shows exactly how to write assembly statements and their resulting machine code. Each table lists the *input format*, the *machine code format*, plus multiple *example inputs* and *corresponding outputs*. Hex digits are shown uppercase. Simple English explanations accompany each case.

---

## 1. MOV Instructions

Six addressing modes are available. All machine instructions occupy two memory cells (bytes).

| Mode                        | Input Syntax      | Machine Code Format    | Example Input    | Example Output |
| --------------------------- | ----------------- | ---------------------- | ---------------- | -------------- |
| **Immediate → Register**    | `MOV <val> -> Rn` | `[2n] [vv]`            | `MOV 1Ch -> R3`  | `23 1C`        |
|                             |                   |                        | `MOV -5 -> R0`   | `20 FB`        |
| **Reg → Reg**               | `MOV Rm -> Rn`    | `[4m] [0n]`            | `MOV R1 -> R4`   | `41 04`        |
|                             |                   |                        | `MOV RF -> R2`   | `4F 02`        |
| **Direct → Reg**            | `MOV [xy] -> Rn`  | `[1x] [yn]`            | `MOV [A0] -> R5` | `1A 05`        |
|                             |                   |                        | `MOV [2F] -> R0` | `12 F0`        |
| **Reg → Direct**            | `MOV Rn -> [xy]`  | `[3n] [0y]`            | `MOV R2 -> [B3]` | `32 B3`        |
|                             |                   |                        | `MOV R0 -> [00]` | `30 00`        |
| **\[Reg] → Reg** (indirect) | `MOV [Rn] -> Rm`  | `[Dm] [0n]` (opcode D) | `MOV [R4] -> R1` | `D1 04`        |
|                             |                   |                        | `MOV [R0] -> R0` | `D0 00`        |
| **Reg → \[Reg]** (indirect) | `MOV Rn -> [Rm]`  | `[En] [0m]` (opcode E) | `MOV R3 -> [R2]` | `E3 02`        |
|                             |                   |                        | `MOV R7 -> [R7]` | `E7 07`        |

**Notes:**

* In `[2n]`, 2 is opcode and n is destination-register nibble.  In `[vv]`, vv is the one-byte immediate value in hex or two's complement.
* For direct addresses, `x` and `y` are high and low hex digits of the 8-bit address.
* Opcodes D (13) and E (14) cover register-indirect loads and stores.

---

## 2. Register Operations

### 2.1 Rotate

| Operation    | Syntax      | Code Format | Example     | Output  |
| ------------ | ----------- | ----------- | ----------- | ------- |
| Rotate right | `ROT Rn, x` | `[An] [0x]` | `ROT R1, 1` | `A1 01` |
| (x times)    |             |             | `ROT R3, 4` | `A3 04` |

**Meaning:** Bit-pattern in Rn is rotated x times to the right; result in Rn.

### 2.2 Two-operand → one-result

Four-byte instruction word: 1 byte holds opcode+first-source, second byte holds second-source+dest. Format: `[Op m] [np]`, where opcode is the high nibble, m=Rn source nibble, n=Rm, p=Rp.

| Operation | Syntax              | Opcode | Code Format | Examples             | Output  |
| --------- | ------------------- | ------ | ----------- | -------------------- | ------- |
| ADDI      | `ADDI Rn, Rm -> Rp` | 5      | `[5m] [np]` | `ADDI R1, R3 -> R12` | `51 3C` |
|           |                     |        |             | `ADDI R0, R0 -> R0`  | `50 00` |
| ADDF      | `ADDF Rn, Rm -> Rp` | 6      | `[6m] [np]` | `ADDF R2, R2 -> R3`  | `62 23` |
| OR        | `OR Rn, Rm -> Rp`   | 7      | `[7m] [np]` | `OR R4, R5 -> R4`    | `74 54` |
| AND       | `AND Rn, Rm -> Rp`  | 8      | `[8m] [np]` | `AND R1, R1 -> R0`   | `81 10` |
| XOR       | `XOR Rn, Rm -> Rp`  | 9      | `[9m] [np]` | `XOR R0, RF -> R1`   | `90 F1` |

**Tip:** m,n,p are hex digits 0–F. Second byte high nibble = Rm, low nibble = Rp.

---

## 3. Control Instructions

| Instruction    | Syntax         | Opcode           | Code Format | Example        | Output  |
| -------------- | -------------- | ---------------- | ----------- | -------------- | ------- |
| JMP addr       | `JMP xy`       | B                | `[Bx] [0y]` | `JMP 1A`       | `B1 0A` |
| JMP register   | `JMP Rn`       | F                | `[Fn] [00]` | `JMP R3`       | `F3 00` |
| JMPEQ addr, Rm | `JMPEQ xy, Rm` | F (with flag EQ) | `[Fx] [0m]` | `JMPEQ 20, R2` | `F2 02` |
| JMPNE Rn, Rm   | `JMPNE Rn, Rm` | F (with flag NE) | `[Fn] [0m]` | `JMPNE R4, R4` | `F4 04` |
| JMPGE Rn, Rm   | `JMPGE Rn, Rm` | F (flag GE)      | `[Fn] [0m]` | `JMPGE R5, R0` | `F5 00` |
| JMPLE Rn, Rm   | `JMPLE Rn, Rm` | F (flag LE)      | `[Fn] [0m]` | `JMPLE R1, R1` | `F1 01` |
| JMPGT Rn, Rm   | `JMPGT Rn, Rm` | F (flag GT)      | `[Fn] [0m]` | `JMPGT R2, R3` | `F2 03` |
| JMPLT Rn, Rm   | `JMPLT Rn, Rm` | F (flag LT)      | `[Fn] [0m]` | `JMPLT RA, R0` | `FA 00` |
| NOP            | `NOP`          | 0                | `[00] [00]` | `NOP`          | `00 00` |
| HALT           | `HALT`         | C                | `[C0] [00]` | `HALT`         | `C0 00` |

**Note:** All conditional jumps share opcode F; low nibble of first byte encodes which condition and/or using second nibble for register or address high.

---

## 4. DATA Directive

| Form                     | Syntax             | Behavior                               | Example             | Memory Bytes |
| ------------------------ | ------------------ | -------------------------------------- | ------------------- | ------------ |
| Byte list                | `DATA v1, v2, ...` | Stores each value in successive bytes. | `DATA -1, "A", 0Fh` | `FF 41 0F`   |
| String (null-terminated) | `DATA "text"`      | ASCII codes + \`00\` terminator.       | `DATA "Hi"`         | `48 69 00`   |

Values can be:

* Two-digit hex (`0A`, `FFh`)
* 8-bit binary (`01010101b`)
* Signed decimal (-128 … +127)
* Floating (`-3.2`, `0.03`)
* A character in quotes (`'c'` or `"c"`)
* A label (address inserted at assembly)

---

This completes the mapping from assembly syntax to machine code. Use these tables as reference when writing your assembler.
