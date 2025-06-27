# Brookshear Machine Assembly Language Guide

This guide explains how to write assembly code for the Brookshear Machine Code Generator. Each instruction follows a specific format and generates corresponding machine code (opcode + operand).

## General Rules
- Register names are written as `R` followed by a hex digit (R0, R1, R2, ..., RF)
- Memory addresses are written in hex format
- Values are written in hex format
- The assembler generates 2 bytes per instruction: opcode byte + operand byte

---

## 1. DATA Instruction

**Purpose**: Store raw data values in memory

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `DATA<hex>` | `<hex>00` | `DATA4A` | `4A00` | Store hex value 4A |
| `DATA<hex>` | `<hex>00` | `DATAFF` | `FF00` | Store hex value FF |
| `DATA<hex>` | `<hex>00` | `DATA01` | `0100` | Store hex value 01 |

---

## 2. HALT Instruction

**Purpose**: Stop program execution

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `HALT` | `C000` | `HALT` | `C000` | Stop execution |

---

## 3. NOP Instruction

**Purpose**: No operation (do nothing)

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `NOP` | `0FFF` | `NOP` | `0FFF` | No operation |

---

## 4. ADD Instructions

### ADDI (Integer Addition)

**Purpose**: Add two registers as integers and store result in destination register

**Format**: `ADDI R<src1>, R<src2> -> R<dest>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `ADDI R<s1>, R<s2> -> R<d>` | `5<d><s1><s2>` | `ADDI R1, R3 -> RC` | `5C13` | Add R1+R3, store in RC |
| `ADDI R<s1>, R<s2> -> R<d>` | `5<d><s1><s2>` | `ADDI R0, R2 -> R5` | `5502` | Add R0+R2, store in R5 |
| `ADDI R<s1>, R<s2> -> R<d>` | `5<d><s1><s2>` | `ADDI RF, RA -> R8` | `58FA` | Add RF+RA, store in R8 |

### ADDF (Floating Point Addition)

**Purpose**: Add two registers as floating point numbers and store result in destination register

**Format**: `ADDF R<src1>, R<src2> -> R<dest>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `ADDF R<s1>, R<s2> -> R<d>` | `6<d><s1><s2>` | `ADDF R2, R4 -> R6` | `6624` | Add R2+R4 (float), store in R6 |
| `ADDF R<s1>, R<s2> -> R<d>` | `6<d><s1><s2>` | `ADDF R0, R1 -> RF` | `6F01` | Add R0+R1 (float), store in RF |
| `ADDF R<s1>, R<s2> -> R<d>` | `6<d><s1><s2>` | `ADDF R7, R9 -> R3` | `6379` | Add R7+R9 (float), store in R3 |

---

## 5. Logical Operations

### OR Operation

**Purpose**: Bitwise OR of two registers

**Format**: `OR R<src1>, R<src2> -> R<dest>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `OR R<s1>, R<s2> -> R<d>` | `7<d><s1><s2>` | `OR R1, R2 -> R3` | `7312` | R1 OR R2, store in R3 |
| `OR R<s1>, R<s2> -> R<d>` | `7<d><s1><s2>` | `OR R4, R5 -> R0` | `7045` | R4 OR R5, store in R0 |
| `OR R<s1>, R<s2> -> R<d>` | `7<d><s1><s2>` | `OR RF, RA -> RB` | `7BFA` | RF OR RA, store in RB |

### AND Operation

**Purpose**: Bitwise AND of two registers

**Format**: `AND R<src1>, R<src2> -> R<dest>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `AND R<s1>, R<s2> -> R<d>` | `8<d><s1><s2>` | `AND R1, R2 -> R3` | `8312` | R1 AND R2, store in R3 |
| `AND R<s1>, R<s2> -> R<d>` | `8<d><s1><s2>` | `AND R6, R7 -> R8` | `8867` | R6 AND R7, store in R8 |
| `AND R<s1>, R<s2> -> R<d>` | `8<d><s1><s2>` | `AND R0, RF -> R1` | `810F` | R0 AND RF, store in R1 |

### XOR Operation

**Purpose**: Bitwise XOR of two registers

**Format**: `XOR R<src1>, R<src2> -> R<dest>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `XOR R<s1>, R<s2> -> R<d>` | `9<d><s1><s2>` | `XOR R1, R2 -> R3` | `9312` | R1 XOR R2, store in R3 |
| `XOR R<s1>, R<s2> -> R<d>` | `9<d><s1><s2>` | `XOR R4, R8 -> RC` | `9C48` | R4 XOR R8, store in RC |
| `XOR R<s1>, R<s2> -> R<d>` | `9<d><s1><s2>` | `XOR RA, RB -> R5` | `95AB` | RA XOR RB, store in R5 |

---

## 6. ROT (Rotate) Instruction

**Purpose**: Rotate bits in a register

**Format**: `ROT R<register>, <rotations>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `ROT R<r>, <n>` | `A<r>0<n>` | `ROT R3, 2` | `A302` | Rotate R3 by 2 positions |
| `ROT R<r>, <n>` | `A<r>0<n>` | `ROT R7, 4` | `A704` | Rotate R7 by 4 positions |
| `ROT R<r>, <n>` | `A<r>0<n>` | `ROT RC, 1` | `AC01` | Rotate RC by 1 position |

---

## 7. MOV Instructions

### Move Register to Register

**Format**: `MOV R<src> -> R<dest>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `MOV R<s> -> R<d>` | `40<s><d>` | `MOV R1 -> R2` | `4012` | Copy R1 to R2 |
| `MOV R<s> -> R<d>` | `40<s><d>` | `MOV R5 -> RF` | `405F` | Copy R5 to RF |
| `MOV R<s> -> R<d>` | `40<s><d>` | `MOV RA -> R3` | `40A3` | Copy RA to R3 |

### Move Value to Register

**Format**: `MOV <value> -> R<dest>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `MOV <v> -> R<d>` | `2<d><value>` | `MOV 4A -> R3` | `234A` | Store value 4A in R3 |
| `MOV <v> -> R<d>` | `2<d><value>` | `MOV FF -> R1` | `21FF` | Store value FF in R1 |
| `MOV <v> -> R<d>` | `2<d><value>` | `MOV 01 -> RC` | `2C01` | Store value 01 in RC |

### Move Memory to Register

**Format**: `MOV [<address>] -> R<dest>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `MOV [<addr>] -> R<d>` | `1<d><addr>` | `MOV [3A] -> R2` | `123A` | Load from address 3A to R2 |
| `MOV [<addr>] -> R<d>` | `1<d><addr>` | `MOV [FF] -> R5` | `15FF` | Load from address FF to R5 |
| `MOV [<addr>] -> R<d>` | `1<d><addr>` | `MOV [10] -> RA` | `1A10` | Load from address 10 to RA |

### Move Register to Memory

**Format**: `MOV R<src> -> [<address>]`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `MOV R<s> -> [<addr>]` | `3<s><addr>` | `MOV R4 -> [2C]` | `342C` | Store R4 to address 2C |
| `MOV R<s> -> [<addr>]` | `3<s><addr>` | `MOV R1 -> [A0]` | `31A0` | Store R1 to address A0 |
| `MOV R<s> -> [<addr>]` | `3<s><addr>` | `MOV RF -> [15]` | `3F15` | Store RF to address 15 |

### Move Register to Indirect Memory

**Format**: `MOV R<src> -> [R<dest>]`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `MOV R<s> -> [R<d>]` | `E0<s><d>` | `MOV R3 -> [R5]` | `E035` | Store R3 to address in R5 |
| `MOV R<s> -> [R<d>]` | `E0<s><d>` | `MOV R1 -> [R2]` | `E012` | Store R1 to address in R2 |
| `MOV R<s> -> [R<d>]` | `E0<s><d>` | `MOV RA -> [RC]` | `E0AC` | Store RA to address in RC |

### Move Indirect Memory to Register

**Format**: `MOV [R<src>] -> R<dest>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `MOV [R<s>] -> R<d>` | `D0<d><s>` | `MOV [R2] -> R4` | `D042` | Load from address in R2 to R4 |
| `MOV [R<s>] -> R<d>` | `D0<d><s>` | `MOV [R1] -> R3` | `D031` | Load from address in R1 to R3 |
| `MOV [R<s>] -> R<d>` | `D0<d><s>` | `MOV [RF] -> R0` | `D00F` | Load from address in RF to R0 |

---

## 8. Jump Instructions

### Unconditional Jump

**Format**: `JMP <address>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `JMP <addr>` | `B0<addr>` | `JMP 4A` | `B04A` | Jump to address 4A |
| `JMP <addr>` | `B0<addr>` | `JMP 10` | `B010` | Jump to address 10 |
| `JMP <addr>` | `B0<addr>` | `JMP FF` | `B0FF` | Jump to address FF |

### Jump to Register

**Format**: `JMPR <register>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `JMPR <r>` | `F00<r>` | `JMPR 3` | `F003` | Jump to address in register 3 |
| `JMPR <r>` | `F00<r>` | `JMPR A` | `F00A` | Jump to address in register A |
| `JMPR <r>` | `F00<r>` | `JMPR F` | `F00F` | Jump to address in register F |

### Conditional Jump - Equal

#### Jump if Equal (with address)

**Format**: `JMPEQ <address>, R<register>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `JMPEQ <addr>, R<r>` | `B<r><addr>` | `JMPEQ 20, R3` | `B320` | Jump to 20 if R3 equals R0 |
| `JMPEQ <addr>, R<r>` | `B<r><addr>` | `JMPEQ 4A, R1` | `B14A` | Jump to 4A if R1 equals R0 |
| `JMPEQ <addr>, R<r>` | `B<r><addr>` | `JMPEQ FF, R7` | `B7FF` | Jump to FF if R7 equals R0 |

#### Jump if Equal (register-based)

**Format**: `JMPEQR <addr_reg>, R<comp_reg>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `JMPEQR <ar>, R<cr>` | `F<cr>0<ar>` | `JMPEQR 2, R5` | `F502` | Jump to address in R2 if R5 equals R0 |
| `JMPEQR <ar>, R<cr>` | `F<cr>0<ar>` | `JMPEQR 4, R1` | `F104` | Jump to address in R4 if R1 equals R0 |
| `JMPEQR <ar>, R<cr>` | `F<cr>0<ar>` | `JMPEQR A, R3` | `F30A` | Jump to address in RA if R3 equals R0 |

### Conditional Jumps - Comparisons

#### Jump if Less Than

**Format**: `JMPLT R<addr_reg>, R<comp_reg>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `JMPLT R<ar>, R<cr>` | `F<cr>5<ar>` | `JMPLT R2, R3` | `F352` | Jump to address in R2 if R3 < R0 |
| `JMPLT R<ar>, R<cr>` | `F<cr>5<ar>` | `JMPLT R4, R1` | `F154` | Jump to address in R4 if R1 < R0 |
| `JMPLT R<ar>, R<cr>` | `F<cr>5<ar>` | `JMPLT RA, R5` | `F55A` | Jump to address in RA if R5 < R0 |

#### Jump if Greater Than

**Format**: `JMPGT R<addr_reg>, R<comp_reg>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `JMPGT R<ar>, R<cr>` | `F<cr>4<ar>` | `JMPGT R1, R2` | `F241` | Jump to address in R1 if R2 > R0 |
| `JMPGT R<ar>, R<cr>` | `F<cr>4<ar>` | `JMPGT R3, R4` | `F443` | Jump to address in R3 if R4 > R0 |
| `JMPGT R<ar>, R<cr>` | `F<cr>4<ar>` | `JMPGT R7, R8` | `F847` | Jump to address in R7 if R8 > R0 |

#### Jump if Less Than or Equal

**Format**: `JMPLE R<addr_reg>, R<comp_reg>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `JMPLE R<ar>, R<cr>` | `F<cr>3<ar>` | `JMPLE R2, R5` | `F532` | Jump to address in R2 if R5 <= R0 |
| `JMPLE R<ar>, R<cr>` | `F<cr>3<ar>` | `JMPLE R6, R1` | `F136` | Jump to address in R6 if R1 <= R0 |
| `JMPLE R<ar>, R<cr>` | `F<cr>3<ar>` | `JMPLE RC, R9` | `F93C` | Jump to address in RC if R9 <= R0 |

#### Jump if Greater Than or Equal

**Format**: `JMPGE R<addr_reg>, R<comp_reg>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `JMPGE R<ar>, R<cr>` | `F<cr>2<ar>` | `JMPGE R1, R3` | `F321` | Jump to address in R1 if R3 >= R0 |
| `JMPGE R<ar>, R<cr>` | `F<cr>2<ar>` | `JMPGE R4, R7` | `F724` | Jump to address in R4 if R7 >= R0 |
| `JMPGE R<ar>, R<cr>` | `F<cr>2<ar>` | `JMPGE RF, R2` | `F22F` | Jump to address in RF if R2 >= R0 |

#### Jump if Not Equal

**Format**: `JMPNE R<addr_reg>, R<comp_reg>`

| Input Format | Output Format | Example Input | Example Output | Description |
|--------------|---------------|---------------|----------------|-------------|
| `JMPNE R<ar>, R<cr>` | `F<cr>1<ar>` | `JMPNE R3, R4` | `F413` | Jump to address in R3 if R4 != R0 |
| `JMPNE R<ar>, R<cr>` | `F<cr>1<ar>` | `JMPNE R1, R5` | `F511` | Jump to address in R1 if R5 != R0 |
| `JMPNE R<ar>, R<cr>` | `F<cr>1<ar>` | `JMPNE R8, RA` | `FA18` | Jump to address in R8 if RA != R0 |

---

## Important Notes

1. **Hex Values**: All numeric values should be written in hexadecimal format
2. **Register Names**: Always use `R` followed by a hex digit (0-F)
3. **Address Format**: Memory addresses in square brackets `[address]` or `[Rn]` for indirect
4. **Case Sensitivity**: Instructions appear to be case-sensitive
5. **Spacing**: Pay attention to spacing around operators like `->` and commas
6. **Two-Byte Output**: Every instruction generates exactly 2 bytes of machine code

## Sample Program

```assembly
MOV 20 -> R1        # Load value 20 into R1 -> output: 2120
MOV 30 -> R2        # Load value 30 into R2 -> output: 2230  
ADDI R1, R2 -> R3   # Add R1 and R2, store in R3 -> output: 5312
MOV R3 -> [40]      # Store R3 to memory address 40 -> output: 3340
HALT                # Stop execution -> output: C000
```