use leptos_motion_core::{
    AnimationError, AnimationHandle, DefaultErrorHandler, ErrorContext, ErrorHandler,
    RecoveryStrategy,
};

#[test]
fn test_error_context_creation() {
    let context = ErrorContext::new("test_operation");

    assert_eq!(context.operation, "test_operation");
    assert!(context.component.is_none());
    assert_eq!(context.additional_info.len(), 0);
}

#[test]
fn test_error_context_with_component() {
    let context = ErrorContext::new("test_operation").with_component("test_component");

    assert_eq!(context.operation, "test_operation");
    assert_eq!(context.component, Some("test_component".to_string()));
}

#[test]
fn test_error_context_with_info() {
    let context = ErrorContext::new("test_operation")
        .with_info("key1", "value1")
        .with_info("key2", "value2");

    assert_eq!(context.operation, "test_operation");
    assert_eq!(
        context.additional_info.get("key1"),
        Some(&"value1".to_string())
    );
    assert_eq!(
        context.additional_info.get("key2"),
        Some(&"value2".to_string())
    );
}

#[test]
fn test_recovery_strategy_ordering() {
    // RecoveryStrategy doesn't implement PartialOrd, so we can't compare them
    // Just test that they can be created and compared for equality
    let strategy1 = RecoveryStrategy::Retry;
    let strategy2 = RecoveryStrategy::Retry;
    let strategy3 = RecoveryStrategy::Fallback;

    assert_eq!(strategy1, strategy2);
    assert_ne!(strategy1, strategy3);
}

#[test]
fn test_default_error_handler_creation() {
    let handler = DefaultErrorHandler::default();

    assert!(handler.log_errors);
    assert!(!handler.show_user_messages);
}

#[test]
fn test_default_error_handler_custom() {
    let handler = DefaultErrorHandler {
        log_errors: false,
        show_user_messages: true,
    };

    assert!(!handler.log_errors);
    assert!(handler.show_user_messages);
}

#[test]
fn test_error_handler_engine_unavailable() {
    let handler = DefaultErrorHandler::default();
    let error = AnimationError::EngineUnavailable("test".to_string());
    let context = ErrorContext::new("test");

    let strategy = handler.handle_error(&error, &context);
    assert_eq!(strategy, RecoveryStrategy::Fallback);

    let message = handler.user_message(&error);
    assert_eq!(message, "Animation system temporarily unavailable");
}

#[test]
fn test_error_handler_invalid_property() {
    let handler = DefaultErrorHandler::default();
    let error = AnimationError::InvalidProperty {
        property: "invalid_prop".to_string(),
    };
    let context = ErrorContext::new("test");

    let strategy = handler.handle_error(&error, &context);
    assert_eq!(strategy, RecoveryStrategy::Skip);

    let message = handler.user_message(&error);
    assert_eq!(message, "Invalid animation property");
}

#[test]
fn test_error_handler_already_running() {
    let handler = DefaultErrorHandler::default();
    let error = AnimationError::AlreadyRunning {
        handle: AnimationHandle(1),
    };
    let context = ErrorContext::new("test");

    let strategy = handler.handle_error(&error, &context);
    assert_eq!(strategy, RecoveryStrategy::Skip);

    let message = handler.user_message(&error);
    assert_eq!(message, "Animation already in progress");
}

#[test]
fn test_error_handler_not_found() {
    let handler = DefaultErrorHandler::default();
    let error = AnimationError::NotFound {
        handle: AnimationHandle(1),
    };
    let context = ErrorContext::new("test");

    let strategy = handler.handle_error(&error, &context);
    assert_eq!(strategy, RecoveryStrategy::Abort);

    let message = handler.user_message(&error);
    assert_eq!(message, "Animation not found");
}

#[test]
fn test_error_handler_dom_error() {
    let handler = DefaultErrorHandler::default();
    let error = AnimationError::DomError("DOM operation failed".to_string());
    let context = ErrorContext::new("test");

    let strategy = handler.handle_error(&error, &context);
    assert_eq!(strategy, RecoveryStrategy::Retry);

    let message = handler.user_message(&error);
    assert_eq!(message, "Animation display error");
}

#[test]
fn test_error_handler_math_error() {
    let handler = DefaultErrorHandler::default();
    let error = AnimationError::MathError("Division by zero".to_string());
    let context = ErrorContext::new("test");

    let strategy = handler.handle_error(&error, &context);
    assert_eq!(strategy, RecoveryStrategy::Skip);

    let message = handler.user_message(&error);
    assert_eq!(message, "Animation calculation error");
}

#[test]
fn test_error_handler_performance_budget_exceeded() {
    let handler = DefaultErrorHandler::default();
    let error = AnimationError::PerformanceBudgetExceeded("Frame budget exceeded".to_string());
    let context = ErrorContext::new("test");

    let strategy = handler.handle_error(&error, &context);
    assert_eq!(strategy, RecoveryStrategy::Fallback);

    let message = handler.user_message(&error);
    assert_eq!(message, "Animation performance limit reached");
}

#[test]
fn test_error_handler_invalid_config() {
    let handler = DefaultErrorHandler::default();
    let error = AnimationError::InvalidConfig("Invalid configuration".to_string());
    let context = ErrorContext::new("test");

    let strategy = handler.handle_error(&error, &context);
    assert_eq!(strategy, RecoveryStrategy::Fallback);

    let message = handler.user_message(&error);
    assert_eq!(message, "Invalid animation configuration");
}

#[test]
fn test_error_handler_memory_error() {
    let handler = DefaultErrorHandler::default();
    let error = AnimationError::MemoryError("Out of memory".to_string());
    let context = ErrorContext::new("test");

    let strategy = handler.handle_error(&error, &context);
    assert_eq!(strategy, RecoveryStrategy::Abort);

    let message = handler.user_message(&error);
    assert_eq!(message, "Animation memory error");
}

#[test]
fn test_error_handler_timing_error() {
    let handler = DefaultErrorHandler::default();
    let error = AnimationError::TimingError("Invalid timing".to_string());
    let context = ErrorContext::new("test");

    let strategy = handler.handle_error(&error, &context);
    assert_eq!(strategy, RecoveryStrategy::Retry);

    let message = handler.user_message(&error);
    assert_eq!(message, "Animation timing error");
}

#[test]
fn test_error_context_clone() {
    let context = ErrorContext::new("test_operation")
        .with_component("test_component")
        .with_info("key", "value");

    let cloned = context.clone();

    assert_eq!(cloned.operation, context.operation);
    assert_eq!(cloned.component, context.component);
    assert_eq!(cloned.additional_info, context.additional_info);
}

#[test]
fn test_error_context_debug() {
    let context = ErrorContext::new("test_operation")
        .with_component("test_component")
        .with_info("key", "value");

    let debug_str = format!("{:?}", context);
    assert!(debug_str.contains("test_operation"));
    assert!(debug_str.contains("test_component"));
    assert!(debug_str.contains("key"));
    assert!(debug_str.contains("value"));
}

#[test]
fn test_recovery_strategy_debug() {
    let strategies = vec![
        RecoveryStrategy::Retry,
        RecoveryStrategy::Fallback,
        RecoveryStrategy::Skip,
        RecoveryStrategy::Abort,
    ];

    for strategy in strategies {
        let debug_str = format!("{:?}", strategy);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_recovery_strategy_clone() {
    let strategy = RecoveryStrategy::Fallback;
    let cloned = strategy.clone();

    assert_eq!(strategy, cloned);
}

#[test]
fn test_recovery_strategy_copy() {
    let strategy = RecoveryStrategy::Abort;
    let copied = strategy;

    assert_eq!(strategy, copied);
}

#[test]
fn test_default_error_handler_clone() {
    let handler = DefaultErrorHandler {
        log_errors: true,
        show_user_messages: false,
    };

    let cloned = handler.clone();

    assert_eq!(handler.log_errors, cloned.log_errors);
    assert_eq!(handler.show_user_messages, cloned.show_user_messages);
}

#[test]
fn test_default_error_handler_debug() {
    let handler = DefaultErrorHandler::default();
    let debug_str = format!("{:?}", handler);

    assert!(debug_str.contains("DefaultErrorHandler"));
}

#[test]
fn test_error_context_timestamp() {
    let context1 = ErrorContext::new("test1");
    std::thread::sleep(std::time::Duration::from_millis(1));
    let context2 = ErrorContext::new("test2");

    assert!(context1.timestamp < context2.timestamp);
}

#[test]
fn test_error_context_additional_info_mutability() {
    let mut context = ErrorContext::new("test");
    context
        .additional_info
        .insert("new_key".to_string(), "new_value".to_string());

    assert_eq!(
        context.additional_info.get("new_key"),
        Some(&"new_value".to_string())
    );
}

#[test]
fn test_recovery_strategy_equality() {
    let strategy1 = RecoveryStrategy::Fallback;
    let strategy2 = RecoveryStrategy::Fallback;
    let strategy3 = RecoveryStrategy::Skip;

    assert_eq!(strategy1, strategy2);
    assert_ne!(strategy1, strategy3);
}

#[test]
fn test_error_handler_with_disabled_logging() {
    let handler = DefaultErrorHandler {
        log_errors: false,
        show_user_messages: true,
    };

    let error = AnimationError::EngineUnavailable("test".to_string());
    let context = ErrorContext::new("test");

    // Should not log but still return strategy
    let strategy = handler.handle_error(&error, &context);
    assert_eq!(strategy, RecoveryStrategy::Fallback);

    // Should show user message
    let message = handler.user_message(&error);
    assert!(!message.is_empty());
}

#[test]
fn test_error_handler_with_disabled_user_messages() {
    let handler = DefaultErrorHandler {
        log_errors: true,
        show_user_messages: false,
    };

    let error = AnimationError::InvalidProperty {
        property: "test".to_string(),
    };

    // Should return empty message when disabled
    let message = handler.user_message(&error);
    assert_eq!(message, "");
}
