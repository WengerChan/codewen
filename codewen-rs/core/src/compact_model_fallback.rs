use codewen_analytics::CompactionImplementation;
use codewen_analytics::CompactionReason;
use codewen_otel::SessionTelemetry;
use codewen_protocol::error::CodewenErr;
use tracing::warn;

/// Retries failures that may be model-specific and succeed with a different model.
pub(crate) fn should_retry_with_current_model(error: &CodewenErr) -> bool {
    matches!(
        error,
        CodewenErr::InvalidRequest(_)
            | CodewenErr::UnexpectedStatus(_)
            | CodewenErr::ContextWindowExceeded
            | CodewenErr::UsageLimitReached(_)
            | CodewenErr::ServerOverloaded
            | CodewenErr::InternalServerError
            | CodewenErr::RetryLimit(_)
    )
}

pub(crate) fn record_model_fallback(
    session_telemetry: &SessionTelemetry,
    previous_model: &str,
    current_model: &str,
    reason: CompactionReason,
    implementation: CompactionImplementation,
    fallback_error: Option<&CodewenErr>,
) {
    let reason_tag = match reason {
        CompactionReason::UserRequested => "user_requested",
        CompactionReason::ContextLimit => "context_limit",
        CompactionReason::ModelDownshift => "model_downshift",
        CompactionReason::CompHashChanged => "comp_hash_changed",
    };
    let implementation_tag = match implementation {
        CompactionImplementation::Responses => "responses",
        CompactionImplementation::ResponsesCompactionV2 => "responses_compaction_v2",
        CompactionImplementation::ResponsesCompact => "responses_compact",
    };
    let outcome = if fallback_error.is_none() {
        "succeeded"
    } else {
        "failed"
    };
    session_telemetry.counter(
        "codewen.compaction.model_fallback",
        /*inc*/ 1,
        &[
            ("reason", reason_tag),
            ("implementation", implementation_tag),
            ("outcome", outcome),
        ],
    );
    warn!(
        previous_model,
        current_model,
        ?reason,
        ?implementation,
        outcome,
        ?fallback_error,
        "previous-model compaction failed; retried with current model"
    );
}
