# pep610

[PEP 610][pep610] specifies how the Direct URL Origin of installed distributions should be recorded.

The up-to-date, [canonical specification][pep610-pypa] is maintained on the [PyPA specs page][pypa-specs].

[pep610]: https://www.python.org/dev/peps/pep-0610/
[pep610-pypa]: https://packaging.python.org/en/latest/specifications/direct-url/#direct-url
[pypa-specs]: https://packaging.python.org/en/latest/specifications/

## Usage

```rust
fn print_url_info(contents: DirectURL) {
    println!("url: {:?}", contents.url.to_string());
    match contents.info {
        Info::Dir(dir_info) => {
            println!("  * editable: {:?}", dir_info.is_editable());
        }
        Info::Archive(archive_info) => {
            println!("  * hash: {:?}", archive_info.hash);
        }
        Info::VCS(vcs_info) => {
            println!("  * vcs: {:?}", vcs_info.vcs);
            println!("  * commit_id: {:?}", vcs_info.commit_id);
            println!("  * requested_revision: {:?}", vcs_info.requested_revision);
            println!("  * resolved_revision: {:?}", vcs_info.resolved_revision);
            println!(
                "  * resolved_revision_type: {:?}",
                vcs_info.resolved_revision_type
            );
        }
    }
}
```
