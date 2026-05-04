# SEDU
SEDU is a silly data parsing lang i made in Rust for err.. Fun!!!!

## Usage (Python)

```python
import sedu

print(sedu.run_sedu("""
a: hello
print: $a
"""))
```

## Output
```['hello']```

## Building
You will need Cargo and pip.
```python
pip install maturin
```
```python
maturin build --release
```
