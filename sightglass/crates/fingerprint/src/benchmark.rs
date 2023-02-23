use crate::hash;
use crate::util::to_string_lossy;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs::File, path::Path};

/// Describes a fingerprinted benchmark.
///
/// ```
/// # use sightglass_fingerprint::Benchmark;
/// let benchmark = Benchmark::fingerprint("../../benchmarks/noop/benchmark.wasm").unwrap();
/// assert_eq!(benchmark.name, "noop");
/// assert_eq!(benchmark.path, format!("benchmarks{}noop{}benchmark.wasm", std::path::MAIN_SEPARATOR, std::path::MAIN_SEPARATOR));
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Benchmark {
    /// A unique identifier for the benchmark: `<name>-<hash slug>`.
    pub id: String,
    /// The benchmark name; this is calculated from either the Wasm file name (if it does not start
    /// with "benchmark") or from the parent directory. This accommodates both the current benchmark
    /// structure where each directory contains a "benchmark.wasm" file as well as non-Sightglass
    /// benchmarks, e.g., "spidermonkey.wasm".
    pub name: String,
    /// The path to the benchmark on the system.
    pub path: String,
    /// The SHA256 hash of the benchmark file.
    pub hash: String,
    /// The size of the file; this may be useful for comparing compile times between benchmarks of
    /// different sizes.
    pub size: u64,
}

impl Benchmark {
    pub fn fingerprint<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path
            .as_ref()
            .canonicalize()
            .context("unable to get absolute path of benchmark")?;
        let name = simplify_benchmark_name(&path)?;
        let file_hash = hash::file(&path);

        // Calculate the hash for the benchmark file.
        let size = File::open(&path)
            .expect("should be able to open the benchmark file")
            .metadata()
            .expect("should be able to collect the benchmark file size")
            .len();

        Ok(Self {
            id: format!("{}-{}", name, hash::slug(&file_hash)),
            name,
            path: simplify_benchmark_path(&path),
            hash: file_hash,
            size,
        })
    }
}

/// Simplify the benchmark name if possible; e.g.:
/// - `.../<name>/benchmark.wasm` -> <name>
/// - `.../<name>.wasm -> <name>
fn simplify_benchmark_name<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    let stem = path
        .file_stem()
        .ok_or(anyhow!("the benchmark must have a file name"))?;
    let name = if stem == "benchmark" {
        path.parent()
            .ok_or(anyhow!("the benchmark must have a parent directory"))?
            .file_name()
            .ok_or(anyhow!("the parent directory to have a name"))?
    } else {
        stem
    };
    Ok(to_string_lossy(name))
}

/// Simplify the benchmark path if possible; e.g.:
/// `/home/user/code/sightglass/benchmarks/<name>/benchmark.wasm` ->
/// `benchmarks/<name>/benchmark.wasm`. This function finds a path component matching `benchmarks/`
/// and cuts the path there.
fn simplify_benchmark_path<P: AsRef<Path>>(path: P) -> String {
    let path = path.as_ref();
    if let Some(i) = path.iter().position(|c| c == "benchmarks") {
        let shortened_path: PathBuf = path.iter().skip(i).collect();
        to_string_lossy(shortened_path.as_os_str())
    } else {
        to_string_lossy(path.as_os_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortened_benchmark_names() {
        assert_eq!(
            simplify_benchmark_name("benchmarks/noop/benchmark.wasm").unwrap(),
            "noop"
        );
        assert_eq!(simplify_benchmark_name("a/b/c.wasm").unwrap(), "c");
    }

    #[test]
    fn shortened_benchmark_paths() {
        assert_eq!(
            simplify_benchmark_path("/home/user/sightglass/benchmarks/benchmark.wasm"),
            format!("benchmarks{}benchmark.wasm", std::path::MAIN_SEPARATOR)
        );
        assert_eq!(
            simplify_benchmark_path("code/benchmarks/noop.wasm"),
            format!("benchmarks{}noop.wasm", std::path::MAIN_SEPARATOR)
        );
        // Note here how `simplify_benchmark_path` does not modify the path separator.
        assert_eq!(
            simplify_benchmark_path("some/other/path.wasm"),
            "some/other/path.wasm"
        );
    }
}
