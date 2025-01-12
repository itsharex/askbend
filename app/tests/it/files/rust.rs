// Copyright 2023 Databend Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use askbend::FileOperator;
use askbend::Parse;
use askbend::RustCode;

#[test]
pub fn test_rust_files() -> Result<()> {
    let file = FileOperator::create("tests/testdata/", "rs", &[]);
    let metas = file.list()?;

    let rusts = RustCode::parse_multiple(&[metas[0].full_path.clone()])?;
    for code in &rusts.snippet_files {
        assert_eq!(code.file_path, "tests/testdata/rust.rs");
        for snippet in &code.code_snippets {
            println!("--{:?}", snippet);
        }
    }

    Ok(())
}
