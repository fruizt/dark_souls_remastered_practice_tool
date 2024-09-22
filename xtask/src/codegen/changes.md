As i could not make to work the function `codegen_base_addresses` as it ask for a env that supposedly addds different patches versions
i decided to change the function in the package `practice_tool_tasks`, the original function is in the code commented

please replace with

```rust
pub fn codegen_base_addresses(
    codegen_path: PathBuf,
    patches_paths: impl Iterator<Item = PathBuf>,
    aobs: &[Box<dyn Aob>],
) {
    let mut processed_versions: HashSet<Version> = HashSet::new();

    let version = Version(0 as u32, 0 as u32, 0 as u32);
    let version_data = vec![VersionData { version, aobs }]
    // let mut version_data = patches_paths
    //     .filter(|p| p.exists())
    //     .filter_map(|exe| {
    //         let file_map = FileMap::open(&exe).unwrap();
    //         let pe_file = PeFile::from_bytes(&file_map).unwrap();

    //         let version = pe_file
    //             .resources()
    //             .unwrap()
    //             .version_info()
    //             .unwrap()
    //             .fixed()
    //             .unwrap()
    //             .dwProductVersion;
    //         let version = Version(0 as u32, 0 as u32, 0 as u32);

    //         if processed_versions.contains(&version) {
    //             None
    //         } else {
    //             let exe = exe.canonicalize().unwrap();
    //             println!("\nVERSION {}: {:?}", version.to_fromsoft_string(), exe);

    //             let aobs = find_aobs(&pe_file, aobs);
    //             processed_versions.insert(version);
    //             Some(VersionData { version, aobs })
    //         }
    //     })
    //     .collect::<Vec<_>>();

    version_data.sort_by_key(|vd| vd.version);

    let mut codegen = codegen_base_addresses_struct(aobs);
    codegen.push_str(&codegen_version_enum(&version_data));

    let codegen = version_data.iter().fold(codegen, |mut o, i| {
        o.push_str(&codegen_base_addresses_instances(&i.version, &i.aobs));
        o
    });

    File::create(codegen_path).unwrap().write_all(codegen.as_bytes()).unwrap();
}
```
