use super::agent;
use codewen_model_provider::create_model_provider;
use codewen_protocol::models::ManagedFileSystemPermissions;
use codewen_protocol::models::PermissionProfile;
use codewen_protocol::permissions::NetworkSandboxPolicy;
use codewen_protocol::protocol::SandboxPolicy;
use core_test_support::responses::start_mock_server;
use core_test_support::test_codewen::test_codewen;
use pretty_assertions::assert_eq;
use std::sync::Arc;
use tempfile::TempDir;

#[tokio::test]
async fn consolidation_uses_canonical_parent_enforcement() -> anyhow::Result<()> {
    let server = start_mock_server().await;
    let home = Arc::new(TempDir::new()?);
    let test = test_codewen()
        .with_home(home)
        .build_with_auto_env(&server)
        .await?;
    let provider = create_model_provider(
        test.config.model_provider.clone(),
        Some(test.thread_manager.auth_manager()),
    );

    let root = crate::memory_root(&test.config.codewen_home);
    let managed_worker_policy = SandboxPolicy::WorkspaceWrite {
        writable_roots: vec![root.clone()],
        network_access: false,
        exclude_tmpdir_env_var: true,
        exclude_slash_tmp: true,
    };

    for (parent_permission_profile, expected_permission_profile) in [
        (PermissionProfile::Disabled, PermissionProfile::Disabled),
        (
            PermissionProfile::External {
                network: NetworkSandboxPolicy::Restricted,
            },
            PermissionProfile::External {
                network: NetworkSandboxPolicy::Restricted,
            },
        ),
        (
            PermissionProfile::Managed {
                file_system: ManagedFileSystemPermissions::Unrestricted,
                network: NetworkSandboxPolicy::Enabled,
            },
            PermissionProfile::from_legacy_sandbox_policy_for_cwd(
                &managed_worker_policy,
                root.as_path(),
            ),
        ),
    ] {
        let agent_config =
            agent::get_config(&test.config, parent_permission_profile, provider.as_ref())
                .expect("agent config should be created");

        assert_eq!(
            agent_config.permissions.permission_profile(),
            &expected_permission_profile
        );
    }

    test.codewen.shutdown_and_wait().await?;
    Ok(())
}
