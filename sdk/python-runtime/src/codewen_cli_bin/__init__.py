import os
from pathlib import Path

PACKAGE_NAME = "openai-codewen-cli-bin"
PACKAGE_METADATA_FILENAME = "codewen-package.json"


def bundled_package_dir() -> Path:
    path = Path(__file__).resolve().parent
    metadata_path = path / PACKAGE_METADATA_FILENAME
    if not metadata_path.is_file():
        raise FileNotFoundError(
            f"{PACKAGE_NAME} is installed but missing its package metadata at {metadata_path}"
        )
    return path


def bundled_codewen_path() -> Path:
    exe = "codewen.exe" if os.name == "nt" else "codewen"
    path = bundled_package_dir() / "bin" / exe
    if not path.is_file():
        raise FileNotFoundError(
            f"{PACKAGE_NAME} is installed but missing its packaged codewen binary at {path}"
        )
    return path


def bundled_path_dir() -> Path | None:
    path = bundled_package_dir() / "codewen-path"
    return path if path.is_dir() else None


__all__ = [
    "PACKAGE_NAME",
    "bundled_codewen_path",
    "bundled_package_dir",
    "bundled_path_dir",
]
