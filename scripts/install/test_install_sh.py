#!/usr/bin/env python3

import hashlib
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import textwrap
import unittest


INSTALL_SCRIPT = Path(__file__).with_name("install.sh")
VERSION = "0.142.5"
MISMATCH_VERSION = "0.145.0"


class InstallShTest(unittest.TestCase):
    def test_metadata_fetch_failure_is_not_reported_as_missing_assets(self) -> None:
        result, requests = run_installer(VERSION, metadata_failure=True)

        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(
            requests,
            [
                "https://api.github.com/repos/openai/codewen/releases/tags/"
                f"rust-v{VERSION}"
            ],
        )
        self.assertIn(
            f"Could not fetch GitHub release metadata for Codewen {VERSION}",
            result.stderr,
        )
        self.assertNotIn("Could not find Codewen package", result.stderr)

    def test_exact_release_fetches_metadata_once(self) -> None:
        result, requests = run_installer(VERSION)

        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(
            requests,
            [
                "https://api.github.com/repos/openai/codewen/releases/tags/"
                f"rust-v{VERSION}",
                "https://github.com/openai/codewen/releases/download/"
                f"rust-v{VERSION}/codewen-package_SHA256SUMS",
            ],
        )
        self.assertIn(f"Resolved version: {VERSION}", result.stdout)

    def test_alpha_hotfix_release_is_valid(self) -> None:
        version = "0.145.0-alpha.23.1"
        result, requests = run_installer(version)

        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(
            requests,
            [
                "https://api.github.com/repos/openai/codewen/releases/tags/"
                f"rust-v{version}",
                "https://github.com/openai/codewen/releases/download/"
                f"rust-v{version}/codewen-package_SHA256SUMS",
            ],
        )
        self.assertIn(f"Resolved version: {version}", result.stdout)

    def test_latest_release_reuses_version_metadata(self) -> None:
        result, requests = run_installer("latest")

        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(
            requests,
            [
                "https://api.github.com/repos/openai/codewen/releases/latest",
                "https://github.com/openai/codewen/releases/download/"
                f"rust-v{VERSION}/codewen-package_SHA256SUMS",
            ],
        )
        self.assertIn(f"Resolved version: {VERSION}", result.stdout)

    def test_compact_metadata_is_independent_of_field_order(self) -> None:
        result, requests = run_installer(
            "latest", metadata_json=release_metadata(compact=True, reorder=True)
        )

        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(
            requests,
            [
                "https://api.github.com/repos/openai/codewen/releases/latest",
                "https://github.com/openai/codewen/releases/download/"
                f"rust-v{VERSION}/codewen-package_SHA256SUMS",
            ],
        )
        self.assertIn(f"Resolved version: {VERSION}", result.stdout)

    def test_json_like_strings_and_nested_fields_do_not_define_assets(self) -> None:
        result, requests = run_installer(
            VERSION, metadata_json=legacy_release_metadata_with_decoys()
        )

        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(len(requests), 2)
        self.assertIn("/codewen-npm-", requests[1])
        self.assertNotIn("codewen-package_SHA256SUMS", requests[1])

    def test_macos_install_exposes_code_mode_host_beside_codex(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            archive_path, checksum_path, metadata_json = create_package_release(root)

            result, _requests = run_installer_in(
                root,
                VERSION,
                metadata_json=metadata_json,
                archive_path=archive_path,
                checksum_path=checksum_path,
                force_macos=True,
            )

            self.assertEqual(result.returncode, 0, result.stderr)
            install_bin = root / "install-bin"
            current = root / "codewen-home" / "packages" / "standalone" / "current"
            codewen_path = install_bin / "codewen"
            host_path = install_bin / "codewen-code-mode-host"
            self.assertEqual(os.readlink(codewen_path), str(current / "bin" / "codewen"))
            self.assertEqual(
                os.readlink(host_path),
                str(current / "bin" / "codewen-code-mode-host"),
            )
            self.assertTrue(os.access(host_path, os.X_OK))

    def test_releases_latest_installs_verified_package(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            archive_path, checksum_path, metadata_json = create_package_release(root)

            result, requests = run_installer_in(
                root,
                "latest",
                metadata_json=metadata_json,
                archive_path=archive_path,
                checksum_path=checksum_path,
                force_macos=True,
                use_releases=True,
            )

            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(
                requests,
                [
                    "https://releases.openai.com/codex/channels/latest",
                    f"https://releases.openai.com/codex/releases/{VERSION}/codewen-package_SHA256SUMS",
                    f"https://releases.openai.com/codex/releases/{VERSION}/codewen-package-aarch64-apple-darwin.tar.gz",
                ],
            )

    def test_releases_asset_download_falls_back_to_github(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            archive_path, checksum_path, metadata_json = create_package_release(root)

            result, requests = run_installer_in(
                root,
                "latest",
                metadata_json=metadata_json,
                archive_path=archive_path,
                checksum_path=checksum_path,
                force_macos=True,
                use_releases=True,
                releases_mode="asset_fallback",
            )

            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(
                requests,
                [
                    "https://releases.openai.com/codex/channels/latest",
                    f"https://releases.openai.com/codex/releases/{VERSION}/codewen-package_SHA256SUMS",
                    "https://github.com/openai/codewen/releases/download/"
                    f"rust-v{VERSION}/codewen-package_SHA256SUMS",
                    f"https://releases.openai.com/codex/releases/{VERSION}/codewen-package-aarch64-apple-darwin.tar.gz",
                    "https://github.com/openai/codewen/releases/download/"
                    f"rust-v{VERSION}/codewen-package-aarch64-apple-darwin.tar.gz",
                ],
            )
            self.assertIn("retrying from GitHub Releases", result.stderr)

    def test_releases_exact_rejects_wrong_binary_version(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            archive_path, checksum_path, metadata_json = create_package_release(
                root,
                metadata_version=MISMATCH_VERSION,
            )

            result, requests = run_installer_in(
                root,
                MISMATCH_VERSION,
                metadata_json=metadata_json,
                archive_path=archive_path,
                checksum_path=checksum_path,
                force_macos=True,
                use_releases=True,
            )

            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(
                requests,
                [
                    f"https://releases.openai.com/codex/releases/{MISMATCH_VERSION}/release.json",
                    f"https://releases.openai.com/codex/releases/{MISMATCH_VERSION}/codewen-package_SHA256SUMS",
                    f"https://releases.openai.com/codex/releases/{MISMATCH_VERSION}/codewen-package-aarch64-apple-darwin.tar.gz",
                ],
            )
            self.assertIn(
                f"did not report expected version {MISMATCH_VERSION}",
                result.stderr,
            )
            self.assertNotIn("installed successfully", result.stdout)

    def test_releases_exact_legacy_fallback_reuses_offline_install(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            archive_path, metadata_json = create_legacy_release(root)

            first_result, first_requests = run_installer_in(
                root,
                VERSION,
                metadata_json=metadata_json,
                legacy_archive_path=archive_path,
                force_macos=True,
                use_releases=True,
                releases_mode="channel_failure",
            )

            self.assertEqual(first_result.returncode, 0, first_result.stderr)
            self.assertEqual(
                first_requests,
                [
                    f"https://releases.openai.com/codex/releases/{VERSION}/release.json",
                    "https://api.github.com/repos/openai/codewen/releases/tags/"
                    f"rust-v{VERSION}",
                    "https://github.com/openai/codewen/releases/download/"
                    f"rust-v{VERSION}/codewen-npm-darwin-arm64-{VERSION}.tgz",
                ],
            )

            (root / "requests.log").unlink()
            second_result, second_requests = run_installer_in(
                root,
                VERSION,
                metadata_json=metadata_json,
                force_macos=True,
                use_releases=True,
                releases_mode="channel_failure",
            )

            self.assertEqual(second_result.returncode, 0, second_result.stderr)
            self.assertEqual(
                second_requests,
                [
                    f"https://releases.openai.com/codex/releases/{VERSION}/release.json",
                    "https://api.github.com/repos/openai/codewen/releases/tags/"
                    f"rust-v{VERSION}",
                ],
            )
            self.assertNotIn("Downloading Codewen CLI", second_result.stdout)


def run_installer(
    release: str,
    *,
    metadata_failure: bool = False,
    metadata_json: str | None = None,
) -> tuple[subprocess.CompletedProcess[str], list[str]]:
    with tempfile.TemporaryDirectory() as temp_dir:
        return run_installer_in(
            Path(temp_dir),
            release,
            metadata_failure=metadata_failure,
            metadata_json=metadata_json,
        )


def run_installer_in(
    root: Path,
    release: str,
    *,
    metadata_failure: bool = False,
    metadata_json: str | None = None,
    archive_path: Path | None = None,
    checksum_path: Path | None = None,
    legacy_archive_path: Path | None = None,
    force_macos: bool = False,
    use_releases: bool = False,
    releases_mode: str = "",
) -> tuple[subprocess.CompletedProcess[str], list[str]]:
    bin_dir = root / "bin"
    bin_dir.mkdir(exist_ok=True)
    request_log = root / "requests.log"
    fake_curl = bin_dir / "curl"
    fake_curl.write_text(
        textwrap.dedent(
            """\
            #!/bin/sh
            url=""
            output=""
            previous=""
            for arg in "$@"; do
              case "$arg" in
                https://*) url="$arg" ;;
              esac
              if [ "$previous" = "-o" ]; then
                output="$arg"
              fi
              previous="$arg"
            done
            printf '%s\n' "$url" >>"$CODEWEN_TEST_REQUEST_LOG"

            case "$url" in
              https://api.github.com/*)
                if [ "$CODEWEN_TEST_METADATA_FAILURE" = "1" ]; then
                  echo "curl: (22) The requested URL returned error: 403" >&2
                  exit 22
                fi
                printf '%s\n' "$CODEWEN_TEST_METADATA_JSON"
                ;;
              https://releases.openai.com/codex/channels/latest|https://releases.openai.com/codex/releases/*/release.json)
                if [ "$CODEWEN_TEST_RELEASES_MODE" = "channel_failure" ]; then
                  exit 22
                fi
                printf '%s\n' "$CODEWEN_TEST_RELEASES_METADATA_JSON"
                ;;
              https://releases.openai.com/codex/releases/*/codewen-package_SHA256SUMS)
                if [ "$CODEWEN_TEST_RELEASES_MODE" = "asset_fallback" ]; then
                  exit 22
                fi
                if [ -n "$CODEWEN_TEST_CHECKSUM_PATH" ]; then
                  cp "$CODEWEN_TEST_CHECKSUM_PATH" "$output"
                else
                  exit 22
                fi
                ;;
              https://releases.openai.com/codex/releases/*/codewen-package-*.tar.gz)
                if [ "$CODEWEN_TEST_RELEASES_MODE" = "asset_fallback" ]; then
                  exit 22
                fi
                if [ -n "$CODEWEN_TEST_ARCHIVE_PATH" ]; then
                  cp "$CODEWEN_TEST_ARCHIVE_PATH" "$output"
                else
                  exit 22
                fi
                ;;
              https://github.com/openai/codewen/releases/download/*/codewen-package_SHA256SUMS)
                if [ -n "$CODEWEN_TEST_CHECKSUM_PATH" ]; then
                  cp "$CODEWEN_TEST_CHECKSUM_PATH" "$output"
                else
                  exit 22
                fi
                ;;
              https://github.com/openai/codewen/releases/download/*/codewen-package-*.tar.gz)
                if [ -n "$CODEWEN_TEST_ARCHIVE_PATH" ]; then
                  cp "$CODEWEN_TEST_ARCHIVE_PATH" "$output"
                else
                  exit 22
                fi
                ;;
              https://github.com/openai/codewen/releases/download/*/codewen-npm-*.tgz)
                if [ -n "$CODEWEN_TEST_LEGACY_ARCHIVE_PATH" ]; then
                  cp "$CODEWEN_TEST_LEGACY_ARCHIVE_PATH" "$output"
                else
                  exit 22
                fi
                ;;
              *)
                exit 22
                ;;
            esac
            """
        ),
        encoding="utf-8",
    )
    fake_curl.chmod(0o755)
    if force_macos:
        fake_uname = bin_dir / "uname"
        fake_uname.write_text(
            "#!/bin/sh\n"
            'case "$1" in\n'
            "  -s) printf 'Darwin\\n' ;;\n"
            "  -m) printf 'arm64\\n' ;;\n"
            "esac\n",
            encoding="utf-8",
        )
        fake_uname.chmod(0o755)

    home = root / "home"
    home.mkdir(exist_ok=True)
    env = os.environ.copy()
    env.update(
        {
            "CODEWEN_HOME": str(root / "codewen-home"),
            "CODEWEN_INSTALL_DIR": str(root / "install-bin"),
            "CODEWEN_NON_INTERACTIVE": "1",
            "CODEWEN_RELEASE": release,
            "CODEWEN_INSTALLER_USE_RELEASES_OPENAI_COM": (
                "TRUE" if use_releases else "0"
            ),
            "CODEWEN_TEST_ARCHIVE_PATH": str(archive_path or ""),
            "CODEWEN_TEST_CHECKSUM_PATH": str(checksum_path or ""),
            "CODEWEN_TEST_LEGACY_ARCHIVE_PATH": str(legacy_archive_path or ""),
            "CODEWEN_TEST_METADATA_FAILURE": "1" if metadata_failure else "0",
            "CODEWEN_TEST_METADATA_JSON": (
                metadata_json if metadata_json is not None else release_metadata()
            ),
            "CODEWEN_TEST_RELEASES_METADATA_JSON": (
                metadata_json if metadata_json is not None else release_metadata()
            ),
            "CODEWEN_TEST_RELEASES_MODE": releases_mode,
            "CODEWEN_TEST_REQUEST_LOG": str(request_log),
            "HOME": str(home),
            "PATH": f"{bin_dir}:/usr/bin:/bin",
            "SHELL": "/bin/sh",
        }
    )
    result = subprocess.run(
        ["/bin/sh", str(INSTALL_SCRIPT)],
        capture_output=True,
        check=False,
        env=env,
        text=True,
    )
    requests = (
        request_log.read_text(encoding="utf-8").splitlines()
        if request_log.exists()
        else []
    )
    return result, requests


def create_package_release(
    root: Path,
    *,
    metadata_version: str = VERSION,
) -> tuple[Path, Path, str]:
    package_dir = root / "package"
    (package_dir / "bin").mkdir(parents=True)
    (package_dir / "codewen-path").mkdir()
    (package_dir / "codewen-package.json").write_text("{}\n", encoding="utf-8")
    write_executable(
        package_dir / "bin" / "codewen",
        f"#!/bin/sh\nprintf 'codewen-cli {VERSION}\\n'\n",
    )
    write_executable(
        package_dir / "bin" / "codewen-code-mode-host",
        "#!/bin/sh\nexit 0\n",
    )
    write_executable(package_dir / "codewen-path" / "rg", "#!/bin/sh\nexit 0\n")

    asset = "codewen-package-aarch64-apple-darwin.tar.gz"
    archive_path = root / asset
    with tarfile.open(archive_path, "w:gz") as archive:
        for path in package_dir.iterdir():
            archive.add(path, arcname=path.name)

    archive_digest = hashlib.sha256(archive_path.read_bytes()).hexdigest()
    checksum_path = root / "codewen-package_SHA256SUMS"
    checksum_path.write_text(f"{archive_digest}  {asset}\n", encoding="utf-8")
    checksum_digest = hashlib.sha256(checksum_path.read_bytes()).hexdigest()
    metadata_json = json.dumps(
        {
            "assets": [
                {"name": asset, "digest": f"sha256:{archive_digest}"},
                {
                    "name": "codewen-package_SHA256SUMS",
                    "digest": f"sha256:{checksum_digest}",
                },
            ],
            "tag_name": f"rust-v{metadata_version}",
        },
        indent=2,
    )
    return archive_path, checksum_path, metadata_json


def create_legacy_release(root: Path) -> tuple[Path, str]:
    package_dir = root / "legacy-package"
    vendor_dir = package_dir / "package" / "vendor" / "aarch64-apple-darwin"
    (vendor_dir / "codewen").mkdir(parents=True)
    (vendor_dir / "path").mkdir()
    write_executable(
        vendor_dir / "codewen" / "codewen",
        f"#!/bin/sh\nprintf 'codewen-cli {VERSION}\\n'\n",
    )
    write_executable(vendor_dir / "path" / "rg", "#!/bin/sh\nexit 0\n")

    asset = f"codewen-npm-darwin-arm64-{VERSION}.tgz"
    archive_path = root / asset
    with tarfile.open(archive_path, "w:gz") as archive:
        archive.add(package_dir / "package", arcname="package")

    archive_digest = hashlib.sha256(archive_path.read_bytes()).hexdigest()
    metadata_json = json.dumps(
        {
            "assets": [{"name": asset, "digest": f"sha256:{archive_digest}"}],
            "tag_name": f"rust-v{VERSION}",
        },
        indent=2,
    )
    return archive_path, metadata_json


def write_executable(path: Path, contents: str) -> None:
    path.write_text(contents, encoding="utf-8")
    path.chmod(0o755)


def release_metadata(*, compact: bool = False, reorder: bool = False) -> str:
    assets = [
        asset_metadata(
            f"codewen-package-{target}.tar.gz",
            f"sha256:{'a' * 64}",
            reorder=reorder,
        )
        for target in (
            "aarch64-apple-darwin",
            "x86_64-apple-darwin",
            "aarch64-unknown-linux-musl",
            "x86_64-unknown-linux-musl",
        )
    ]
    assets.append(
        asset_metadata(
            "codewen-package_SHA256SUMS",
            f"sha256:{'b' * 64}",
            reorder=reorder,
        )
    )
    separators = (",", ":") if compact else None
    return json.dumps(
        {"assets": assets, "body": "braces: { } [ ]", "tag_name": f"rust-v{VERSION}"},
        indent=None if compact else 2,
        separators=separators,
    )


def asset_metadata(name: str, digest: str, *, reorder: bool) -> dict[str, str]:
    if reorder:
        return {"digest": digest, "name": name}
    return {"name": name, "digest": digest}


def legacy_release_metadata_with_decoys() -> str:
    fake_digest = f"sha256:{'0' * 64}"
    assets = [
        {
            "metadata": {
                "name": "codewen-package-x86_64-unknown-linux-musl.tar.gz",
                "digest": fake_digest,
            },
            "digest": f"sha256:{'c' * 64}",
            "name": f"codewen-npm-{target}-{VERSION}.tgz",
        }
        for target in ("darwin-arm64", "darwin-x64", "linux-arm64", "linux-x64")
    ]
    return json.dumps(
        {
            "body": (
                f'fake: {{"name":"codewen-package_SHA256SUMS","digest":"{fake_digest}"}}'
            ),
            "assets": assets,
            "tag_name": f"rust-v{VERSION}",
        },
        separators=(",", ":"),
    )


if __name__ == "__main__":
    unittest.main()
