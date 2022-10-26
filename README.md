# monirs

## What monirs

monirs is execute command or rust function triggered by file change.

## How to use

If you want use monirs,you prepare moni.json like below example.

```json
{
  "workspace": "./",
  "target_extensions": ["rs", "txt"],
  "ignore_filenames": ["test.rs"],
  "ignore_path_words": ["utils"],
  "execute_command": "echo Hello World"
}
```

### Parameters

- workspace is target of monitaring root directory.
- If you set target_extensions, monirs is only monitaring these extensions file.
- If you set ignore_filenames, monirs is not monitaring containe filename file.
- If you set ignore_path_words, monirs is not monitaring containe filename file.
- execute_command is must set. This value is execute command when file change

And run rust program is below<br>

```rust
fn main() {
    MoniJsonConfig::from_file("moni.json")
        .unwrap()
        .to_instance(DefaultMoniPrinter::default())

        .monitaring()
}

```

If you want to use commands with the path of the detected file, set the moni.json as follows.

```json
{
  "workspace": "./",
  "target_extensions": ["rs", "txt"],
  "ignore_filenames": ["test.rs"],
  "ignore_path_words": ["utils"],
  "execute_command": "echo MONI_FILE_PATH"
}
```

MONI_FILE_PATH words in execute_command is dynamically assigned the path of the detected file.<br>

In the exapmle above,the console outputs the file path where the change was detected.

## How to customize

- You would generate Moni instance used by MoniBuilder.
- You customize to Moni execute rust function like below example.

```rust
fn main() {
    let exe_fn = |filepath: &str| -> Result<(), String> {
        let mut reader = BufReader::new(File::open(filepath).unwrap());
        let mut content = String::new();
        reader.read_to_string(&mut content).unwrap();
        println!("file path is \n{}\n", filepath);
        println!("file content is \n{}\n", content);
        Ok(())
    };
    MoniBuilder::new()
        .root("./")
        .ignore_re("target")
        .exe_fn(exe_fn)
        .build()
        .monitaring();
}
```
