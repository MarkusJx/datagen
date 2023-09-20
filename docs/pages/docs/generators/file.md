# file

The `file` generator reads a JSON array from inside a file and outputs
a random item from that array as a value.

The generator takes the following arguments:

- `path`: The path to the file to read. This can be either absolute or
  relative to the current working directory.
- `mode`: Whether to return random items from the array (`random`) or
  to return items in order (`sequential`). Defaults to `random`.

## Example

Assuming the following file exists at `data/words.json`:

```json
["hello", "world"]
```

And your schema looks like this:

```json
{
  "type": "file",
  "path": "data/words.json",
  "mode": "sequential"
}
```

### Output

```json
"hello"
```

The next iteration will return `"world"`.
