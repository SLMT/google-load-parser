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

```bash
> google-load-parser.exe [Sub Command] [Input Dir] [Output Dir]
```

You can also use the following command to show debugging information.

```bash
> RUST_LOG=DEBUG google-load-parser.exe [Sub Command] [Input Dir] [Output Dir]
```

This program currently supports two sub commands:

- `trim`
- `transfer`

### Trimming

Since the downloaded files are compressed as `gz` files, we need to decompress the files before processing them. This command decompresses the `gz` files in the given `[Input Dir]` directory and trims unnecessary information from the files such that the output files have only the necessary information we need.

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

This command processes the trimmed files in the given `[Input Dir]` directory into the timeline files, where each row represents the changing of CPU usage of a machine in the form of timeline.

Note that the program maps machine ids from its original domain into [1~15000] in order to save the size of the files.

## License

MIT

[1]: https://github.com/google/cluster-data
[2]: https://rustup.rs/