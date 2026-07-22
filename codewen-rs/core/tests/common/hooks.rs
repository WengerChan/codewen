use codewen_config::CONFIG_TOML_FILE;
use codewen_config::ConfigLayerStack;
use codewen_config::TomlValue;
use codewen_core::config::Config;
use codewen_features::Feature;
use codewen_hooks::HookListEntry;
use codewen_utils_absolute_path::AbsolutePathBuf;

pub fn trust_discovered_hooks(config: &mut Config) {
    config
        .features
        .enable(Feature::CodewenHooks)
        .expect("test config should allow feature update");

    let listed = codewen_hooks::list_hooks(codewen_hooks::HooksConfig {
        feature_enabled: true,
        config_layer_stack: Some(config.config_layer_stack.clone()),
        ..codewen_hooks::HooksConfig::default()
    });
    assert!(
        !listed.hooks.is_empty(),
        "trusted hook fixture should discover at least one hook"
    );
    trust_hooks(config, listed.hooks);
}

pub fn trust_hooks(config: &mut Config, hooks: Vec<HookListEntry>) {
    config.config_layer_stack =
        trusted_config_layer_stack(&config.config_layer_stack, &config.codewen_home, hooks);
}

pub fn trusted_config_layer_stack(
    config_layer_stack: &ConfigLayerStack,
    codewen_home: &AbsolutePathBuf,
    hooks: Vec<HookListEntry>,
) -> ConfigLayerStack {
    let mut user_config = config_layer_stack
        .get_active_user_layer()
        .map(|layer| layer.config.clone())
        .unwrap_or_else(|| TomlValue::Table(Default::default()));
    let user_table = user_config
        .as_table_mut()
        .expect("user config should be a table");
    let hooks_table = user_table
        .entry("hooks")
        .or_insert_with(|| TomlValue::Table(Default::default()))
        .as_table_mut()
        .expect("hooks config should be a table");
    let state_table = hooks_table
        .entry("state")
        .or_insert_with(|| TomlValue::Table(Default::default()))
        .as_table_mut()
        .expect("hook state config should be a table");
    for hook in hooks {
        let mut hook_state = TomlValue::Table(Default::default());
        let hook_state_table = hook_state
            .as_table_mut()
            .expect("hook state should be a table");
        hook_state_table.insert(
            "trusted_hash".to_string(),
            TomlValue::String(hook.current_hash),
        );
        state_table.insert(hook.key, hook_state);
    }

    config_layer_stack
        .with_user_config(&codewen_home.join(CONFIG_TOML_FILE), user_config)
        .expect("hook user config should be valid")
}
