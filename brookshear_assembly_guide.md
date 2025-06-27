# Extended Brookshear Machine Assembly Guide

## File Format and Basic Structure

Each line in your assembly file follows this format:
```
location instruction comment
```

- **location**: Optional address (xy:) or label (name:)
- **instruction**: The operation to perform
- **comment**: Optional text after //

## MOV Instructions

The MOV instruction copies data from source to destination. Here are all six variations:

### 1. MOV value -> Rn (Load Immediate)

| Input Format | Description | Example Input | Machine Code Output | Explanation |
|--------------|-------------|---------------|-------------------|-------------|
| `MOV hex -> Rn` | Hex value to register | `MOV 1C -> R3` | `2 3 1C` | Load hex 1C into R3 |
| `MOV hex -> Rn` | Hex with 'h' suffix | `MOV 2Ah -> R1` | `2 1 2A` | Load hex 2A into R1 |
| `MOV binary -> Rn` | Binary value | `MOV 10110011 -> R4` | `2 4 B3` | Load binary 10110011 (hex B3) into R4 |
| `MOV decimal -> Rn` | Signed decimal | `MOV -25 -> R2` | `2 2 E7` | Load -25 (hex E7 in 2's complement) into R2 |
| `MOV decimal -> Rn` | Positive decimal | `MOV +100 -> R5` | `2 5 64` | Load +100 (hex 64) into R5 |
| `MOV float -> Rn` | Floating point | `MOV 3.5 -> R6` | `2 6 38` | Load 3.5 in BM float format into R6 |
| `MOV char -> Rn` | ASCII character | `MOV "A" -> R7` | `2 7 41` | Load ASCII 'A' (hex 41) into R7 |
| `MOV label -> Rn` | Label reference | `MOV loop -> R8` | `2 8 20` | Load address of 'loop' (e.g., 20) into R8 |

### 2. MOV Rm -> Rn (Register to Register)

| Input Format | Description | Example Input | Machine Code Output | Explanation |
|--------------|-------------|---------------|-------------------|-------------|
| `MOV Rm -> Rn` | Copy register to register | `MOV R3 -> R1` | `4 0 3 1` | Copy contents of R3 to R1 |
| `MOV Rm -> Rn` | Same register copy | `MOV R5 -> R5` | `4 0 5 5` | Copy R5 to itself (no-op effect) |
| `MOV Rm -> Rn` | Different registers | `MOV RA -> R2` | `4 0 A 2` | Copy contents of RA to R2 |

### 3. MOV [xy] -> Rn (Load from Memory)

| Input Format | Description | Example Input | Machine Code Output | Explanation |
|--------------|-------------|---------------|-------------------|-------------|
| `MOV [xy] -> Rn` | Load from address | `MOV [3C] -> R4` | `1 4 3C` | Load value at memory address 3C into R4 |
| `MOV [label] -> Rn` | Load from labeled address | `MOV [data1] -> R2` | `1 2 50` | Load value at 'data1' address (e.g., 50) into R2 |
| `MOV [xy] -> Rn` | Load from address | `MOV [FF] -> R0` | `1 0 FF` | Load value at address FF into R0 |

### 4. MOV Rn -> [xy] (Store to Memory)

| Input Format | Description | Example Input | Machine Code Output | Explanation |
|--------------|-------------|---------------|-------------------|-------------|
| `MOV Rn -> [xy]` | Store to address | `MOV R1 -> [A0]` | `3 1 A0` | Store R1 contents to memory address A0 |
| `MOV Rn -> [label]` | Store to labeled address | `MOV R5 -> [buffer]` | `3 5 80` | Store R5 to 'buffer' address (e.g., 80) |
| `MOV Rn -> [xy]` | Store to address | `MOV RC -> [2F]` | `3 C 2F` | Store RC contents to address 2F |

### 5. MOV [Rm] -> Rn (Indirect Load)

| Input Format | Description | Example Input | Machine Code Output | Explanation |
|--------------|-------------|---------------|-------------------|-------------|
| `MOV [Rm] -> Rn` | Load using register as address | `MOV [R3] -> R1` | `D 1 3 0` | Load from address stored in R3 into R1 |
| `MOV [Rm] -> Rn` | Indirect addressing | `MOV [R7] -> R4` | `D 4 7 0` | Load from address in R7 into R4 |
| `MOV [Rm] -> Rn` | Pointer dereferencing | `MOV [RA] -> R2` | `D 2 A 0` | Load from address in RA into R2 |

### 6. MOV Rn -> [Rm] (Indirect Store)

| Input Format | Description | Example Input | Machine Code Output | Explanation |
|--------------|-------------|---------------|-------------------|-------------|
| `MOV Rn -> [Rm]` | Store using register as address | `MOV R2 -> [R3]` | `E 2 3 0` | Store R2 to address stored in R3 |
| `MOV Rn -> [Rm]` | Indirect store | `MOV R8 -> [R1]` | `E 8 1 0` | Store R8 to address in R1 |
| `MOV Rn -> [Rm]` | Pointer store | `MOV R0 -> [R9]` | `E 0 9 0` | Store R0 to address in R9 |

## Register Operation Instructions

### ROT Rn, x (Rotate Right)

| Input Format | Description | Example Input | Machine Code Output | Before | After | Explanation |
|--------------|-------------|---------------|-------------------|---------|-------|-------------|
| `ROT Rn, x` | Rotate right by x bits | `ROT R1, 1` | `A 1 0 1` | 10110011 | 11011001 | Rotate R1 right by 1 bit |
| `ROT Rn, x` | Rotate right by x bits | `ROT R3, 4` | `A 3 0 4` | 11110000 | 00001111 | Rotate R3 right by 4 bits |
| `ROT Rn, x` | Rotate right by x bits | `ROT R5, 2` | `A 5 0 2` | 01010101 | 01010101 | Rotate R5 right by 2 bits |

### ADDI Rn, Rm -> Rp (Add Integers)

| Input Format | Description | Example Input | Machine Code Output | Rn Value | Rm Value | Result in Rp | Explanation |
|--------------|-------------|---------------|-------------------|-----------|-----------|--------------|-------------|
| `ADDI Rn, Rm -> Rp` | Add signed integers | `ADDI R1, R2 -> R3` | `5 1 2 3` | 15 | 10 | 25 | Add R1(15) + R2(10) = R3(25) |
| `ADDI Rn, Rm -> Rp` | Add with overflow | `ADDI R4, R5 -> R6` | `5 4 5 6` | 120 | 50 | -86 | Add with 8-bit overflow |
| `ADDI Rn, Rm -> Rp` | Add negative numbers | `ADDI R7, R8 -> R9` | `5 7 8 9` | -30 | -20 | -50 | Add two negative numbers |

### ADDF Rn, Rm -> Rp (Add Floating Point)

| Input Format | Description | Example Input | Machine Code Output | Rn Value | Rm Value | Result in Rp | Explanation |
|--------------|-------------|---------------|-------------------|-----------|-----------|--------------|-------------|
| `ADDF Rn, Rm -> Rp` | Add float numbers | `ADDF R1, R2 -> R3` | `6 1 2 3` | 2.5 | 1.25 | 3.75 | Add floating point numbers |
| `ADDF Rn, Rm -> Rp` | Add floats | `ADDF R4, R5 -> R4` | `6 4 5 4` | 0.5 | -0.25 | 0.25 | Add and store in source reg |

### OR Rn, Rm -> Rp (Bitwise OR)

| Input Format | Description | Example Input | Machine Code Output | Rn Value | Rm Value | Result in Rp | Explanation |
|--------------|-------------|---------------|-------------------|-----------|-----------|--------------|-------------|
| `OR Rn, Rm -> Rp` | Bitwise OR operation | `OR R1, R2 -> R3` | `7 1 2 3` | 11001100 | 10101010 | 11101110 | OR each bit position |
| `OR Rn, Rm -> Rp` | OR with same register | `OR R4, R4 -> R5` | `7 4 4 5` | 01010101 | 01010101 | 01010101 | OR register with itself |

### AND Rn, Rm -> Rp (Bitwise AND)

| Input Format | Description | Example Input | Machine Code Output | Rn Value | Rm Value | Result in Rp | Explanation |
|--------------|-------------|---------------|-------------------|-----------|-----------|--------------|-------------|
| `AND Rn, Rm -> Rp` | Bitwise AND operation | `AND R1, R2 -> R3` | `8 1 2 3` | 11001100 | 10101010 | 10001000 | AND each bit position |
| `AND Rn, Rm -> Rp` | Mask operation | `AND R4, R5 -> R6` | `8 4 5 6` | 11111111 | 00001111 | 00001111 | Use R5 as mask for R4 |

### XOR Rn, Rm -> Rp (Bitwise XOR)

| Input Format | Description | Example Input | Machine Code Output | Rn Value | Rm Value | Result in Rp | Explanation |
|--------------|-------------|---------------|-------------------|-----------|-----------|--------------|-------------|
| `XOR Rn, Rm -> Rp` | Bitwise XOR operation | `XOR R1, R2 -> R3` | `9 1 2 3` | 11001100 | 10101010 | 01100110 | XOR each bit position |
| `XOR Rn, Rm -> Rp` | Toggle bits | `XOR R4, R5 -> R4` | `9 4 5 4` | 11110000 | 01010101 | 10100101 | Toggle bits in R4 using R5 |

## Control Instructions

### JMP Instructions (Unconditional Jump)

| Input Format | Description | Example Input | Machine Code Output | Explanation |
|--------------|-------------|---------------|-------------------|-------------|
| `JMP xy` | Jump to address | `JMP 3C` | `B 0 0 3C` | Jump to address 3C |
| `JMP label` | Jump to label | `JMP loop` | `B 0 0 20` | Jump to 'loop' address (e.g., 20) |
| `JMP Rn` | Jump to address in register | `JMP R5` | `F 0 0 5` | Jump to address stored in R5 |

### JMPEQ Instructions (Jump if Equal)

| Input Format | Description | Example Input | Machine Code Output | When Jumps | Explanation |
|--------------|-------------|---------------|-------------------|------------|-------------|
| `JMPEQ xy, Rm` | Jump if Rm equals R0 | `JMPEQ 40, R3` | `B 4 0 3` | R3 = R0 | Jump to 40 if R3 equals R0 |
| `JMPEQ label, Rm` | Jump if equal to label | `JMPEQ end, R1` | `B 8 0 1` | R1 = R0 | Jump to 'end' if R1 equals R0 |
| `JMPEQ Rn, Rm` | Jump if registers equal | `JMPEQ R2, R4` | `F 2 4 0` | R4 = R0 | Jump to address in R2 if R4 = R0 |

### Other Conditional Jumps

| Input Format | Description | Example Input | Machine Code Output | Condition | Explanation |
|--------------|-------------|---------------|-------------------|-----------|-------------|
| `JMPNE Rn, Rm` | Jump if not equal | `JMPNE R3, R1` | `F 3 1 1` | R1 ≠ R0 | Jump to address in R3 if R1 ≠ R0 |
| `JMPGE Rn, Rm` | Jump if greater/equal | `JMPGE R4, R2` | `F 4 2 2` | R2 ≥ R0 | Jump if R2 ≥ R0 (unsigned) |
| `JMPLE Rn, Rm` | Jump if less/equal | `JMPLE R5, R3` | `F 5 3 3` | R3 ≤ R0 | Jump if R3 ≤ R0 (unsigned) |
| `JMPGT Rn, Rm` | Jump if greater | `JMPGT R6, R4` | `F 6 4 4` | R4 > R0 | Jump if R4 > R0 (unsigned) |
| `JMPLT Rn, Rm` | Jump if less | `JMPLT R7, R5` | `F 7 5 5` | R5 < R0 | Jump if R5 < R0 (unsigned) |

### Control Instructions (No Operation/Halt)

| Input Format | Description | Example Input | Machine Code Output | Explanation |
|--------------|-------------|---------------|-------------------|-------------|
| `NOP` | No operation | `NOP` | `0 0 0 0` | Do nothing, takes 2 memory cells |
| `HALT` | Stop execution | `HALT` | `C 0 0 0` | Stop the machine |

## DATA Instructions

### DATA values (Multiple Values)

| Input Format | Description | Example Input | Memory Output | Explanation |
|--------------|-------------|---------------|---------------|-------------|
| `DATA hex,hex` | Multiple hex values | `DATA 1A, 2B, 3C` | `1A 2B 3C` | Store three hex bytes |
| `DATA dec,char` | Mixed data types | `DATA 100, "A", 2F` | `64 41 2F` | Decimal 100, ASCII A, hex 2F |
| `DATA bin,hex` | Binary and hex | `DATA 11110000, A5` | `F0 A5` | Binary and hex values |

### DATA string (String Data)

| Input Format | Description | Example Input | Memory Output | Explanation |
|--------------|-------------|---------------|---------------|-------------|
| `DATA "text"` | String with null terminator | `DATA "Hi"` | `48 69 00` | ASCII H, i, plus null terminator |
| `DATA "a","b"` | String with quotes | `DATA "a","b"` | `61 22 2C 22 62 00` | String: a","b with null |
| `DATA "ab"` | Simple string | `DATA "ab"` | `61 62 00` | ASCII a, b, plus null |

## Addresses and Labels

### Explicit Addresses

| Input Format | Description | Example Input | Effect | Explanation |
|--------------|-------------|---------------|---------|-------------|
| `xy: instruction` | Set address for instruction | `20: MOV R1 -> R2` | Instruction at 20 | Place instruction at address 20 |
| `xy: DATA value` | Set address for data | `80: DATA "text"` | Data starts at 80 | Place data starting at address 80 |
| `xy:` | Set address for next item | `30:` | Next at 30 | Next instruction/data goes to 30 |

### Labels

| Input Format | Description | Example Input | Usage Example | Explanation |
|--------------|-------------|---------------|---------------|-------------|
| `label: instruction` | Define label | `loop: ADDI R1, R2 -> R1` | `JMP loop` | Create label 'loop' for this address |
| `label:` | Label without instruction | `start:` | `MOV start -> R3` | Label for current address |
| Label usage in MOV | Use label as value | `MOV loop -> R4` | Load address | Load label's address into register |
| Label usage in JMP | Use label as target | `JMP start` | Jump to label | Jump to labeled address |

## Complete Example Breakdown

Here's the chessboard example broken down instruction by instruction:

| Line | Input | Machine Code | Address | Explanation |
|------|-------|--------------|---------|-------------|
| 1 | `MOV [dispmem] -> R1` | `1 1 24` | 00-01 | Load address from dispmem into R1 |
| 2 | `MOV 1 -> R2` | `2 2 01` | 02-03 | Load constant 1 into R2 |
| 3 | `MOV [bwpatt] -> R3` | `1 3 26` | 04-05 | Load pattern 1 into R3 |
| 4 | `MOV [wbpatt] -> R4` | `1 4 27` | 06-07 | Load pattern 2 into R4 |
| 5 | `startloop: ADDI R1, R2 -> R1` | `5 1 2 1` | 08-09 | Add 1 to R1 (increment counter) |
| 6 | `MOV R1 -> RA` | `4 0 1 A` | 0A-0B | Copy R1 to RA |
| 7 | `ROT RA, 4` | `A A 0 4` | 0C-0D | Rotate RA right 4 bits |
| 8 | `AND RA, R2 -> RA` | `8 A 2 A` | 0E-0F | Mask bit 0 of RA |

## Key Points to Remember

1. **Hex digits**: Use 0-9, A-F (case insensitive)
2. **Register names**: R0 through RF (16 registers)
3. **Memory addresses**: Two hex digits (00-FF)
4. **Labels**: Start with letter, 4+ characters
5. **Comments**: Use // for comments
6. **Immediate values**: Can be hex, binary, decimal, float, or character
7. **Machine code**: Each instruction becomes 2 memory cells
8. **Execution**: Starts at address 00 unless specified otherwise