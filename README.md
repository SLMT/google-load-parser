# Google Load Parser

This program is designed to parse and transfer the resource usage files downloaded from [Google cluster-usage traces][1] into the timeline sequences of CPU usage for each machine.

## Requirements

In order to build the source, you need `cargo`, which can be downloaded via [rustup][2], the Rust toolchain installer.

## Building

Use the following command to build the program. We recommend to build the program in `RELEASE` mode, which makes the program run much faster.

```bash
> cargo build --release
```

Then, you will find the executable `google-load-parser.exe` in `target/release`.

## Usage

```txt
google-load-parser v1.1.0
SLMT <sam123456777@gmail.com>
Parse the load trace of Google's testing cluster

USAGE:
    google-load-parser.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    transfer    Transfers the files in the given directory to daily timeline files
    trim        Trims the files in the given directory to leave only necessary data
```

You can add `RUST_LOG=DEBUG` in front of the command to show debugging information.

```bash
> RUST_LOG=DEBUG google-load-parser.exe [SUBCOMMAND]
```

This program currently supports two sub commands:

- `trim`
- `transfer`

### Trimming

```txt
Trims the files in the given directory to leave only necessary data

USAGE:
    google-load-parser.exe trim <INPUT DIR> <OUTPUT DIR>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <INPUT DIR>     the directory containing input files
    <OUTPUT DIR>    the directory for placing output files
```

Since the downloaded files are compressed as `gz` files, we need to decompress the files before processing them. This command decompresses the `gz` files in the given `[INPUT DIR]` directory and trims unnecessary information from the files such that the output files in `[OUTPUT DIR]` have only the necessary information we need.

An example of the content of one of decompressed file:

```csv
5612000000,5700000000,4665712499,369,4820204869,0.03143,0.05389,0.06946,0.005997,0.006645,0.05408,7.629e-05,0.0003834,0.2415,0.002571,2.911,,0,0,0.02457
5612000000,5700000000,4665712499,369,4820204869,0.03143,0.05389,0.06946,0.005997,0.006645,0.05408,7.629e-05,0.0003834,0.2415,0.002571,2.911,,0,0,0.02457
5612000000,5700000000,4665712499,798,3349189123,0.02698,0.06714,0.07715,0.004219,0.004868,0.06726,7.915e-05,0.0003681,0.27,0.00293,3.285,0.008261,0,0,0.01608
...
```

After performing the trimming:

```csv
5612000000,5700000000,3349189123,0.02698
5612000000,5700000000,372630265,0.04114
5612000000,5700000000,1437119225,0.07275
...
```

This can greatly reduces the size of files.

### Transferring

```txt
Transfers the files in the given directory to daily timeline files

USAGE:
    google-load-parser.exe transfer <INPUT DIR> <OUTPUT DIR> [SLOT LENGTH]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <INPUT DIR>      the directory containing input files
    <OUTPUT DIR>     the directory for placing output files
    <SLOT LENGTH>    the length of time slot (in micro-seconds) [default: 60000000]
```

This command processes the trimmed files in the given `[INPUT DIR]` directory into the daily timeline files, where each row represents the changing of CPU usage of a machine in the form of timeline in a day. You can specify the length of a time slot in micro-seconds, which defines the sample rate of the timeline.

Note that the program maps machine ids from its original domain into [1~15000] in order to save the size of the files.

## License

MIT

[1]: https://github.com/google/cluster-data
[2]: https://rustup.rs/