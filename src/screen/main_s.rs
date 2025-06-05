use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::{state::AppState, widget::menu_selection::MenuSelection};

pub struct MainScreen;

pub fn render_main_screen(f: &mut Frame<'_>, chunk: Rect, state: &mut AppState) {
    let title_monsterdex = Paragraph::new(
        "
███╗   ███╗ █████╗ ███╗  ██╗ ██████╗████████╗███████╗██████╗ ██████╗ ███████╗██╗  ██╗
████╗ ████║██╔══██╗████╗ ██║██╔════╝╚══██╔══╝██╔════╝██╔══██╗██╔══██╗██╔════╝╚██╗██╔╝
██╔████╔██║██║  ██║██╔██╗██║╚█████╗    ██║   █████╗  ██████╔╝██║  ██║█████╗   ╚███╔╝ 
██║╚██╔╝██║██║  ██║██║╚████║ ╚═══██╗   ██║   ██╔══╝  ██╔══██╗██║  ██║██╔══╝   ██╔██╗ 
██║ ╚═╝ ██║╚█████╔╝██║ ╚███║██████╔╝   ██║   ███████╗██║  ██║██████╔╝███████╗██╔╝╚██╗
╚═╝     ╚═╝ ╚════╝ ╚═╝  ╚══╝╚═════╝    ╚═╝   ╚══════╝╚═╝  ╚═╝╚═════╝ ╚══════╝╚═╝  ╚═╝
        ",
    );

    let [title, main_menu_chunk] = Layout::vertical([Constraint::Max(10), Constraint::Min(10)])
        .margin(5)
        .areas(chunk);

    // render Menu Selection on Main Screen
    f.render_widget(title_monsterdex, title);
    f.render_stateful_widget(MenuSelection, main_menu_chunk, state);
}
