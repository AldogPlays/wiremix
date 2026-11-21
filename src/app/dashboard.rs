//! Coordinates section layout, live visibility and section-scoped pointer targets.

use std::collections::HashSet;

use crossterm::event::{KeyCode, MouseButton, MouseEventKind};
use ratatui::{
    layout::Flex,
    prelude::{Alignment, Buffer, Constraint, Layout, Position, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, StatefulWidget, Widget},
};
use smallvec::{smallvec, SmallVec};

use super::{Action, MouseArea, TabKind};
use crate::config::{help::Help, Config};
use crate::device_kind::DeviceKind;
use crate::help::{HelpWidget, HelpWidgetState};
use crate::object_list::{ObjectList, ObjectListWidget};
use crate::view::{self, ListKind, View};
use crate::wirehose::ObjectId;

pub(super) struct Section {
    pub title: String,
    pub list: ObjectList,
}

impl Section {
    fn new(title: String, list: ObjectList) -> Self {
        Self { title, list }
    }
}

pub(super) struct Dashboard {
    pub sections: Vec<Section>,
    pub focused: usize,
    pub help_position: Option<u16>,
    pub mouse_areas: Vec<MouseArea>,
}

impl Dashboard {
    pub fn new(initial_focus: TabKind) -> Self {
        let sections = vec![
            Section::new(
                TabKind::Playback.to_string(),
                ObjectList::new(ListKind::Node(view::NodeKind::Playback), None),
            ),
            Section::new(
                TabKind::Recording.to_string(),
                ObjectList::new(
                    ListKind::Node(view::NodeKind::Recording),
                    None,
                ),
            ),
            Section::new(
                TabKind::Output.to_string(),
                ObjectList::new(
                    ListKind::Node(view::NodeKind::Output),
                    Some(DeviceKind::Sink),
                ),
            ),
            Section::new(
                TabKind::Input.to_string(),
                ObjectList::new(
                    ListKind::Node(view::NodeKind::Input),
                    Some(DeviceKind::Source),
                ),
            ),
            Section::new(
                TabKind::Configuration.to_string(),
                ObjectList::new(ListKind::Device, None),
            ),
        ];
        Self {
            sections,
            focused: initial_focus.index(),
            help_position: None,
            mouse_areas: Vec::new(),
        }
    }

    pub fn current_list(&mut self) -> &mut ObjectList {
        &mut self.sections[self.focused].list
    }

    pub fn focus(&mut self, index: usize) {
        if index < self.sections.len() && index != self.focused {
            self.current_list().dropdown_close();
            self.focused = index;
        }
    }

    pub fn previous(&mut self) {
        self.focus(
            self.focused
                .checked_sub(1)
                .unwrap_or(self.sections.len() - 1),
        );
    }

    pub fn next(&mut self) {
        self.focus((self.focused + 1) % self.sections.len());
    }

    pub fn mouse_actions(
        &self,
        column: u16,
        row: u16,
        kind: MouseEventKind,
    ) -> SmallVec<[Action; 4]> {
        self.mouse_areas
            .iter()
            .rev()
            .find(|(rect, kinds, _)| {
                rect.contains(Position { x: column, y: row })
                    && kinds.contains(&kind)
            })
            .map(|(_, _, actions)| actions.clone())
            .unwrap_or_default()
    }

    /// Reconcile lists and capture visibility using the exact layout to be drawn.
    pub fn prepare<'a, 'b>(
        &'a mut self,
        area: Rect,
        view: &'a View<'b>,
        config: &'a Config,
    ) -> Presentation<'a, 'b> {
        let menu_was_open = self.sections[self.focused].list.dropdown_is_open();
        let mut controls = self.controls(area.width, config);
        let (mut sections, mut controls_area) = dashboard_areas(
            area,
            &self.sections,
            self.focused,
            view,
            controls.len(),
        );
        for (section, rect) in self.sections.iter_mut().zip(&sections) {
            section.list.update(*rect, view);
        }
        // Reconciliation can close a removed object's menu. Its replacement
        // hints may reserve a different number of rows in this very frame.
        if menu_was_open != self.sections[self.focused].list.dropdown_is_open()
        {
            controls = self.controls(area.width, config);
            (sections, controls_area) = dashboard_areas(
                area,
                &self.sections,
                self.focused,
                view,
                controls.len(),
            );
            for (section, rect) in self.sections.iter_mut().zip(&sections) {
                section.list.update(*rect, view);
            }
        }
        let mut visible_objects = HashSet::new();
        for (section, rect) in self.sections.iter().zip(&sections) {
            visible_objects.extend(section.list.visible_objects(rect, view));
        }
        Presentation {
            dashboard: self,
            area,
            view,
            config,
            sections,
            controls,
            controls_area,
            visible_objects,
        }
    }

    fn controls(&self, width: u16, config: &Config) -> Vec<Line<'static>> {
        let focused = &self.sections[self.focused].list;
        control_lines(
            config,
            width,
            focused.is_device(),
            focused.dropdown_is_open(),
            self.help_position.is_some(),
        )
    }
}

/// A prepared frame: its visibility and drawing share one set of rectangles.
pub(super) struct Presentation<'a, 'b> {
    dashboard: &'a mut Dashboard,
    area: Rect,
    view: &'a View<'b>,
    config: &'a Config,
    sections: Vec<Rect>,
    controls: Vec<Line<'static>>,
    controls_area: Rect,
    pub visible_objects: HashSet<ObjectId>,
}

impl Presentation<'_, '_> {
    pub fn render(self, buf: &mut Buffer) {
        let Self {
            dashboard,
            area,
            view,
            config,
            sections,
            controls,
            controls_area,
            ..
        } = self;
        let Dashboard {
            sections: tabs,
            focused,
            mouse_areas,
            help_position,
        } = dashboard;
        let focused = *focused;
        // Pointer targets are rebuilt with the frame, including help overlays.
        mouse_areas.clear();
        let list_area = Rect {
            height: controls_area.y.saturating_sub(area.y),
            ..area
        };

        for (index, (tab, section)) in tabs.iter_mut().zip(sections).enumerate()
        {
            let focused = index == focused;
            Block::bordered()
                .border_type(if focused {
                    config.char_set.help_border
                } else {
                    BorderType::Plain
                })
                .border_style(if focused {
                    config.theme.help_border
                } else {
                    Style::default()
                })
                .title_style(if focused {
                    config.theme.help_border.add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                })
                .title(format!(" {} ", tab.title))
                .render(section, buf);

            let start = mouse_areas.len();
            mouse_areas.push((
                section,
                smallvec![MouseEventKind::Down(MouseButton::Left)],
                smallvec![Action::SelectTab(index)],
            ));
            mouse_areas.push((
                section,
                smallvec![MouseEventKind::ScrollUp],
                smallvec![Action::MoveUp],
            ));
            mouse_areas.push((
                section,
                smallvec![MouseEventKind::ScrollDown],
                smallvec![Action::MoveDown],
            ));
            let mut widget = ObjectListWidget {
                object_list: &mut tab.list,
                view,
                config,
                focused,
            };
            widget.render(section, buf, mouse_areas);
            // Each list action must first address the section under the pointer.
            for (_, _, actions) in &mut mouse_areas[start..] {
                if actions.first() != Some(&Action::SelectTab(index)) {
                    actions.insert(0, Action::SelectTab(index));
                }
            }
        }
        for (line, row) in controls.into_iter().zip(controls_area.rows()) {
            line.render(row, buf);
        }

        // Render the help menu if it's open
        if let Some(ref mut help_position) = help_position {
            // Ignore any mouse actions on the lower area
            mouse_areas.clear();
            // Close the help menu if clicked anywhere outside
            mouse_areas.push((
                area,
                smallvec![MouseEventKind::Down(MouseButton::Left)],
                smallvec![Action::Help],
            ));

            let width: u16 = config
                .help
                .widths
                .iter()
                .fold(HelpWidget::base_width(), |acc, &x| acc.saturating_add(x))
                .try_into()
                .unwrap_or(u16::MAX);
            let [help_area] = Layout::horizontal([Constraint::Max(width)])
                .flex(Flex::Center)
                .areas(list_area);
            // Fit to the number of help text rows, or 80% of the area if they
            // don't all fit
            let height: u16 = config
                .help
                .rows
                .len()
                .saturating_add(2)
                .try_into()
                .unwrap_or(u16::MAX)
                .min(((help_area.height as f32) * 0.90) as u16);
            let [help_area] = Layout::vertical([Constraint::Length(height)])
                .flex(Flex::Center)
                .areas(help_area);

            Clear.render(help_area, buf);

            HelpWidget { config }.render(
                help_area,
                buf,
                &mut HelpWidgetState {
                    mouse_areas,
                    help_position,
                },
            );
        }
    }
}

/// Keep section frames visible, prioritizing complete cards before extra content.
pub(super) fn dashboard_areas(
    area: Rect,
    tabs: &[Section],
    focused: usize,
    view: &View,
    controls: usize,
) -> (Vec<Rect>, Rect) {
    let count = tabs.len();
    let footer_height = (controls as u16)
        .min(3)
        .min(area.height.saturating_sub(count as u16).max(1));
    let body_height = area.height.saturating_sub(footer_height);
    let base = if body_height >= count as u16 * 3 {
        3
    } else {
        1
    };
    let mut heights = vec![0u16; count];
    let mut remaining = body_height;
    for _ in 0..base {
        for height in &mut heights {
            if remaining > 0 {
                *height += 1;
                remaining -= 1;
            }
        }
    }
    let order: Vec<_> = std::iter::once(focused)
        .chain((0..count).filter(|&index| index != focused))
        .collect();
    // First expose one complete control card in every populated section.
    for &index in &order {
        let minimum = tabs[index].list.preferred_height(view).min(5);
        let extra = minimum.saturating_sub(heights[index]).min(remaining);
        heights[index] += extra;
        remaining -= extra;
    }
    // Then distribute room for additional devices/streams, rather than leaving
    // large empty lists while another section is unnecessarily scrolled.
    while remaining > 0 {
        let mut changed = false;
        for &index in &order {
            if remaining > 0
                && heights[index] < tabs[index].list.preferred_height(view)
            {
                heights[index] += 1;
                remaining -= 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    heights[focused] += remaining;
    let mut y = area.y;
    let sections = heights
        .into_iter()
        .map(|height| {
            let rect = Rect::new(area.x, y, area.width, height);
            y = y.saturating_add(height);
            rect
        })
        .collect();
    (
        sections,
        Rect::new(
            area.x,
            y,
            area.width,
            area.height.saturating_sub(body_height),
        ),
    )
}

/// Hints come from actual bindings, so remapped or disabled keys are not lied
/// about. The action names TabLeft/TabRight remain compatible with old configs.
pub(super) fn control_lines(
    config: &Config,
    width: u16,
    device: bool,
    menu: bool,
    help: bool,
) -> Vec<Line<'static>> {
    let mut hints = if help {
        vec![
            (Action::Help, "Close help"),
            (Action::Exit, "Quit"),
            (Action::MoveUp, "Up"),
            (Action::MoveDown, "Down"),
            (Action::CloseDropdown, "Close"),
        ]
    } else {
        vec![
            (Action::Help, "Help"),
            (Action::Exit, "Quit"),
            (Action::TabRight, "Next section"),
            (Action::TabLeft, "Previous"),
            (Action::MoveUp, "Up"),
            (Action::MoveDown, "Down"),
        ]
    };
    if !device && !menu && !help {
        hints.extend([
            (Action::SetRelativeVolume(-0.01), "Volume -"),
            (Action::SetRelativeVolume(0.01), "Volume +"),
            (Action::ToggleMute, "Mute"),
            (Action::SetDefault, "Default"),
        ]);
    }
    if !help {
        hints.push((
            Action::ActivateDropdown,
            if menu {
                "Choose"
            } else if device {
                "Profile"
            } else {
                "Route"
            },
        ));
    }
    if menu && !help {
        hints.push((Action::CloseDropdown, "Cancel"));
    }

    let mut lines = Vec::new();
    let mut spans = Vec::new();
    let mut used = 0;
    for (action, label) in hints {
        let mut keys: Vec<_> = config
            .keybindings
            .iter()
            .filter(|(_, bound)| **bound == action)
            .map(|(key, _)| {
                let priority = match key.code {
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Tab
                    | KeyCode::BackTab
                    | KeyCode::Enter
                    | KeyCode::Esc => 0,
                    _ => 1,
                };
                let label = if key.modifiers.is_empty() {
                    match key.code {
                        KeyCode::Up => "↑".to_owned(),
                        KeyCode::Down => "↓".to_owned(),
                        KeyCode::Left => "←".to_owned(),
                        KeyCode::Right => "→".to_owned(),
                        _ => Help::format_key(key),
                    }
                } else {
                    Help::format_key(key)
                };
                (priority, !key.modifiers.is_empty(), label)
            })
            .collect();
        keys.sort();
        let Some((_, _, key)) = keys.first() else {
            continue;
        };
        let length = Line::raw(format!("{key} {label}")).width();
        if used > 0 && used + 3 + length > usize::from(width) {
            lines.push(Line::from(std::mem::take(&mut spans)));
            used = 0;
        }
        if used > 0 {
            spans.push(Span::styled(" | ", config.theme.control_hint));
            used += 3;
        }
        spans.push(Span::styled(
            key.clone(),
            config.theme.control_hint.add_modifier(Modifier::BOLD),
        ));
        spans
            .push(Span::styled(format!(" {label}"), config.theme.control_hint));
        used += length;
    }
    if !spans.is_empty() {
        lines.push(Line::from(spans));
    }
    lines
        .into_iter()
        .map(|line| line.alignment(Alignment::Center))
        .collect()
}
