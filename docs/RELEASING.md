# Releasing FM27 Training Assistant

## 1. Tag and trigger GitHub Actions

1. Bump the version in `app/Cargo.toml` and `agent/Cargo.toml` and commit your changes on `main`.
2. Create and push the release tag to trigger `.github/workflows/release.yml`:

```bash
git tag vX.Y.Z
git push public-releases main
git push public-releases vX.Y.Z
```

GitHub Actions builds:
- `FM27-Training-Assistant-Windows.zip`
- `FM27-Training-Assistant-macOS.dmg`
- `SHA256SUMS.txt` (combined from Windows `Get-FileHash` and macOS `shasum -a 256`)

> **Note:** `FM27Trainer.Bridge.dll` links against local Football Manager 27 + BepInEx 6 IL2CPP assemblies (`BepInEx/core/*.dll` and `BepInEx/interop/*.dll`), which are not available on GitHub-hosted runners. Build and attach the plugin DLL manually after the tag release is created.

## 2. Build and upload `FM27Trainer.Bridge.dll` locally

On a Windows machine with FM27 and BepInEx 6 IL2CPP installed:

```powershell
dotnet build bridge/FM27Trainer.Bridge.csproj -c Release -p:FM27Dir="PATH_TO_FM27"
gh release upload vX.Y.Z bridge/bin/Release/net6.0/FM27Trainer.Bridge.dll --clobber --repo cilek14/fm27-trainer
```
