# rust-diesel-mysql

Sample setup with mysql with rust using diesel.

## Error encountered

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.98 secs
     Running `target/debug/rust-diesel-mysql`
thread 'main' panicked at 'Error connecting to mysql://root:12345678@127.0.0.1:3306/test: BadConnection("Authentication plugin \'caching_sha2_password\' cannot be loaded: dlopen(/usr/local/Cellar/mysql/5.7.22/lib/plugin/caching_sha2_password.so, 2): image not found")', libcore/result.rs:945:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

For solution, see the `docker-compose.yml`.
