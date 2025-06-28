# Here’s a cleaned‑up version of the instruction table with corrected opcodes and consistent formatting

| Instruction & Description                                         | Machine‑code | Input                                           | Output / Behavior                                 |
| ----------------------------------------------------------------- | :----------: | ----------------------------------------------- | ------------------------------------------------- |
| **No‑Op**                                                         |    `0FFF`    | PC=00                                           | Does nothing; PC ← PC + 2                         |
| **Load (direct)**                                                 |  `1 A 3 7`   | PC=10, M\[0x37]=0x9C, R\[A]=0x00                | R\[A] ← M\[0x37] (0x9C); PC ← 0x12                |
| **Load immediate**                                                |  `2 B 0 F`   | PC=00, R\[B]=0x00                               | R\[B] ← 0x0F; PC ← 0x02                           |
| **Store (direct)**                                                |  `3 C 4 2`   | PC=20, R\[C]=0x55, M\[0x42]=0x00                | M\[0x42] ← R\[C] (0x55); PC ← 0x22                |
| **Move register**                                                 |  `40 1 2 3`  | PC=08, R\[1]=0xAB, R\[2]=0x00                   | R\[2] ← R\[1] (0xAB); PC ← 0x0A                   |
| **Add (two’s‑complement)**                                        |  `5 0 1 2`   | PC=04, R\[1]=0x05, R\[2]=0xFB                   | R\[0] ← 5 + (–5) = 0; PC ← 0x06                   |
| **Add (floating‑point)**                                          |  `6 3 4 5`   | PC=0C, R\[4]=0x4100 (–2.0), R\[5]=0x4200 (+4.0) | R\[3] ← –2.0 + 4.0 = +2.0 ≈ 0x4000; PC ← 0x0E     |
| **OR**                                                            |  `7 A B C`   | PC=14, R\[B]=0xF0, R\[C]=0x0F                   | R\[A] ← 0xF0 OR 0x0F = 0xFF; PC ← 0x16            |
| **AND**                                                           |  `8 2 3 4`   | PC=18, R\[3]=0xAA, R\[4]=0xCC                   | R\[2] ← 0xAA AND 0xCC = 0x88; PC ← 0x1A           |
| **XOR**                                                           |  `9 5 6 7`   | PC=1C, R\[6]=0xFF, R\[7]=0x0F                   | R\[5] ← 0xFF XOR 0x0F = 0xF0; PC ← 0x1E           |
| **Rotate right**                                                  |  `A 7 0 4`   | PC=30, R\[7]=0b10010110, x=4                    | R\[7] ← ROR(R\[7],4) = 0b01101001; PC ← 0x32      |
| **Jump (unconditional)**                                          |   `B0 2 0`   | PC=00                                           | PC ← 0x20                                         |
| **Jump if equal (R\[r] == R0)**                                   |   `B1 1 2`   | PC=10, R\[1]=0x10, R\[0]=0x10                   | R1==R0 → PC ← 0x12, else PC ← 0x12                |
| **Halt**                                                          |    `C000`    | PC=22                                           | Stop execution                                    |
| **Load indirect**                                                 |  `D A 1 2`   | PC=40, R\[2]=0x80, M\[0x80]=0x3C                | R\[A] ← M\[R\[2]] (0x3C); PC ← 0x42               |
| **Store indirect**                                                |  `E 7 3 4`   | PC=50, R\[7]=0xDE, R\[4]=0x20                   | M\[R\[4]] ← R\[7] → M\[0x20]=0xDE; PC ← 0x52      |
| **Jump to register**                                              |    `F00B`    | PC=70, R\[B]=0x90                               | PC ← R\[B] (0x90)                                 |
| **Conditional jump to reg** (x=0:==,1:≠,2:≥,3:≤,4:>,5:< unsigned) |  `F A 4 2`   | PC=80, R\[A]=0x05, R\[0]=0x02, R\[2]=0x40       | 5>2 → true → PC ← R\[2] (0x40); false → PC ← 0x82 |

## Key points

* **2‑byte instructions**: each is 4 hex digits; first digit = opcode
* **PC auto‑increments** by 2 unless a jump changes it
* **Addressing modes**: direct, immediate, register‑indirect, and jumps
* **Display**: memory 0x80–0xFF maps to 32×32 monochrome bitmap

Let me know if you’d like any part expanded or an example program walkthrough!
