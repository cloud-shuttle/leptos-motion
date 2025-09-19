//! Pool configuration settings

/// Configuration for the animation pool
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of animations in the pool
    pub max_capacity: usize,
    /// Initial pool size
    pub initial_size: usize,
    /// Growth factor when pool needs to expand
    pub growth_factor: f32,
    /// Whether to pre-allocate animations
    pub pre_allocate: bool,
    /// Maximum memory usage in bytes
    pub max_memory_bytes: usize,
    /// Whether to enable memory pressure monitoring
    pub enable_memory_monitoring: bool,
    /// Cleanup interval in seconds
    pub cleanup_interval: f64,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_capacity: 1000,
            initial_size: 100,
            growth_factor: 1.5,
            pre_allocate: true,
            max_memory_bytes: 100 * 1024 * 1024, // 100MB
            enable_memory_monitoring: true,
            cleanup_interval: 30.0,
        }
    }
}

impl PoolConfig {
    /// Create a new pool configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum capacity
    pub fn with_max_capacity(mut self, capacity: usize) -> Self {
        self.max_capacity = capacity;
        self
    }

    /// Set initial size
    pub fn with_initial_size(mut self, size: usize) -> Self {
        self.initial_size = size;
        self
    }

    /// Set growth factor
    pub fn with_growth_factor(mut self, factor: f32) -> Self {
        self.growth_factor = factor;
        self
    }

    /// Set pre-allocation
    pub fn with_pre_allocate(mut self, pre_allocate: bool) -> Self {
        self.pre_allocate = pre_allocate;
        self
    }

    /// Set maximum memory usage
    pub fn with_max_memory(mut self, memory_bytes: usize) -> Self {
        self.max_memory_bytes = memory_bytes;
        self
    }

    /// Set memory monitoring
    pub fn with_memory_monitoring(mut self, enable: bool) -> Self {
        self.enable_memory_monitoring = enable;
        self
    }

    /// Set cleanup interval
    pub fn with_cleanup_interval(mut self, interval: f64) -> Self {
        self.cleanup_interval = interval;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.max_capacity == 0 {
            return Err("Max capacity must be greater than 0".to_string());
        }

        if self.initial_size > self.max_capacity {
            return Err("Initial size cannot exceed max capacity".to_string());
        }

        if self.growth_factor <= 1.0 {
            return Err("Growth factor must be greater than 1.0".to_string());
        }

        if self.max_memory_bytes == 0 {
            return Err("Max memory must be greater than 0".to_string());
        }

        if self.cleanup_interval <= 0.0 {
            return Err("Cleanup interval must be greater than 0".to_string());
        }

        Ok(())
    }
}
