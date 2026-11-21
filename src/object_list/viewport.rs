//! Card geometry shared by scrolling, rendering and live-object tracking.

use std::ops::Range;
use std::rc::Rc;

use ratatui::prelude::{Constraint, Layout, Rect};

pub(super) struct Viewport {
    pub header: Rect,
    pub list: Rect,
    pub footer: Rect,
    pub cards: Rc<[Rect]>,
    complete_cards: usize,
    bottom_card_complete: bool,
}

impl Viewport {
    pub fn new(area: Rect, height: u16, spacing: u16) -> Self {
        let area = Rect {
            x: area.x.saturating_add(u16::from(area.width > 0)),
            width: area.width.saturating_sub(2),
            ..area
        };
        let [header, list, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(area);
        let stride = height.saturating_add(spacing);
        let complete_cards =
            usize::from(list.height.saturating_add(spacing) / stride);
        // A title-only card still needs live updates. Spacing alone does not.
        let rendered = if list.height == 0 {
            0
        } else {
            usize::from((list.height - 1) / stride + 1)
        };
        let mut constraints = vec![Constraint::Length(height); complete_cards];
        if rendered > complete_cards {
            constraints.push(Constraint::Max(height));
        }
        let cards = Layout::vertical(constraints).spacing(spacing).split(list);
        Self {
            header,
            list,
            footer,
            cards,
            complete_cards,
            bottom_card_complete: list.height % stride >= height,
        }
    }

    pub fn reconcile(
        &self,
        mut top: usize,
        count: usize,
        selected: Option<usize>,
    ) -> usize {
        if top >= count {
            top = count.saturating_sub(self.complete_cards);
        }
        if let Some(selected) = selected {
            if selected >= top.saturating_add(self.complete_cards) {
                top = selected
                    .saturating_sub(self.complete_cards.saturating_sub(1));
            } else if selected < top {
                top = selected;
            }
        }
        top
    }

    pub fn visible_range(&self, top: usize, count: usize) -> Range<usize> {
        let start = top.min(count);
        start..start.saturating_add(self.cards.len()).min(count)
    }

    pub fn has_more_below(&self, top: usize, count: usize) -> bool {
        let next = top.saturating_add(self.complete_cards);
        // Preserve the existing indicator treatment of a complete last card
        // whose trailing spacing does not fit.
        next < count
            && !(next == count.saturating_sub(1) && self.bottom_card_complete)
    }
}
