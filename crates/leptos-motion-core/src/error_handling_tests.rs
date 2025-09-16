//! Comprehensive unit tests for error handling system

use crate::{AnimationError, AnimationHandle, ErrorContext, ErrorHandler, DefaultErrorHandler, RecoveryStrategy};
use std::collections::HashMap;

#[cfg(test)]
mod animation_error_tests {
    use super::*;

    #[test]
    fn test_animation_error_engine_unavailable() {
        let error = AnimationError::EngineUnavailable("Test engine".to_string());
        assert_eq!(format!("{}", error), "Animation engine not available: Test engine");
    }

    #[test]
    fn test_animation_error_invalid_property() {
        let error = AnimationError::InvalidProperty {
            property: "invalid_prop".to_string(),
        };
        assert_eq!(format!("{}", error), "Invalid animation property: invalid_prop");
    }

    #[test]
    fn test_animation_error_already_running() {
        let handle = AnimationHandle(123);
        let error = AnimationError::AlreadyRunning { handle };
        assert_eq!(format!("{}", error), "Animation already running with handle: AnimationHandle(123)");
    }

    #[test]
    fn test_animation_error_not_found() {
        let handle = AnimationHandle(456);
        let error = AnimationError::NotFound { handle };
        assert_eq!(format!("{}", error), "Animation not found: AnimationHandle(456)");
    }

    #[test]
    fn test_animation_error_dom_error() {
        let error = AnimationError::DomError("DOM operation failed".to_string());
        assert_eq!(format!("{}", error), "DOM operation failed: DOM operation failed");
    }

    #[test]
    fn test_animation_error_math_error() {
        let error = AnimationError::MathError("Division by zero".to_string());
        assert_eq!(format!("{}", error), "Math error: Division by zero");
    }

    #[test]
    fn test_animation_error_performance_budget_exceeded() {
        let error = AnimationError::PerformanceBudgetExceeded("Frame time exceeded".to_string());
        assert_eq!(format!("{}", error), "Performance budget exceeded: Frame time exceeded");
    }

    #[test]
    fn test_animation_error_invalid_config() {
        let error = AnimationError::InvalidConfig("Invalid duration".to_string());
        assert_eq!(format!("{}", error), "Invalid animation configuration: Invalid duration");
    }

    #[test]
    fn test_animation_error_memory_error() {
        let error = AnimationError::MemoryError("Out of memory".to_string());
        assert_eq!(format!("{}", error), "Memory allocation failed: Out of memory");
    }

    #[test]
    fn test_animation_error_timing_error() {
        let error = AnimationError::TimingError("Invalid timing".to_string());
        assert_eq!(format!("{}", error), "Animation timing error: Invalid timing");
    }

    #[test]
    fn test_animation_error_not_implemented() {
        let error = AnimationError::NotImplemented("Feature X".to_string());
        assert_eq!(format!("{}", error), "Feature not yet implemented: Feature X");
    }

    #[test]
    fn test_animation_error_invalid_value() {
        let error = AnimationError::InvalidValue("NaN value".to_string());
        assert_eq!(format!("{}", error), "Invalid animation value: NaN value");
    }

    #[test]
    fn test_animation_error_debug() {
        let error = AnimationError::EngineUnavailable("Test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("EngineUnavailable"));
        assert!(debug_str.contains("Test"));
    }

    #[test]
    fn test_animation_error_debug_formatting() {
        let error = AnimationError::EngineUnavailable("Test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("EngineUnavailable"));
        assert!(debug_str.contains("Test"));
    }
}

#[cfg(test)]
mod error_context_tests {
    use super::*;

    #[test]
    fn test_error_context_new() {
        let context = ErrorContext::new("test_operation");
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.component, None);
        assert!(context.additional_info.is_empty());
    }

    #[test]
    fn test_error_context_with_component() {
        let context = ErrorContext::new("test_operation")
            .with_component("TestComponent");
        
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.component, Some("TestComponent".to_string()));
    }

    #[test]
    fn test_error_context_with_info() {
        let context = ErrorContext::new("test_operation")
            .with_info("key1", "value1")
            .with_info("key2", "value2");
        
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.additional_info.get("key1"), Some(&"value1".to_string()));
        assert_eq!(context.additional_info.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_error_context_chaining() {
        let context = ErrorContext::new("test_operation")
            .with_component("TestComponent")
            .with_info("key1", "value1")
            .with_info("key2", "value2");
        
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.component, Some("TestComponent".to_string()));
        assert_eq!(context.additional_info.get("key1"), Some(&"value1".to_string()));
        assert_eq!(context.additional_info.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_error_context_timestamp() {
        let context1 = ErrorContext::new("test_operation");
        std::thread::sleep(std::time::Duration::from_millis(1));
        let context2 = ErrorContext::new("test_operation");
        
        assert!(context2.timestamp > context1.timestamp);
    }

    #[test]
    fn test_error_context_clone() {
        let context1 = ErrorContext::new("test_operation")
            .with_component("TestComponent")
            .with_info("key1", "value1");
        
        let context2 = context1.clone();
        assert_eq!(context1.operation, context2.operation);
        assert_eq!(context1.component, context2.component);
        assert_eq!(context1.additional_info, context2.additional_info);
    }

    #[test]
    fn test_error_context_debug() {
        let context = ErrorContext::new("test_operation")
            .with_component("TestComponent")
            .with_info("key1", "value1");
        
        let debug_str = format!("{:?}", context);
        assert!(debug_str.contains("ErrorContext"));
        assert!(debug_str.contains("test_operation"));
        assert!(debug_str.contains("TestComponent"));
    }
}

#[cfg(test)]
mod recovery_strategy_tests {
    use super::*;

    #[test]
    fn test_recovery_strategy_retry() {
        let strategy = RecoveryStrategy::Retry;
        assert_eq!(format!("{:?}", strategy), "Retry");
    }

    #[test]
    fn test_recovery_strategy_fallback() {
        let strategy = RecoveryStrategy::Fallback;
        assert_eq!(format!("{:?}", strategy), "Fallback");
    }

    #[test]
    fn test_recovery_strategy_skip() {
        let strategy = RecoveryStrategy::Skip;
        assert_eq!(format!("{:?}", strategy), "Skip");
    }

    #[test]
    fn test_recovery_strategy_abort() {
        let strategy = RecoveryStrategy::Abort;
        assert_eq!(format!("{:?}", strategy), "Abort");
    }

    #[test]
    fn test_recovery_strategy_equality() {
        let strategy1 = RecoveryStrategy::Retry;
        let strategy2 = RecoveryStrategy::Retry;
        let strategy3 = RecoveryStrategy::Fallback;
        
        assert_eq!(strategy1, strategy2);
        assert_ne!(strategy1, strategy3);
    }

    #[test]
    fn test_recovery_strategy_clone() {
        let strategy1 = RecoveryStrategy::Retry;
        let strategy2 = strategy1.clone();
        assert_eq!(strategy1, strategy2);
    }

    #[test]
    fn test_recovery_strategy_copy() {
        let strategy1 = RecoveryStrategy::Retry;
        let strategy2 = strategy1; // This should work because RecoveryStrategy implements Copy
        assert_eq!(strategy1, strategy2);
    }
}

#[cfg(test)]
mod default_error_handler_tests {
    use super::*;

    #[test]
    fn test_default_error_handler_default() {
        let handler = DefaultErrorHandler::default();
        assert!(handler.log_errors);
        assert!(!handler.show_user_messages);
    }

    #[test]
    fn test_default_error_handler_new() {
        let handler = DefaultErrorHandler {
            log_errors: false,
            show_user_messages: true,
        };
        assert!(!handler.log_errors);
        assert!(handler.show_user_messages);
    }

    #[test]
    fn test_default_error_handler_clone() {
        let handler1 = DefaultErrorHandler {
            log_errors: false,
            show_user_messages: true,
        };
        let handler2 = handler1.clone();
        assert_eq!(handler1.log_errors, handler2.log_errors);
        assert_eq!(handler1.show_user_messages, handler2.show_user_messages);
    }

    #[test]
    fn test_default_error_handler_debug() {
        let handler = DefaultErrorHandler::default();
        let debug_str = format!("{:?}", handler);
        assert!(debug_str.contains("DefaultErrorHandler"));
    }

    #[test]
    fn test_default_error_handler_handle_error_engine_unavailable() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::EngineUnavailable("Test".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Fallback);
    }

    #[test]
    fn test_default_error_handler_handle_error_invalid_property() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::InvalidProperty {
            property: "invalid".to_string(),
        };
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Skip);
    }

    #[test]
    fn test_default_error_handler_handle_error_already_running() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::AlreadyRunning {
            handle: AnimationHandle(123),
        };
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Skip);
    }

    #[test]
    fn test_default_error_handler_handle_error_not_found() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::NotFound {
            handle: AnimationHandle(123),
        };
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Abort);
    }

    #[test]
    fn test_default_error_handler_handle_error_dom_error() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::DomError("DOM error".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Retry);
    }

    #[test]
    fn test_default_error_handler_handle_error_math_error() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::MathError("Math error".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Skip);
    }

    #[test]
    fn test_default_error_handler_handle_error_performance_budget_exceeded() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::PerformanceBudgetExceeded("Performance error".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Fallback);
    }

    #[test]
    fn test_default_error_handler_handle_error_invalid_config() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::InvalidConfig("Config error".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Fallback);
    }

    #[test]
    fn test_default_error_handler_handle_error_memory_error() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::MemoryError("Memory error".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Abort);
    }

    #[test]
    fn test_default_error_handler_handle_error_timing_error() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::TimingError("Timing error".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Retry);
    }

    #[test]
    fn test_default_error_handler_handle_error_not_implemented() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::NotImplemented("Feature".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Abort);
    }

    #[test]
    fn test_default_error_handler_handle_error_invalid_value() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::InvalidValue("Value error".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Skip);
    }

    #[test]
    fn test_default_error_handler_log_error() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::EngineUnavailable("Test".to_string());
        let context = ErrorContext::new("test_operation");
        
        // This should not panic
        handler.log_error(&error, &context);
    }

    #[test]
    fn test_default_error_handler_user_message_disabled() {
        let handler = DefaultErrorHandler {
            log_errors: true,
            show_user_messages: false,
        };
        let error = AnimationError::EngineUnavailable("Test".to_string());
        
        let message = handler.user_message(&error);
        assert_eq!(message, "");
    }

    #[test]
    fn test_default_error_handler_user_message_enabled() {
        let handler = DefaultErrorHandler {
            log_errors: true,
            show_user_messages: true,
        };
        
        let error = AnimationError::EngineUnavailable("Test".to_string());
        let message = handler.user_message(&error);
        assert_eq!(message, "Animation system temporarily unavailable");
        
        let error = AnimationError::InvalidProperty {
            property: "invalid".to_string(),
        };
        let message = handler.user_message(&error);
        assert_eq!(message, "Invalid animation property");
        
        let error = AnimationError::AlreadyRunning {
            handle: AnimationHandle(123),
        };
        let message = handler.user_message(&error);
        assert_eq!(message, "Animation already in progress");
        
        let error = AnimationError::NotFound {
            handle: AnimationHandle(123),
        };
        let message = handler.user_message(&error);
        assert_eq!(message, "Animation not found");
        
        let error = AnimationError::DomError("DOM error".to_string());
        let message = handler.user_message(&error);
        assert_eq!(message, "Animation display error");
        
        let error = AnimationError::MathError("Math error".to_string());
        let message = handler.user_message(&error);
        assert_eq!(message, "Animation calculation error");
        
        let error = AnimationError::PerformanceBudgetExceeded("Performance error".to_string());
        let message = handler.user_message(&error);
        assert_eq!(message, "Animation performance limit reached");
        
        let error = AnimationError::InvalidConfig("Config error".to_string());
        let message = handler.user_message(&error);
        assert_eq!(message, "Invalid animation configuration");
        
        let error = AnimationError::MemoryError("Memory error".to_string());
        let message = handler.user_message(&error);
        assert_eq!(message, "Animation memory error");
        
        let error = AnimationError::TimingError("Timing error".to_string());
        let message = handler.user_message(&error);
        assert_eq!(message, "Animation timing error");
        
        let error = AnimationError::NotImplemented("Feature".to_string());
        let message = handler.user_message(&error);
        assert_eq!(message, "Feature not yet available");
        
        let error = AnimationError::InvalidValue("Value error".to_string());
        let message = handler.user_message(&error);
        assert_eq!(message, "Invalid animation value");
    }
}

#[cfg(test)]
mod error_handler_trait_tests {
    use super::*;

    // Custom error handler for testing
    struct TestErrorHandler {
        pub recovery_strategy: RecoveryStrategy,
        pub log_called: bool,
        pub user_message: String,
    }

    impl ErrorHandler for TestErrorHandler {
        fn handle_error(&self, _error: &AnimationError, _context: &ErrorContext) -> RecoveryStrategy {
            self.recovery_strategy
        }

        fn log_error(&self, _error: &AnimationError, _context: &ErrorContext) {
            // Mark that log_error was called
        }

        fn user_message(&self, _error: &AnimationError) -> String {
            self.user_message.clone()
        }
    }

    #[test]
    fn test_custom_error_handler() {
        let handler = TestErrorHandler {
            recovery_strategy: RecoveryStrategy::Retry,
            log_called: false,
            user_message: "Custom message".to_string(),
        };
        
        let error = AnimationError::EngineUnavailable("Test".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Retry);
        
        let message = handler.user_message(&error);
        assert_eq!(message, "Custom message");
    }

    #[test]
    fn test_error_handler_trait_object() {
        let handler: Box<dyn ErrorHandler> = Box::new(DefaultErrorHandler::default());
        
        let error = AnimationError::EngineUnavailable("Test".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Fallback);
        
        let message = handler.user_message(&error);
        assert_eq!(message, ""); // Default handler has show_user_messages = false
    }
}
