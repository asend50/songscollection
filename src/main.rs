/*
By: <Asen Doiron>
Date: 2026-04-20
Program Details: <The purpose of this program is to create a simple song collection where the user can add songs, remove songs, sort songs, and select a random song.>
*/

mod modules;
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::listview::ListView;
use crate::modules::text_button::TextButton;
use crate::modules::text_input::TextInput;
use macroquad::prelude::*;
use miniquad::date;


/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "songscollection".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

fn sub_disable(btn_sub: &mut TextButton, btn_clear: &mut TextButton) -> () {
    btn_sub.enabled = false;
    btn_clear.enabled = false;
}

fn sub_enable(btn_sub: &mut TextButton, btn_clear: &mut TextButton) -> () {
    btn_sub.enabled = true;
    btn_clear.enabled = true;
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(date::now() as u64); 
    
    let lightblue = Color::new(0.678, 0.847, 0.902, 1.0);

    let lesslightblue = Color::new(0.450, 0.650, 0.850, 1.0);

    let backgroundblue = Color::new(0.550, 0.800, 0.970, 1.0);

    let selectionblue = Color::new(0.550, 0.900, 0.999, 1.0);

    let buttonblue = Color::new(0.550, 0.900, 0.999, 1.0);

    let purple = Color::new(0.500, 0.300, 0.999, 1.0);

    let buttonselectblue = Color::new(0.550, 0.950, 0.999, 1.0);

    let lightred = Color::new(0.999, 0.350, 0.350, 1.0);

    let lightgreen = Color::new(0.500, 0.999, 0.450, 1.0);

    let mut btn_add = TextButton::new(610.0, 350.0, 50.0, 50.0, "+", buttonblue, buttonselectblue, 30);
    btn_add.with_text_color(BLACK);

    let mut btn_sub = TextButton::new(690.0, 350.0, 50.0, 50.0, "-", lightred, RED, 30);
    btn_sub.with_text_color(BLACK);

    let mut btn_clear = TextButton::new(775.0, 350.0, 75.0, 50.0, "Clear", lightgreen, GREEN, 30);
    btn_clear.with_text_color(BLACK);

    let mut btn_sortAZ = TextButton::new(350.0, 290.0, 125.0, 50.0, "A-Z", purple, PURPLE, 30);
    btn_sortAZ.with_text_color(BLACK);

    let mut btn_sortZA = TextButton::new(500.0, 290.0, 125.0, 50.0, "Z-A", purple, PURPLE, 30);
    btn_sortZA.with_text_color(BLACK);

    let mut btn_random = TextButton::new(650.0, 290.0, 125.0, 50.0, "Random", purple, PURPLE, 30);
    btn_random.with_text_color(BLACK);

    let mut btn_exit = TextButton::new(800.0, 290.0, 50.0, 50.0, "X", lightred, RED, 30);
    btn_exit.with_text_color(BLACK);

   

    let mut lbl_songs = Label::new("", 350.0, 280.0, 30);

    let mut listview = ListView::new(
        &vec![
            "Let It Happen".to_string(),
            "Bohemian Rhapsody".to_string(),
            "A Little Death".to_string(),
            "Falling Down".to_string(),
            "The Winner Takes It All".to_string(),
        ],
        350.0,
        450.0,
        50,
    );
    listview.with_colors(BLACK, Some(backgroundblue), Some(selectionblue));
    listview.with_border(lesslightblue, 6.0);
    
    let mut txt_input = TextInput::new(350.0, 350.0, 250.0, 40.0, 25.0);

    let mut songs = vec![
        "Let It Happen".to_string(),
        "Bohemian Rhapsody".to_string(),
        "A Little Death".to_string(),
        "Falling Down".to_string(),
        "The Winner Takes It All".to_string(),
    ];

    for spot in 0..songs.len() {
        if songs[spot] == txt_input.get_text() {
            songs.remove(spot);
            break;
        }
    }

    let mut index =rand::gen_range(0, songs.len());

    btn_sub.enabled = true;

    loop {
        clear_background(lightblue);

        listview.draw();

        if btn_exit.click() {
            break;
        }

        if btn_add.click() {
            songs.push(txt_input.get_text());
            listview.clear();
            listview.add_items(&songs);
        }

        if btn_sub.click() {
            songs.remove(songs.len() - 1);
            listview.clear();
            listview.add_items(&songs);
        }

        if songs.is_empty() {
            lbl_songs.set_text("No Songs");
        } else {
            lbl_songs.set_text(&format!("{} Songs", songs.len()));
        }

        if songs.len() == 0 {
            sub_disable(&mut btn_sub, &mut btn_clear);
        }

        if songs.len() > 0 {
            sub_enable(&mut btn_sub, &mut btn_clear);
        }

        if songs.len() > 0 {
            btn_sortAZ.enabled = true;
            btn_sortZA.enabled = true;
            btn_random.enabled = true;
            btn_clear.enabled = true;
        } else {
            btn_sortAZ.enabled = false;
            btn_sortZA.enabled = false;
            btn_random.enabled = false;
            btn_clear.enabled = false;
        }

        if txt_input.get_text().is_empty() {
            btn_add.enabled = false;
        } else {
            btn_add.enabled = true;
        }

        if btn_sortAZ.click() {
            songs.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            listview.clear();
            listview.add_items(&songs);
        }

        if btn_sortZA.click() {
            songs.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            songs.reverse();
            listview.clear();
            listview.add_items(&songs);
        }

        if btn_random.click() {
           listview.select_item(Some(index));
           index = rand::gen_range(0, songs.len());
        }
        

        if btn_clear.click() {
            songs.clear();
            listview.clear();
        }

        lbl_songs.draw();
        txt_input.draw();
        next_frame().await;
    }
}
