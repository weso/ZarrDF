## Benchmarks

To enable performance profiling without running the benchmarks as root, you may need to adjust the value of `perf_event_paranoid` in the Linux kernel to an appropriate value for your environment.
The most permissive value is -1.

```sh
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
```

For running the benchmarks using the `FlamegraphProfiler` for 5 seconds, run the following command.

```sh
cargo bench --bench <name_of_the_benchmark> -- --profile-time=5
```

We will now find a file called `flamegraph.svg` in `target/criterion/<name-of-benchmark>/profile/flamegraph.svg`.
