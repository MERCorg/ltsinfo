# Overview

This repository contains a command-line tool for performing signature-based branching bisimulation reduction on Labeled Transition Systems (LTS) in the [AUT](https://cadp.inria.fr/man/aut.html) format. The tool is written in the [Rust](https://rust-lang.org/) programming language, whose installation instructions can be found [here](https://rust-lang.org/tools/install/). The algorithms are described in the paper:

  > Jan J.M. Martens and Maurice Laveaux. Faster Signature Refinement for Branching Bisimilarity Minimization. TACAS 2026.

The tool can be compiled and ran using the following command:

```bash
    cargo run --release --bin ltsinfo -- reduce <reduction_type> <INPUT_FILE> <OUTFILE_FILE>
```

Where `<INPUT_FILE>` is the path to the input LTS file. Use the `--help` flag to see all available options. Dependencies are automatically downloaded by `cargo`.

This tool is only extracted to be stable with respect to the paper and its corresponding [artifact](https://github.com/MERCorg/ltsinfo-artifact). However, development of the tool itself is continued in the [MERC](https://github.com/MERCorg/merc) repository under the name `merc-lts`. The latest version can read and write more file formats, has conversion utilities between formats, and has more reduction algorithms implemented. For the latest version, please refer to that repository.