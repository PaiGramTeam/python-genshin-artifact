$env:RUSTFLAGS="-Awarnings"

venv/Scripts/activate.ps1

maturin build

deactivate

$pythonPath = py -c "import sys; print(sys.exec_prefix)"
$env:PYTHONHOME = $pythonPath
$env:PYTHONPATH = $pythonPath
$Env:Path += ";$pythonPath"

py -m pip uninstall python_genshin_artifact -y

$wheelDirectory = ".\target\wheels"
$latestWheel = Get-ChildItem $wheelDirectory -Filter *.whl | Sort-Object LastWriteTime -Descending | Select-Object -First 1
$wheelPath = Join-Path $wheelDirectory $latestWheel.Name

py -m pip install $wheelPath

cargo test

py -m pip uninstall python_genshin_artifact -y
