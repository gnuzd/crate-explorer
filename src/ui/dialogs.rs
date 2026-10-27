use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Clear, Padding, Paragraph},
};

use super::theme::Theme;

struct Section {
    title: &'static str,
    rows: &'static [(&'static str, &'static str)],
}

const SECTIONS: [Section; 6] = [
    Section {
        title: "GLOBAL",
        rows: &[
            ("1-8", "switch view"),
            ("tab", "next pane"),
            ("/", "filter"),
            ("?", "help"),
            (":", "command palette"),
            ("q", "quit"),
        ],
    },
    Section {
        title: "DEPS",
        rows: &[
            ("a", "add"),
            ("u", "update"),
            ("U", "update all compatible"),
            ("r", "remove"),
            ("f", "features"),
            ("w", "why?"),
            ("enter", "details"),
        ],
    },
    Section {
        title: "TREE",
        rows: &[
            ("enter", "expand/collapse"),
            ("e", "expand all"),
            ("d", "duplicates only"),
            ("w", "why?"),
        ],
    },
    Section {
        title: "FEATURES",
        rows: &[
            ("space", "toggle"),
            ("a", "apply"),
            ("r", "reset"),
            ("d", "toggle default"),
        ],
    },
    Section {
        title: "UPDATE",
        rows: &[
            ("space", "select"),
            ("A", "apply compatible"),
            ("a", "apply plan"),
            ("enter", "changelog"),
        ],
    },
    Section {
        title: "AUDIT",
        rows: &[
            ("f", "fix"),
            ("F", "fix all"),
            ("i", "ignore"),
            ("o", "open advisory"),
        ],
    },
];

const KEY_COL: usize = 12; // độ rộng cột phím, để phần mô tả thẳng hàng

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let w = width.min(area.width);
    let h = height.min(area.height);
    Rect::new(
        area.x + (area.width - w) / 2,
        area.y + (area.height - h) / 2,
        w,
        h,
    )
}

fn section_lines(s: &Section, t: &Theme, width: u16) -> Vec<Line<'static>> {
    let kw = s
        .rows
        .iter()
        .map(|(k, _)| k.chars().count())
        .max()
        .unwrap_or(0);

    let rule = "─".repeat(width.saturating_sub(4) as usize);
    let mut lines = vec![
        Line::styled(s.title, Style::new().fg(t.tan).bold()),
        Line::styled(rule, Style::new().fg(t.border)),
    ];

    for &(k, d) in s.rows {
        lines.push(Line::from(vec![
            Span::styled(format!(" {k:<kw$} "), Style::new().fg(t.accent).bold()),
            Span::raw("  "),
            Span::styled(d, Style::new().fg(t.text)),
        ]));
    }
    lines
}

fn grid_row(f: &mut Frame, area: Rect, sections: &[Section], t: &Theme) {
    let cols = Layout::horizontal([Constraint::Ratio(1, 3); 3]).split(area);
    for (col, s) in cols.iter().zip(sections) {
        f.render_widget(Paragraph::new(section_lines(s, t, col.width)), *col);
    }
}

pub fn help(f: &mut Frame, area: Rect, t: &Theme) {
    // Làm mờ toàn bộ nền phía sau
    f.buffer_mut().set_style(area, Style::new().dim());

    let popup = centered(area, 96, 27);
    f.render_widget(Clear, popup);

    let block = Block::bordered()
        .title(Line::from(vec![
            Span::styled(" Help", Style::new().fg(t.accent).bold()),
            Span::styled(" · crab v0.1.0 ", Style::new().fg(t.tan)),
        ]))
        .border_style(Style::new().fg(t.accent))
        .padding(Padding::symmetric(2, 1))
        .style(Style::new().bg(t.bg).fg(t.text));
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    // "[esc] close" vẽ đè lên viền trên, căn phải
    let top = Rect::new(popup.x + 1, popup.y, popup.width.saturating_sub(2), 1);
    f.render_widget(
        Line::styled(" [esc] close ", Style::new().fg(t.accent)).right_aligned(),
        top,
    );

    let [filter, _, row1, _, row2, _, rule, tip] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(9),
        Constraint::Length(1),
        Constraint::Length(6),
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(inner);

    let input = Paragraph::new(Line::from(vec![
        Span::styled(" / ", Style::new().fg(t.accent).bold()),
        Span::styled("type to filter keys…", Style::new().fg(t.muted)),
    ]))
    .block(Block::bordered().border_style(Style::new().fg(t.border)));
    f.render_widget(input, filter);

    grid_row(f, row1, &SECTIONS[..3], t);
    grid_row(f, row2, &SECTIONS[3..], t);

    f.render_widget(
        Line::styled("─".repeat(rule.width as usize), Style::new().fg(t.border)),
        rule,
    );

    f.render_widget(
        Line::styled(
            "tip: every action previews the Cargo.toml diff before writing",
            Style::new().fg(t.muted),
        ),
        tip,
    );
}
