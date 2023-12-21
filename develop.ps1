$env:RUSTFLAGS="-Awarnings";
pip uninstall python_genshin_artifact -y
maturin develop

# py -m pip install maturin
# $env:PYTHONPATH="C:\Users\luoshuijs\AppData\Local\Programs\Python\Python311"
# cargo test