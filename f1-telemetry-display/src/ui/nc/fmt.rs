use std::time::{SystemTime, UNIX_EPOCH};

use ncurses::*;

use f1_telemetry::packet::generic::{Team, TyreCompoundVisual};

const PERCENTAGE_BAR_SLICES: i8 = 20;

#[allow(dead_code)]
pub enum Color {
    Black = COLOR_BLACK as isize,
    Red = COLOR_RED as isize,
    Green = COLOR_GREEN as isize,
    Yellow = COLOR_YELLOW as isize,
    Blue = COLOR_BLUE as isize,
    Magenta = COLOR_MAGENTA as isize,
    Cyan = COLOR_CYAN as isize,
    White = COLOR_WHITE as isize,

    StatusOk = 100,
    StatusCaution,
    StatusWarning,
    StatusDanger,

    TyreSoft,
    TyreMedium,
    TyreHard,
    TyreIntermediate,
    TyreWet,
    TyreOther,

    Mercedes,
    Ferrari,
    RedBullRacing,
    Williams,
    RacingPoint,
    Renault,
    ToroRosso,
    Haas,
    McLaren,
    AlfaRomeo,
    AlphaTauri,
    Alpine,
    AstonMartin,
    MyTeam,
}

trait ToColor {
    fn get_color(&self) -> Color;
}

impl ToColor for Team {
    fn get_color(&self) -> Color {
        match self {
            Team::Mercedes => Color::Mercedes,
            Team::Ferrari => Color::Ferrari,
            Team::RedBullRacing => Color::RedBullRacing,
            Team::Williams => Color::Williams,
            Team::RacingPoint => Color::RacingPoint,
            Team::Renault => Color::Renault,
            Team::ToroRosso => Color::ToroRosso,
            Team::Haas => Color::Haas,
            Team::McLaren => Color::McLaren,
            Team::AlfaRomeo => Color::AlfaRomeo,
            Team::AlphaTauri => Color::AlphaTauri,
            Team::Alpine => Color::Alpine,
            Team::AstonMartin => Color::AstonMartin,
            Team::MyTeam => Color::MyTeam,
            _ => Color::Black,
        }
    }
}

impl ToColor for TyreCompoundVisual {
    fn get_color(&self) -> Color {
        match self {
            TyreCompoundVisual::Hard => Color::TyreHard,
            TyreCompoundVisual::Medium => Color::TyreMedium,
            TyreCompoundVisual::Soft => Color::TyreSoft,
            TyreCompoundVisual::Inter => Color::TyreIntermediate,
            TyreCompoundVisual::Wet => Color::TyreWet,
            _ => Color::TyreOther,
        }
    }
}

pub fn init_colors() {
    start_color();

    init_base_color_pairs();
    init_team_colors();
    init_status_colors();
    init_tyre_colors();
}

fn init_base_color_pairs() {
    init_pair(COLOR_BLACK, COLOR_BLACK, COLOR_BLACK);
    init_pair(COLOR_RED, COLOR_RED, COLOR_BLACK);
    init_pair(COLOR_GREEN, COLOR_GREEN, COLOR_BLACK);
    init_pair(COLOR_YELLOW, COLOR_YELLOW, COLOR_BLACK);
    init_pair(COLOR_BLUE, COLOR_BLUE, COLOR_BLACK);
    init_pair(COLOR_MAGENTA, COLOR_MAGENTA, COLOR_BLACK);
    init_pair(COLOR_CYAN, COLOR_CYAN, COLOR_BLACK);
    init_pair(COLOR_WHITE, COLOR_WHITE, COLOR_BLACK);
}

fn init_team_colors() {
    for (t, c) in vec![
        (Color::Mercedes, (0, 210, 190)),
        (Color::Ferrari, (220, 0, 0)),
        (Color::RedBullRacing, (60, 0, 255)),
        (Color::Williams, (0, 128, 255)),
        (Color::RacingPoint, (245, 150, 200)),
        (Color::Renault, (255, 245, 0)),
        (Color::ToroRosso, (70, 155, 255)),
        (Color::Haas, (119, 119, 119)),
        (Color::McLaren, (255, 135, 0)),
        (Color::AlfaRomeo, (155, 0, 0)),
        (Color::AlphaTauri, (255, 255, 255)),
        (Color::Alpine, (0, 144, 255)),
        (Color::AstonMartin, (0, 111, 98)),
        (Color::MyTeam, (118, 0, 218)),
    ] {
        let idx = t as i16;
        let (r, g, b) = rgb_to_curses(c);
        init_color(idx, r, g, b);
        init_pair(idx, COLOR_WHITE, idx);
    }
}

#[inline]
fn rgb_to_curses(c: (i16, i16, i16)) -> (i16, i16, i16) {
    let r = (c.0 as f32 / 255.0 * 500.0) as i16;
    let g = (c.1 as f32 / 255.0 * 500.0) as i16;
    let b = (c.2 as f32 / 255.0 * 500.0) as i16;

    (r, g, b)
}

fn init_status_colors() {
    init_color(Color::StatusWarning as i16, 1000, 812, 686);

    for (status, c) in vec![
        (Color::StatusOk, Color::Green),
        (Color::StatusCaution, Color::Yellow),
        (Color::StatusWarning, Color::StatusWarning),
        (Color::StatusDanger, Color::Red),
    ] {
        init_pair(status as i16, c as i16, COLOR_BLACK);
    }
}

fn init_tyre_colors() {
    for (t, c) in vec![
        (Color::TyreHard, (1000, 1000, 1000)),
        (Color::TyreMedium, (863, 804, 196)),
        (Color::TyreSoft, (953, 290, 200)),
        (Color::TyreIntermediate, (514, 812, 294)),
        (Color::TyreWet, (114, 435, 733)),
        (Color::TyreOther, (500, 500, 500)),
    ] {
        let idx = t as i16;
        init_color(idx, c.0, c.1, c.2);
        init_pair(idx, idx, COLOR_BLACK);
    }
}

pub fn set_bold() {
    attron(A_BOLD());
}

pub fn wset_bold(w: WINDOW) {
    wattron(w, A_BOLD());
}

pub fn set_team_color(w: WINDOW, team: Team) {
    wcolor_set(w, team.get_color() as i16);
}

pub fn set_tyre_color(w: WINDOW, tyre_compound: TyreCompoundVisual) {
    wcolor_set(w, tyre_compound.get_color() as i16);
}

pub fn set_color(w: Option<WINDOW>, c: i16) {
    match w {
        Some(w) => wcolor_set(w, c),
        None => color_set(c),
    };
}

pub fn reset_color(w: Option<WINDOW>) {
    match w {
        Some(w) => wattroff(w, A_COLOR()),
        None => attroff(A_COLOR()),
    };
}

pub fn set_damage_color(w: Option<WINDOW>, damage_pct: u8, ok: u8, caution: u8, warning: u8) {
    let c = match damage_pct {
        d if d <= ok => Color::StatusOk,
        d if d <= caution => Color::StatusCaution,
        d if d <= warning => Color::StatusWarning,
        _ => Color::StatusDanger,
    };

    set_color(w, c as i16);
}

pub fn set_lap_time_color(w: Option<WINDOW>, last: u32, personal_best: u32, session_best: u32) {
    let c = if last == 0 {
        Color::White
    } else if last <= session_best {
        Color::Magenta
    } else if last <= personal_best {
        Color::Green
    } else {
        Color::Red
    };

    set_color(w, c as i16);
}

pub fn reset() {
    attrset(0);
}

pub fn wreset(w: WINDOW) {
    wattrset(w, 0);
}

pub fn format_perc_bar(perc_value: f32) -> String {
    let bars = 100 / PERCENTAGE_BAR_SLICES;
    let used_bars = (perc_value * 100.0) as i8 / bars;
    format!("{: <20}", (0..used_bars).map(|_| "|").collect::<String>())
}

pub fn center(hwnd: WINDOW, s: &str) -> i32 {
    let w = getmaxx(hwnd);
    (w - s.len() as i32) / 2
}

pub(crate) fn blink() -> bool {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % 2)
        == 0
}

pub fn blink_colour(c1: i16, c2: i16) {
    if blink() {
        set_color(None, c1);
    } else {
        set_color(None, c2)
    }
}
