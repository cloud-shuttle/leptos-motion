//! Pool monitoring component

use super::*;
use leptos::*;
use leptos::prelude::*;

/// Pool monitoring component
#[component]
pub fn PoolMonitor(
    /// Pool to monitor
    pool: ReadSignal<AnimationPool>,
    /// Update interval in milliseconds
    #[prop(optional, default = 1000)]
    update_interval: u64,
) -> impl IntoView {
    // For now, create signals with default values and update them manually
    let (stats, set_stats) = signal(MemoryStats::default());
    let (metrics, set_metrics) = signal(PerformanceMetrics::default());
    let (status, set_status) = signal(PoolStatus::default());

    // Update stats periodically - simplified approach
    set_interval(move || {
        // For now, use default values until AnimationPool trait bounds are fixed
        set_stats.set(MemoryStats::default());
        set_metrics.set(PerformanceMetrics::default());
        set_status.set(PoolStatus::default());
    }, std::time::Duration::from_millis(update_interval));

    view! {
        <div class="pool-monitor" style="padding: 20px; background: #f5f5f5; border-radius: 8px; font-family: monospace;">
            <h3 style="margin-top: 0; color: #333;">"Animation Pool Monitor"</h3>
            
            <div class="stats-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin-bottom: 20px;">
                <div class="stat-card" style="background: white; padding: 15px; border-radius: 6px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <h4 style="margin: 0 0 10px 0; color: #666;">"Memory Usage"</h4>
                    <div style="font-size: 24px; font-weight: bold; color: #007bff;">
                        {format!("{:.1}%", stats.get().usage_percentage())}
                    </div>
                    <div style="font-size: 12px; color: #999;">
                        {format!("{} / {} bytes", stats.get().memory_in_use, stats.get().total_allocated)}
                    </div>
                </div>

                <div class="stat-card" style="background: white; padding: 15px; border-radius: 6px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <h4 style="margin: 0 0 10px 0; color: #666;">"Active Animations"</h4>
                    <div style="font-size: 24px; font-weight: bold; color: #28a745;">
                        {status.get().active_animations}
                    </div>
                    <div style="font-size: 12px; color: #999;">
                        {format!("{} available", status.get().available_animations)}
                    </div>
                </div>

                <div class="stat-card" style="background: white; padding: 15px; border-radius: 6px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <h4 style="margin: 0 0 10px 0; color: #666;">"Cache Hit Rate"</h4>
                    <div style="font-size: 24px; font-weight: bold; color: #ffc107;">
                        {format!("{:.1}%", metrics.get().cache_hit_rate * 100.0)}
                    </div>
                    <div style="font-size: 12px; color: #999;">
                        {format!("{} hits", metrics.get().total_animations_reused)}
                    </div>
                </div>

                <div class="stat-card" style="background: white; padding: 15px; border-radius: 6px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <h4 style="margin: 0 0 10px 0; color: #666;">"Performance Score"</h4>
                    <div style="font-size: 24px; font-weight: bold; color: #17a2b8;">
                        {format!("{:.1}%", metrics.get().performance_score() * 100.0)}
                    </div>
                    <div style="font-size: 12px; color: #999;">
                        {format!("{} created", metrics.get().total_animations_created)}
                    </div>
                </div>
            </div>

            <div class="detailed-stats" style="background: white; padding: 15px; border-radius: 6px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                <h4 style="margin: 0 0 15px 0; color: #333;">"Detailed Statistics"</h4>
                
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 10px; font-size: 14px;">
                    <div>
                        <strong>"Peak Memory:"</strong><br/>
                        {format!("{} bytes", stats.get().peak_memory_usage)}
                    </div>
                    <div>
                        <strong>"Fragmentation:"</strong><br/>
                        {format!("{:.1}%", stats.get().fragmentation_percent)}
                    </div>
                    <div>
                        <strong>"Efficiency:"</strong><br/>
                        {format!("{:.1}%", stats.get().efficiency())}
                    </div>
                    <div>
                        <strong>"Avg Creation Time:"</strong><br/>
                        {format!("{} μs", metrics.get().avg_creation_time_us)}
                    </div>
                    <div>
                        <strong>"Avg Reuse Time:"</strong><br/>
                        {format!("{} μs", metrics.get().avg_reuse_time_us)}
                    </div>
                    <div>
                        <strong>"Pool Expansions:"</strong><br/>
                        {metrics.get().pool_expansions}
                    </div>
                </div>
            </div>
        </div>
    }
}
