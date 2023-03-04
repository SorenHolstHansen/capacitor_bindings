use serde::{Deserialize, Serialize};
use crate::extern_functions::*;

use crate::helpers::*;


pub struct Haptics;

impl Haptics {
    /// Trigger a haptics "impact" feedback
    pub async fn impact(options: impl Into<ImpactOptions>) -> Result<(), Error> {
        run_value_unit(options, haptics_impact).await
    }

    /// Vibrate the device
    pub async fn vibrate(options: impl Into<VibrateOptions>) -> Result<(), Error> {
        run_value_unit(options, haptics_vibrate).await
    }

    /// Trigger a haptics "notification" feedback
    pub async fn notification(options: impl Into<NotificationOptions>) -> Result<(), Error> {
        run_value_unit(options, haptics_notification).await
    }

    pub async fn selection_start() -> Result<(), Error> {
        run_unit_unit(haptics_selectionStart).await
    }

    pub async fn selection_changed() -> Result<(), Error> {
        run_unit_unit(haptics_selectionChanged).await
    }

    pub async fn selection_end() -> Result<(), Error> {
        run_unit_unit(haptics_selectionEnd).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VibrateOptions {
    /// Duration of the vibration in milliseconds.
    pub duration: f64,
}

impl From<f64> for VibrateOptions {
    fn from(duration: f64) -> Self {
        Self { duration }
    }
}

impl Default for VibrateOptions {
    fn default() -> Self {
        Self { duration: 300.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct NotificationOptions {
    /// Notification Feedback Type The type of notification feedback generated by a UINotificationFeedbackGenerator object.
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
}

impl From<NotificationType> for NotificationOptions {
    fn from(notification_type: NotificationType) -> Self {
        Self { notification_type }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NotificationType {
    /// A notification feedback type indicating that a task has completed successfully
    Success,
    /// A notification feedback type indicating that a task has produced a warning
    Warning,
    /// A notification feedback type indicating that a task has failed
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct ImpactOptions {
    /// Impact Feedback Style The mass of the objects in the collision simulated by a UIImpactFeedbackGenerator object.
    pub style: ImpactStyle,
}

impl From<ImpactStyle> for ImpactOptions {
    fn from(style: ImpactStyle) -> Self {
        Self { style }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ImpactStyle {
    /// A collision between large, heavy user interface elements
    Heavy,
    /// A collision between moderately sized user interface elements
    Medium,
    /// A collision between small, light user interface elements
    Light,
}
