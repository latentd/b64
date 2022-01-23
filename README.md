[![Test](https://github.com/latentd/b64/actions/workflows/test.yml/badge.svg)](https://github.com/latentd/b64/actions?query=workflow%3ATest)

# b64

b64 is a simple util to encode/decode base64 texts.

## Installation

Download a release binary from [Releases](https://github.com/latentd/b64/releases).

## Usage

### Encode

```
$ b64 foobar
Zm9vYmFy

$ echo -n "foobar" | b64
Zm9vYmFy
```

### Decode

```
$ b64 -d Zm9vYmFy 
foobar

$ echo -n "Zm9vYmFy" | b64 -d
foobar
```

## License

Apache License Version 2.0
