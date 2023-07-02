# `eth-blockies` Validation Test

Test code and example testcases for validating core algorithm of `eth-blockies` generator



## File/Directory Description

- `catimg-output-to-testcase.rs`: Convert blockies ansi sequence to testcase string
  - Read blockies terminal output from the command `catimg` (version 2.7.0),  
    and print an expected output string.
    
- `gen-rand-testcase-after-check.sh`: Write random testcases with user-defined command
  
- `mod.rs`: Test body implementation
  
- `testcase-example.tar.gz`: Example testcases archive
    
- `testcase` (Directory): 
  - Place testcases here, then run `cargo test -- algorithm_validation --ignored`  
    (or `<PROJECT_ROOT>/scripts/test-algo.sh`) to run algorithm validation.



## Test Procedure

1. Read target testcase file (in `testcase` directory)
2. Generate two blockies data from different sources:
   - \<Expected data\>: Load blockies from testcase file (generated from other external command)
   - \<Generated data\>: Generate blockies with `eth-blockies` library
3. Compare and validate whether \<generated data\> is the same as \<expected data\>



## Testcase File Format

It is OK to create testcase files manually according to the below format, but it is recommended to use the helper script (`gen-rand-testcase-after-check.sh`).

- File name format
  - General seed test file: '*R`resolution`.txt*'
  - Ethereum seed test file: '*R`resolution`_eth.txt*'
  
- File contents format
  - Each line: '*`seed input` `expected output`*'
    - Format of *`expected output`*: '*`color0-rgb-hex`,`color1-rgb-hex`,`color2-rgb-hex`=`indices-left-half`*'
      
- Examples (single line)
  - '*R8_eth.txt*': `0x7fcb209831f438a298789e7723af81e1f5c21da5 6c2242,de66e5,bfc2f0=01012022002002001222202220211120`  
    ![algorithm-validation-tc-example-0.png](https://github.com/snoopy3476/eth-blockies-rs/blob/55867c537aea5a7133d1eb216053b18494e42fd5/tests/algorithm_validation/assets/algorithm-validation-tc-example-0.png?raw=true)
  - '*R5.txt*': `VTNH49eu1IyoHbDbD8pGL 3ab37f,aec754,9c3051=001101101101220`  
    ![algorithm-validation-tc-example-1.png](https://github.com/snoopy3476/eth-blockies-rs/blob/55867c537aea5a7133d1eb216053b18494e42fd5/tests/algorithm_validation/assets/algorithm-validation-tc-example-1.png?raw=true)
