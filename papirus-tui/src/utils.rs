use std::time::Duration;

pub(crate) fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let millis = duration.subsec_millis();

    // Break down the duration
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    // Build the string based on the components
    match (hours, minutes, seconds, millis) {
        (0, 0, 0, ms) => format!("{}ms", ms),
        (0, 0, s, 0) => format!("{}s", s),
        (0, 0, s, ms) => format!("{}.{:03}s", s, ms),
        (0, m, s, 0) => format!("{}m {}s", m, s),
        (0, m, s, ms) => format!("{}m {}.{:03}s", m, s, ms),
        (h, m, s, 0) => format!("{}h {}m {}s", h, m, s),
        (h, m, s, ms) => format!("{}h {}m {}.{:03}s", h, m, s, ms),
    }
}
