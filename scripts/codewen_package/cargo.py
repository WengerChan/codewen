"""Cargo builds for source-built Codewen package artifacts."""

import os
import subprocess
from dataclasses import dataclass
from pathlib import Path

from .targets import REPO_ROOT
from .targets import PackageVariant
from .targets import TargetSpec
from .v8 import resolve_codewen_v8_cargo_env


CODEWEN_RS_ROOT = REPO_ROOT / "codewen-rs"


@dataclass(frozen=True)
class SourceBuildOutputs:
    entrypoint_bin: Path
    code_mode_host_bin: Path
    bwrap_bin: Path | None
    codewen_command_runner_bin: Path | None
    codewen_windows_sandbox_setup_bin: Path | None


def build_source_binaries(
    spec: TargetSpec,
    variant: PackageVariant,
    *,
    cargo: str,
    profile: str,
    entrypoint_bin: Path | None,
    code_mode_host_bin: Path | None,
    bwrap_bin: Path | None,
    codewen_command_runner_bin: Path | None,
    codewen_windows_sandbox_setup_bin: Path | None,
) -> SourceBuildOutputs:
    validate_prebuilt_resource_inputs(
        spec,
        bwrap_bin=bwrap_bin,
        codewen_command_runner_bin=codewen_command_runner_bin,
        codewen_windows_sandbox_setup_bin=codewen_windows_sandbox_setup_bin,
    )
    binaries = source_binaries_for_target(
        spec,
        variant,
        build_entrypoint=entrypoint_bin is None,
        build_code_mode_host=code_mode_host_bin is None,
        build_bwrap=spec.is_linux and bwrap_bin is None,
        build_codewen_command_runner=spec.is_windows and codewen_command_runner_bin is None,
        build_codewen_windows_sandbox_setup=spec.is_windows
        and codewen_windows_sandbox_setup_bin is None,
    )
    if binaries:
        cmd = [
            cargo,
            "build",
            "--target",
            spec.target,
            "--profile",
            profile,
        ]
        for binary in binaries:
            cmd.extend(["--bin", binary])

        cargo_env = None
        if entrypoint_bin is None or code_mode_host_bin is None:
            codewen_v8_env = resolve_codewen_v8_cargo_env(spec)
            if codewen_v8_env:
                cargo_env = {**os.environ, **codewen_v8_env}

        print("+", " ".join(cmd))
        subprocess.run(
            cmd,
            cwd=CODEWEN_RS_ROOT,
            check=True,
            env=cargo_env,
        )

    output_dir = cargo_profile_output_dir(spec, profile)
    outputs = SourceBuildOutputs(
        entrypoint_bin=resolve_output_path(
            entrypoint_bin,
            output_dir / variant.entrypoint_name(spec),
        ),
        code_mode_host_bin=(
            code_mode_host_bin.resolve()
            if code_mode_host_bin is not None
            else output_dir / f"codewen-code-mode-host{spec.exe_suffix}"
        ),
        bwrap_bin=resolve_output_path(
            bwrap_bin,
            output_dir / "bwrap" if spec.is_linux else None,
        ),
        codewen_command_runner_bin=resolve_output_path(
            codewen_command_runner_bin,
            output_dir / "codewen-command-runner.exe" if spec.is_windows else None,
        ),
        codewen_windows_sandbox_setup_bin=resolve_output_path(
            codewen_windows_sandbox_setup_bin,
            output_dir / "codewen-windows-sandbox-setup.exe" if spec.is_windows else None,
        ),
    )
    validate_source_outputs(outputs)
    return outputs


def source_binaries_for_target(
    spec: TargetSpec,
    variant: PackageVariant,
    *,
    build_entrypoint: bool,
    build_code_mode_host: bool,
    build_bwrap: bool,
    build_codewen_command_runner: bool,
    build_codewen_windows_sandbox_setup: bool,
) -> list[str]:
    binaries = []
    if build_entrypoint:
        binaries.append(variant.cargo_bin)
    if build_code_mode_host:
        binaries.append("codewen-code-mode-host")
    if build_bwrap:
        binaries.append("bwrap")
    if build_codewen_command_runner:
        binaries.append("codewen-command-runner")
    if build_codewen_windows_sandbox_setup:
        binaries.append("codewen-windows-sandbox-setup")
    return binaries


def validate_prebuilt_resource_inputs(
    spec: TargetSpec,
    *,
    bwrap_bin: Path | None,
    codewen_command_runner_bin: Path | None,
    codewen_windows_sandbox_setup_bin: Path | None,
) -> None:
    if bwrap_bin is not None and not spec.is_linux:
        raise RuntimeError("--bwrap-bin is only supported for Linux targets.")
    if codewen_command_runner_bin is not None and not spec.is_windows:
        raise RuntimeError(
            "--codewen-command-runner-bin is only supported for Windows targets."
        )
    if codewen_windows_sandbox_setup_bin is not None and not spec.is_windows:
        raise RuntimeError(
            "--codewen-windows-sandbox-setup-bin is only supported for Windows targets."
        )


def resolve_output_path(
    explicit_path: Path | None, default_path: Path | None
) -> Path | None:
    if explicit_path is not None:
        return explicit_path.resolve()

    return default_path


def cargo_profile_output_dir(spec: TargetSpec, profile: str) -> Path:
    target_dir = cargo_target_dir()
    return target_dir / spec.target / cargo_profile_dirname(profile)


def cargo_target_dir() -> Path:
    target_dir = os.environ.get("CARGO_TARGET_DIR")
    if target_dir is None:
        return CODEWEN_RS_ROOT / "target"

    path = Path(target_dir)
    if path.is_absolute():
        return path

    return CODEWEN_RS_ROOT / path


def cargo_profile_dirname(profile: str) -> str:
    if profile == "dev":
        return "debug"
    if profile == "release":
        return "release"
    return profile


def validate_source_outputs(outputs: SourceBuildOutputs) -> None:
    for path in [
        outputs.entrypoint_bin,
        outputs.code_mode_host_bin,
        outputs.bwrap_bin,
        outputs.codewen_command_runner_bin,
        outputs.codewen_windows_sandbox_setup_bin,
    ]:
        if path is not None and not path.is_file():
            raise RuntimeError(f"cargo build did not produce expected binary: {path}")
