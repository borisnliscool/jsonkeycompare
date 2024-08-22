## JsonKeyCompare

A very simple program that compares two or more JSON files and reports any keys that are present in the first file but
not in any of the other files.

#### Usage:

```shell
jsonkeycompare <main_file> <other_file_1> <other_file_2> ... [--fail] [--sort]
```

#### Options:

```
--fail
    Exit with non-zero status if any files are missing keys.
```

```
--sort
    Sort the output alphabetically.
```

#### Tip:

You can use wildcard expansion (at least on Linux) to compare multiple files easily:

```shell
jsonkeycompare main.json *.json
```