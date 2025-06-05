use std::{
    io,
    path::{Path, PathBuf},
    rc::Rc,
};

use anyhow::Result;
use monsterex::{
    keybinding::handle_key,
    monster::monster::MonsterEntity,
    state::{monster::MonsterListState, AppState},
    tui::Tui,
    ui::ui,
};

use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event},
    Terminal,
};
use serde_json::from_str;

fn main() -> Result<()> {
    let Ok(monster) = load_data() else {
        println!("load data error");
        std::process::exit(2);
    };

    let bundle: Vec<Rc<MonsterEntity>> = monster.into_iter().map(Rc::new).collect();
    let Some(monster_list_state) = MonsterListState::new(bundle) else {
        panic!("No Monster Data");
    };

    let monster_list_state = monster_list_state.path(get_asset_dir_path()?);

    // setup terimnal
    let mut tui = Tui::init()?;

    // create app instance
    let app = AppState {
        monster_list: monster_list_state,
        ..Default::default()
    };

    // println!("{:?}", app.monster_list.habitat_page_state);
    run_app(&mut tui.terminal, app)?;
    Ok(())
}

fn run_app<B>(terminal: &mut Terminal<B>, mut app: AppState) -> io::Result<()>
where
    B: Backend,
{
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(event) = event::read()? {
            if handle_key(event, &mut app).exit() {
                return Ok(());
            }
        }
    }
}

fn get_asset_dir_path() -> Result<PathBuf> {
    let asset_path = Path::new("icons");
    let current_dir = std::env::current_dir()?;
    Ok(current_dir.join(asset_path))
}

fn load_data() -> Result<Vec<MonsterEntity>, ()> {
    let monster = from_str(include_str!("../data/monster.json")).expect("load monster data error");
    Ok(monster)
}
