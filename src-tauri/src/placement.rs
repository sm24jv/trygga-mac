use std::time::Duration;

/// How long after a loss of focus a press on the icon still counts as a press on an open panel.
const PRESS_AFTER_HIDING: Duration = Duration::from_millis(200);

/// A rectangle in the physical pixels the tray and the monitors report.
#[derive(Clone, Copy)]
pub struct Area {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

/// A screen, in the physical pixels of its own scale.
pub struct Screen {
    pub area: Area,
    pub scale: f64,
}

/// Where the panel's top left corner goes, in logical pixels: centred under the icon and held on
/// the icon's screen. Logical, because a window converts physical pixels by the scale of the screen
/// it stands on now, and that may be another one than the icon's.
pub fn under_icon(icon: Area, panel_width: f64, screen: &Screen) -> (f64, f64) {
    let middle = (icon.left + icon.width / 2.0) / screen.scale;
    let left_edge = screen.area.left / screen.scale;
    let right_edge = left_edge + screen.area.width / screen.scale;
    let left = (middle - panel_width / 2.0)
        .min(right_edge - panel_width)
        .max(left_edge);
    (left, (icon.top + icon.height) / screen.scale)
}

/// Whether a press on the icon found the panel open. A click on the icon can take the focus first,
/// which hides the panel just before the press arrives, and that click means close.
pub fn found_open(showing: bool, since_hidden: Option<Duration>) -> bool {
    showing || since_hidden.is_some_and(|elapsed| elapsed < PRESS_AFTER_HIDING)
}

#[cfg(test)]
mod tests {
    use super::{found_open, under_icon, Area, Screen};
    use std::time::Duration;

    fn area(left: f64, top: f64, width: f64, height: f64) -> Area {
        Area {
            left,
            top,
            width,
            height,
        }
    }

    fn retina() -> Screen {
        Screen {
            area: area(0.0, 0.0, 2880.0, 1800.0),
            scale: 2.0,
        }
    }

    #[test]
    fn the_panel_stands_centred_under_the_icon() {
        let icon = area(2000.0, 0.0, 44.0, 48.0);
        assert_eq!(under_icon(icon, 380.0, &retina()), (821.0, 24.0));
    }

    #[test]
    fn the_panel_stays_on_the_screen() {
        let right = area(2830.0, 0.0, 44.0, 48.0);
        assert_eq!(under_icon(right, 380.0, &retina()), (1060.0, 24.0));
        let left = area(10.0, 0.0, 44.0, 48.0);
        assert_eq!(under_icon(left, 380.0, &retina()), (0.0, 24.0));
    }

    #[test]
    fn a_second_screen_counts_by_its_own_scale() {
        let second = Screen {
            area: area(1440.0, 0.0, 1920.0, 1080.0),
            scale: 1.0,
        };
        let icon = area(3000.0, 0.0, 22.0, 24.0);
        assert_eq!(under_icon(icon, 380.0, &second), (2821.0, 24.0));
    }

    #[test]
    fn a_press_finds_the_panel_open_while_it_shows_or_just_after_it_hid() {
        assert!(found_open(true, None));
        assert!(found_open(false, Some(Duration::from_millis(50))));
        assert!(!found_open(false, Some(Duration::from_secs(1))));
        assert!(!found_open(false, None));
    }
}
