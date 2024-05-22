# sha2-on-cq

SHA256 PLONKish circuit implementation based on:

 - [Ariel's notes on the general design harnessing CQ protocol](https://hackmd.io/HckfzlaFRNet6DkOp37f0Q)
 - [spreadsheet with table design](https://docs.google.com/spreadsheets/d/111WhDt-uTLvMEzxNTJU4SA2L_nMubz2wc0b3SsoJTys/edit?usp=sharing)

## Visualisation

Current PLONKish table for an empty input to be hashed can be found in [table.md](./table.md).

## Assumptions

1. We are implementing a SNARK (not necessarily ZK-) for the SHA256 hashing algorithm, where the input message is at most 512 bits long (i.e. single message chunk).
2. We are using [CQ](https://eprint.iacr.org/2022/1763) lookup protocol with tables of size up to 13·2<sup>32</sup>.
3. The whole protocol is a pairing based, non-interactive protocol with a trusted setup based on KZG.
4. Protocol implementation (in particular everything connected with PCS) is done with [arkworks libraries](https://github.com/arkworks-rs).

## Arithmetization and 'plonkization' approach

_(Based on V2 sheet from the link above)_

1. There are **no copy constraints**: all gates use fixed offsets.
2. We have **a single column for public input** (instance) containing the SHA256 hash of the secret input.
3. We have **a single column for fixed values** (constants). 
It contains round constants (_k<sub>0</sub>, k<sub>1</sub>, ..., k<sub>63</sub>_) and initial hash words (_h<sub>0</sub>, h<sub>1</sub>, ..., h<sub>7</sub>_).
Values are **spread out** to enable convenient access from gates that are working with them.
4. We have **6 different gates** and thus 6 corresponding selector columns.
Ultimately they will be squashed into one, similarly to the [halo2 optimization](https://zcash.github.io/halo2/design/implementation/selector-combining.html).
5. Advice area (private input and intermediate values):
   - Uses 9 columns.
   - The main part consists of 64 repetitions of 4 rows that represent single SHA256 round.
   - There are 3 mocked rounds in the beginning (initialization buffer) enabling uniform gates.
   - First 16 message schedule values (_w<sub>0</sub>, w<sub>1</sub>, ..., w<sub>15</sub>_ - the input chunk of 512 bits) are the actual secret input to the circuit.
   - Further schedule values (_w<sub>16</sub>, w<sub>17</sub>, ..., w<sub>63</sub>_) are computed on demand (during the rounds they are needed).
   - Instead of explicit representing all the current hash words (_a, b, ..., h_) we are working only with _a_ and _e_, and when we need the other words, we are reusing _a_ and _e_ from previous rounds (rows above).

## Running code

Currently, the Rust package contains two crates:
- library representing witness generation (i.e. PLONKish table creation, together with gates validation),
- binary (can be run with `cargo run`) instantiating the table for the empty input and rendering the result to the markdown file

## Tools

 - https://sha256algorithm.com/
