#![allow(dead_code, unused)]
use ratatui::style::Color;

pub struct GruvboxDark;
pub struct GruvboxLight;

impl GruvboxDark {
    pub const BG: Color = Color::Rgb(40, 40, 40);
    pub const RED: Color = Color::Rgb(204, 36, 29);
    pub const GREEN: Color = Color::Rgb(152, 151, 26);
    pub const YELLOW: Color = Color::Rgb(215, 153, 33);
    pub const BLUE: Color = Color::Rgb(69, 133, 136);
    pub const PURPLE: Color = Color::Rgb(177, 98, 134);
    pub const AQUA: Color = Color::Rgb(104, 157, 106);
    pub const GRAY: Color = Color::Rgb(168, 153, 132);

    pub const GRAY_ALT: Color = Color::Rgb(146, 131, 116);
    pub const RED_ALT: Color = Color::Rgb(251, 73, 52);
    pub const GREEN_ALT: Color = Color::Rgb(184, 187, 38);
    pub const YELLOW_ALT: Color = Color::Rgb(250, 189, 47);
    pub const BLUE_ALT: Color = Color::Rgb(131, 165, 152);
    pub const PURPLE_ALT: Color = Color::Rgb(211, 134, 155);
    pub const AQUA_ALT: Color = Color::Rgb(142, 192, 124);
    pub const FG: Color = Color::Rgb(235, 219, 178);

    pub const BG0_H: Color = Color::Rgb(29, 32, 33);
    pub const BG0: Color = Color::Rgb(40, 40, 40);
    pub const BG1: Color = Color::Rgb(60, 56, 54);
    pub const BG2: Color = Color::Rgb(80, 73, 69);
    pub const BG3: Color = Color::Rgb(102, 92, 84);
    pub const BG4: Color = Color::Rgb(124, 111, 100);
    pub const GRAY2: Color = Color::Rgb(146, 131, 116);
    pub const ORANGE: Color = Color::Rgb(214, 93, 14);

    pub const BG0_S: Color = Color::Rgb(50, 48, 47);
    pub const FG4: Color = Color::Rgb(168, 153, 132);
    pub const FG3: Color = Color::Rgb(189, 174, 147);
    pub const FG2: Color = Color::Rgb(213, 196, 161);
    pub const FG1: Color = Color::Rgb(235, 219, 178);
    pub const FG0: Color = Color::Rgb(251, 241, 199);
    pub const ORANGE_ALT: Color = Color::Rgb(254, 128, 25);
}

impl GruvboxLight {
    pub const BG: Color = Color::Rgb(251, 241, 199);
    pub const RED: Color = Color::Rgb(204, 36, 29);
    pub const GREEN: Color = Color::Rgb(152, 151, 26);
    pub const YELLOW: Color = Color::Rgb(215, 153, 33);
    pub const BLUE: Color = Color::Rgb(69, 133, 136);
    pub const PURPLE: Color = Color::Rgb(177, 98, 134);
    pub const AQUA: Color = Color::Rgb(104, 157, 106);
    pub const GRAY: Color = Color::Rgb(124, 108, 100);

    pub const GRAY_ALT: Color = Color::Rgb(146, 131, 116);
    pub const RED_ALT: Color = Color::Rgb(157, 0, 6);
    pub const GREEN_ALT: Color = Color::Rgb(121, 116, 14);
    pub const YELLOW_ALT: Color = Color::Rgb(181, 119, 100);
    pub const BLUE_ALT: Color = Color::Rgb(7, 102, 120);
    pub const PURPLE_ALT: Color = Color::Rgb(143, 63, 113);
    pub const AQUA_ALT: Color = Color::Rgb(66, 123, 88);
    pub const FG: Color = Color::Rgb(60, 56, 54);

    pub const BG0_H: Color = Color::Rgb(249, 245, 215);
    pub const BG0: Color = Color::Rgb(251, 241, 199);
    pub const BG1: Color = Color::Rgb(235, 219, 178);
    pub const BG2: Color = Color::Rgb(213, 196, 161);
    pub const BG3: Color = Color::Rgb(189, 174, 147);
    pub const BG4: Color = Color::Rgb(168, 153, 132);
    pub const GRAY2: Color = Color::Rgb(146, 131, 116);
    pub const ORANGE: Color = Color::Rgb(214, 93, 14);

    pub const BG0_S: Color = Color::Rgb(242, 229, 188);
    pub const FG4: Color = Color::Rgb(124, 108, 100);
    pub const FG3: Color = Color::Rgb(102, 92, 84);
    pub const FG2: Color = Color::Rgb(80, 73, 69);
    pub const FG1: Color = Color::Rgb(60, 56, 54);
    pub const FG0: Color = Color::Rgb(40, 40, 40);
    pub const ORANGE_ALT: Color = Color::Rgb(175, 58, 3);
}
