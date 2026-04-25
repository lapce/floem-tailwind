use super::colors;
use floem::style::Style;

pub trait TailwindColorExt {
    fn bg_neutral_50(self) -> Self;
    fn bg_neutral_100(self) -> Self;
    fn bg_neutral_200(self) -> Self;
    fn bg_neutral_300(self) -> Self;
    fn bg_neutral_400(self) -> Self;
    fn bg_neutral_500(self) -> Self;
    fn bg_neutral_600(self) -> Self;
    fn bg_neutral_700(self) -> Self;
    fn bg_neutral_800(self) -> Self;
    fn bg_neutral_900(self) -> Self;
    fn bg_neutral_950(self) -> Self;
    fn bg_stone_50(self) -> Self;
    fn bg_stone_100(self) -> Self;
    fn bg_stone_200(self) -> Self;
    fn bg_stone_300(self) -> Self;
    fn bg_stone_400(self) -> Self;
    fn bg_stone_500(self) -> Self;
    fn bg_stone_600(self) -> Self;
    fn bg_stone_700(self) -> Self;
    fn bg_stone_800(self) -> Self;
    fn bg_stone_900(self) -> Self;
    fn bg_stone_950(self) -> Self;
    fn bg_amber_50(self) -> Self;
    fn bg_amber_100(self) -> Self;
    fn bg_amber_200(self) -> Self;
    fn bg_amber_300(self) -> Self;
    fn bg_amber_400(self) -> Self;
    fn bg_amber_500(self) -> Self;
    fn bg_amber_600(self) -> Self;
    fn bg_amber_700(self) -> Self;
    fn bg_amber_800(self) -> Self;
    fn bg_amber_900(self) -> Self;
    fn bg_amber_950(self) -> Self;
    fn bg_lime_50(self) -> Self;
    fn bg_lime_100(self) -> Self;
    fn bg_lime_200(self) -> Self;
    fn bg_lime_300(self) -> Self;
    fn bg_lime_400(self) -> Self;
    fn bg_lime_500(self) -> Self;
    fn bg_lime_600(self) -> Self;
    fn bg_lime_700(self) -> Self;
    fn bg_lime_800(self) -> Self;
    fn bg_lime_900(self) -> Self;
    fn bg_lime_950(self) -> Self;
    fn bg_green_50(self) -> Self;
    fn bg_green_100(self) -> Self;
    fn bg_green_200(self) -> Self;
    fn bg_green_300(self) -> Self;
    fn bg_green_400(self) -> Self;
    fn bg_green_500(self) -> Self;
    fn bg_green_600(self) -> Self;
    fn bg_green_700(self) -> Self;
    fn bg_green_800(self) -> Self;
    fn bg_green_900(self) -> Self;
    fn bg_green_950(self) -> Self;
    fn bg_emerald_50(self) -> Self;
    fn bg_emerald_100(self) -> Self;
    fn bg_emerald_200(self) -> Self;
    fn bg_emerald_300(self) -> Self;
    fn bg_emerald_400(self) -> Self;
    fn bg_emerald_500(self) -> Self;
    fn bg_emerald_600(self) -> Self;
    fn bg_emerald_700(self) -> Self;
    fn bg_emerald_800(self) -> Self;
    fn bg_emerald_900(self) -> Self;
    fn bg_emerald_950(self) -> Self;
    fn bg_teal_50(self) -> Self;
    fn bg_teal_100(self) -> Self;
    fn bg_teal_200(self) -> Self;
    fn bg_teal_300(self) -> Self;
    fn bg_teal_400(self) -> Self;
    fn bg_teal_500(self) -> Self;
    fn bg_teal_600(self) -> Self;
    fn bg_teal_700(self) -> Self;
    fn bg_teal_800(self) -> Self;
    fn bg_teal_900(self) -> Self;
    fn bg_teal_950(self) -> Self;
    fn bg_cyan_50(self) -> Self;
    fn bg_cyan_100(self) -> Self;
    fn bg_cyan_200(self) -> Self;
    fn bg_cyan_300(self) -> Self;
    fn bg_cyan_400(self) -> Self;
    fn bg_cyan_500(self) -> Self;
    fn bg_cyan_600(self) -> Self;
    fn bg_cyan_700(self) -> Self;
    fn bg_cyan_800(self) -> Self;
    fn bg_cyan_900(self) -> Self;
    fn bg_cyan_950(self) -> Self;
    fn bg_sky_50(self) -> Self;
    fn bg_sky_100(self) -> Self;
    fn bg_sky_200(self) -> Self;
    fn bg_sky_300(self) -> Self;
    fn bg_sky_400(self) -> Self;
    fn bg_sky_500(self) -> Self;
    fn bg_sky_600(self) -> Self;
    fn bg_sky_700(self) -> Self;
    fn bg_sky_800(self) -> Self;
    fn bg_sky_900(self) -> Self;
    fn bg_sky_950(self) -> Self;
    fn bg_blue_50(self) -> Self;
    fn bg_blue_100(self) -> Self;
    fn bg_blue_200(self) -> Self;
    fn bg_blue_300(self) -> Self;
    fn bg_blue_400(self) -> Self;
    fn bg_blue_500(self) -> Self;
    fn bg_blue_600(self) -> Self;
    fn bg_blue_700(self) -> Self;
    fn bg_blue_800(self) -> Self;
    fn bg_blue_900(self) -> Self;
    fn bg_blue_950(self) -> Self;
    fn bg_indigo_50(self) -> Self;
    fn bg_indigo_100(self) -> Self;
    fn bg_indigo_200(self) -> Self;
    fn bg_indigo_300(self) -> Self;
    fn bg_indigo_400(self) -> Self;
    fn bg_indigo_500(self) -> Self;
    fn bg_indigo_600(self) -> Self;
    fn bg_indigo_700(self) -> Self;
    fn bg_indigo_800(self) -> Self;
    fn bg_indigo_900(self) -> Self;
    fn bg_indigo_950(self) -> Self;
    fn bg_violet_50(self) -> Self;
    fn bg_violet_100(self) -> Self;
    fn bg_violet_200(self) -> Self;
    fn bg_violet_300(self) -> Self;
    fn bg_violet_400(self) -> Self;
    fn bg_violet_500(self) -> Self;
    fn bg_violet_600(self) -> Self;
    fn bg_violet_700(self) -> Self;
    fn bg_violet_800(self) -> Self;
    fn bg_violet_900(self) -> Self;
    fn bg_violet_950(self) -> Self;
    fn bg_purple_50(self) -> Self;
    fn bg_purple_100(self) -> Self;
    fn bg_purple_200(self) -> Self;
    fn bg_purple_300(self) -> Self;
    fn bg_purple_400(self) -> Self;
    fn bg_purple_500(self) -> Self;
    fn bg_purple_600(self) -> Self;
    fn bg_purple_700(self) -> Self;
    fn bg_purple_800(self) -> Self;
    fn bg_purple_900(self) -> Self;
    fn bg_purple_950(self) -> Self;
    fn bg_fuchsia_50(self) -> Self;
    fn bg_fuchsia_100(self) -> Self;
    fn bg_fuchsia_200(self) -> Self;
    fn bg_fuchsia_300(self) -> Self;
    fn bg_fuchsia_400(self) -> Self;
    fn bg_fuchsia_500(self) -> Self;
    fn bg_fuchsia_600(self) -> Self;
    fn bg_fuchsia_700(self) -> Self;
    fn bg_fuchsia_800(self) -> Self;
    fn bg_fuchsia_900(self) -> Self;
    fn bg_fuchsia_950(self) -> Self;
    fn bg_pink_50(self) -> Self;
    fn bg_pink_100(self) -> Self;
    fn bg_pink_200(self) -> Self;
    fn bg_pink_300(self) -> Self;
    fn bg_pink_400(self) -> Self;
    fn bg_pink_500(self) -> Self;
    fn bg_pink_600(self) -> Self;
    fn bg_pink_700(self) -> Self;
    fn bg_pink_800(self) -> Self;
    fn bg_pink_900(self) -> Self;
    fn bg_pink_950(self) -> Self;
    fn bg_rose_50(self) -> Self;
    fn bg_rose_100(self) -> Self;
    fn bg_rose_200(self) -> Self;
    fn bg_rose_300(self) -> Self;
    fn bg_rose_400(self) -> Self;
    fn bg_rose_500(self) -> Self;
    fn bg_rose_600(self) -> Self;
    fn bg_rose_700(self) -> Self;
    fn bg_rose_800(self) -> Self;
    fn bg_rose_900(self) -> Self;
    fn bg_rose_950(self) -> Self;
    fn text_neutral_50(self) -> Self;
    fn text_neutral_100(self) -> Self;
    fn text_neutral_200(self) -> Self;
    fn text_neutral_300(self) -> Self;
    fn text_neutral_400(self) -> Self;
    fn text_neutral_500(self) -> Self;
    fn text_neutral_600(self) -> Self;
    fn text_neutral_700(self) -> Self;
    fn text_neutral_800(self) -> Self;
    fn text_neutral_900(self) -> Self;
    fn text_neutral_950(self) -> Self;
    fn text_stone_50(self) -> Self;
    fn text_stone_100(self) -> Self;
    fn text_stone_200(self) -> Self;
    fn text_stone_300(self) -> Self;
    fn text_stone_400(self) -> Self;
    fn text_stone_500(self) -> Self;
    fn text_stone_600(self) -> Self;
    fn text_stone_700(self) -> Self;
    fn text_stone_800(self) -> Self;
    fn text_stone_900(self) -> Self;
    fn text_stone_950(self) -> Self;
    fn text_amber_50(self) -> Self;
    fn text_amber_100(self) -> Self;
    fn text_amber_200(self) -> Self;
    fn text_amber_300(self) -> Self;
    fn text_amber_400(self) -> Self;
    fn text_amber_500(self) -> Self;
    fn text_amber_600(self) -> Self;
    fn text_amber_700(self) -> Self;
    fn text_amber_800(self) -> Self;
    fn text_amber_900(self) -> Self;
    fn text_amber_950(self) -> Self;
    fn text_lime_50(self) -> Self;
    fn text_lime_100(self) -> Self;
    fn text_lime_200(self) -> Self;
    fn text_lime_300(self) -> Self;
    fn text_lime_400(self) -> Self;
    fn text_lime_500(self) -> Self;
    fn text_lime_600(self) -> Self;
    fn text_lime_700(self) -> Self;
    fn text_lime_800(self) -> Self;
    fn text_lime_900(self) -> Self;
    fn text_lime_950(self) -> Self;
    fn text_green_50(self) -> Self;
    fn text_green_100(self) -> Self;
    fn text_green_200(self) -> Self;
    fn text_green_300(self) -> Self;
    fn text_green_400(self) -> Self;
    fn text_green_500(self) -> Self;
    fn text_green_600(self) -> Self;
    fn text_green_700(self) -> Self;
    fn text_green_800(self) -> Self;
    fn text_green_900(self) -> Self;
    fn text_green_950(self) -> Self;
    fn text_emerald_50(self) -> Self;
    fn text_emerald_100(self) -> Self;
    fn text_emerald_200(self) -> Self;
    fn text_emerald_300(self) -> Self;
    fn text_emerald_400(self) -> Self;
    fn text_emerald_500(self) -> Self;
    fn text_emerald_600(self) -> Self;
    fn text_emerald_700(self) -> Self;
    fn text_emerald_800(self) -> Self;
    fn text_emerald_900(self) -> Self;
    fn text_emerald_950(self) -> Self;
    fn text_teal_50(self) -> Self;
    fn text_teal_100(self) -> Self;
    fn text_teal_200(self) -> Self;
    fn text_teal_300(self) -> Self;
    fn text_teal_400(self) -> Self;
    fn text_teal_500(self) -> Self;
    fn text_teal_600(self) -> Self;
    fn text_teal_700(self) -> Self;
    fn text_teal_800(self) -> Self;
    fn text_teal_900(self) -> Self;
    fn text_teal_950(self) -> Self;
    fn text_cyan_50(self) -> Self;
    fn text_cyan_100(self) -> Self;
    fn text_cyan_200(self) -> Self;
    fn text_cyan_300(self) -> Self;
    fn text_cyan_400(self) -> Self;
    fn text_cyan_500(self) -> Self;
    fn text_cyan_600(self) -> Self;
    fn text_cyan_700(self) -> Self;
    fn text_cyan_800(self) -> Self;
    fn text_cyan_900(self) -> Self;
    fn text_cyan_950(self) -> Self;
    fn text_sky_50(self) -> Self;
    fn text_sky_100(self) -> Self;
    fn text_sky_200(self) -> Self;
    fn text_sky_300(self) -> Self;
    fn text_sky_400(self) -> Self;
    fn text_sky_500(self) -> Self;
    fn text_sky_600(self) -> Self;
    fn text_sky_700(self) -> Self;
    fn text_sky_800(self) -> Self;
    fn text_sky_900(self) -> Self;
    fn text_sky_950(self) -> Self;
    fn text_blue_50(self) -> Self;
    fn text_blue_100(self) -> Self;
    fn text_blue_200(self) -> Self;
    fn text_blue_300(self) -> Self;
    fn text_blue_400(self) -> Self;
    fn text_blue_500(self) -> Self;
    fn text_blue_600(self) -> Self;
    fn text_blue_700(self) -> Self;
    fn text_blue_800(self) -> Self;
    fn text_blue_900(self) -> Self;
    fn text_blue_950(self) -> Self;
    fn text_indigo_50(self) -> Self;
    fn text_indigo_100(self) -> Self;
    fn text_indigo_200(self) -> Self;
    fn text_indigo_300(self) -> Self;
    fn text_indigo_400(self) -> Self;
    fn text_indigo_500(self) -> Self;
    fn text_indigo_600(self) -> Self;
    fn text_indigo_700(self) -> Self;
    fn text_indigo_800(self) -> Self;
    fn text_indigo_900(self) -> Self;
    fn text_indigo_950(self) -> Self;
    fn text_violet_50(self) -> Self;
    fn text_violet_100(self) -> Self;
    fn text_violet_200(self) -> Self;
    fn text_violet_300(self) -> Self;
    fn text_violet_400(self) -> Self;
    fn text_violet_500(self) -> Self;
    fn text_violet_600(self) -> Self;
    fn text_violet_700(self) -> Self;
    fn text_violet_800(self) -> Self;
    fn text_violet_900(self) -> Self;
    fn text_violet_950(self) -> Self;
    fn text_purple_50(self) -> Self;
    fn text_purple_100(self) -> Self;
    fn text_purple_200(self) -> Self;
    fn text_purple_300(self) -> Self;
    fn text_purple_400(self) -> Self;
    fn text_purple_500(self) -> Self;
    fn text_purple_600(self) -> Self;
    fn text_purple_700(self) -> Self;
    fn text_purple_800(self) -> Self;
    fn text_purple_900(self) -> Self;
    fn text_purple_950(self) -> Self;
    fn text_fuchsia_50(self) -> Self;
    fn text_fuchsia_100(self) -> Self;
    fn text_fuchsia_200(self) -> Self;
    fn text_fuchsia_300(self) -> Self;
    fn text_fuchsia_400(self) -> Self;
    fn text_fuchsia_500(self) -> Self;
    fn text_fuchsia_600(self) -> Self;
    fn text_fuchsia_700(self) -> Self;
    fn text_fuchsia_800(self) -> Self;
    fn text_fuchsia_900(self) -> Self;
    fn text_fuchsia_950(self) -> Self;
    fn text_pink_50(self) -> Self;
    fn text_pink_100(self) -> Self;
    fn text_pink_200(self) -> Self;
    fn text_pink_300(self) -> Self;
    fn text_pink_400(self) -> Self;
    fn text_pink_500(self) -> Self;
    fn text_pink_600(self) -> Self;
    fn text_pink_700(self) -> Self;
    fn text_pink_800(self) -> Self;
    fn text_pink_900(self) -> Self;
    fn text_pink_950(self) -> Self;
    fn text_rose_50(self) -> Self;
    fn text_rose_100(self) -> Self;
    fn text_rose_200(self) -> Self;
    fn text_rose_300(self) -> Self;
    fn text_rose_400(self) -> Self;
    fn text_rose_500(self) -> Self;
    fn text_rose_600(self) -> Self;
    fn text_rose_700(self) -> Self;
    fn text_rose_800(self) -> Self;
    fn text_rose_900(self) -> Self;
    fn text_rose_950(self) -> Self;
    fn border_neutral_50(self) -> Self;
    fn border_neutral_100(self) -> Self;
    fn border_neutral_200(self) -> Self;
    fn border_neutral_300(self) -> Self;
    fn border_neutral_400(self) -> Self;
    fn border_neutral_500(self) -> Self;
    fn border_neutral_600(self) -> Self;
    fn border_neutral_700(self) -> Self;
    fn border_neutral_800(self) -> Self;
    fn border_neutral_900(self) -> Self;
    fn border_neutral_950(self) -> Self;
    fn border_stone_50(self) -> Self;
    fn border_stone_100(self) -> Self;
    fn border_stone_200(self) -> Self;
    fn border_stone_300(self) -> Self;
    fn border_stone_400(self) -> Self;
    fn border_stone_500(self) -> Self;
    fn border_stone_600(self) -> Self;
    fn border_stone_700(self) -> Self;
    fn border_stone_800(self) -> Self;
    fn border_stone_900(self) -> Self;
    fn border_stone_950(self) -> Self;
    fn border_amber_50(self) -> Self;
    fn border_amber_100(self) -> Self;
    fn border_amber_200(self) -> Self;
    fn border_amber_300(self) -> Self;
    fn border_amber_400(self) -> Self;
    fn border_amber_500(self) -> Self;
    fn border_amber_600(self) -> Self;
    fn border_amber_700(self) -> Self;
    fn border_amber_800(self) -> Self;
    fn border_amber_900(self) -> Self;
    fn border_amber_950(self) -> Self;
    fn border_lime_50(self) -> Self;
    fn border_lime_100(self) -> Self;
    fn border_lime_200(self) -> Self;
    fn border_lime_300(self) -> Self;
    fn border_lime_400(self) -> Self;
    fn border_lime_500(self) -> Self;
    fn border_lime_600(self) -> Self;
    fn border_lime_700(self) -> Self;
    fn border_lime_800(self) -> Self;
    fn border_lime_900(self) -> Self;
    fn border_lime_950(self) -> Self;
    fn border_green_50(self) -> Self;
    fn border_green_100(self) -> Self;
    fn border_green_200(self) -> Self;
    fn border_green_300(self) -> Self;
    fn border_green_400(self) -> Self;
    fn border_green_500(self) -> Self;
    fn border_green_600(self) -> Self;
    fn border_green_700(self) -> Self;
    fn border_green_800(self) -> Self;
    fn border_green_900(self) -> Self;
    fn border_green_950(self) -> Self;
    fn border_emerald_50(self) -> Self;
    fn border_emerald_100(self) -> Self;
    fn border_emerald_200(self) -> Self;
    fn border_emerald_300(self) -> Self;
    fn border_emerald_400(self) -> Self;
    fn border_emerald_500(self) -> Self;
    fn border_emerald_600(self) -> Self;
    fn border_emerald_700(self) -> Self;
    fn border_emerald_800(self) -> Self;
    fn border_emerald_900(self) -> Self;
    fn border_emerald_950(self) -> Self;
    fn border_teal_50(self) -> Self;
    fn border_teal_100(self) -> Self;
    fn border_teal_200(self) -> Self;
    fn border_teal_300(self) -> Self;
    fn border_teal_400(self) -> Self;
    fn border_teal_500(self) -> Self;
    fn border_teal_600(self) -> Self;
    fn border_teal_700(self) -> Self;
    fn border_teal_800(self) -> Self;
    fn border_teal_900(self) -> Self;
    fn border_teal_950(self) -> Self;
    fn border_cyan_50(self) -> Self;
    fn border_cyan_100(self) -> Self;
    fn border_cyan_200(self) -> Self;
    fn border_cyan_300(self) -> Self;
    fn border_cyan_400(self) -> Self;
    fn border_cyan_500(self) -> Self;
    fn border_cyan_600(self) -> Self;
    fn border_cyan_700(self) -> Self;
    fn border_cyan_800(self) -> Self;
    fn border_cyan_900(self) -> Self;
    fn border_cyan_950(self) -> Self;
    fn border_sky_50(self) -> Self;
    fn border_sky_100(self) -> Self;
    fn border_sky_200(self) -> Self;
    fn border_sky_300(self) -> Self;
    fn border_sky_400(self) -> Self;
    fn border_sky_500(self) -> Self;
    fn border_sky_600(self) -> Self;
    fn border_sky_700(self) -> Self;
    fn border_sky_800(self) -> Self;
    fn border_sky_900(self) -> Self;
    fn border_sky_950(self) -> Self;
    fn border_blue_50(self) -> Self;
    fn border_blue_100(self) -> Self;
    fn border_blue_200(self) -> Self;
    fn border_blue_300(self) -> Self;
    fn border_blue_400(self) -> Self;
    fn border_blue_500(self) -> Self;
    fn border_blue_600(self) -> Self;
    fn border_blue_700(self) -> Self;
    fn border_blue_800(self) -> Self;
    fn border_blue_900(self) -> Self;
    fn border_blue_950(self) -> Self;
    fn border_indigo_50(self) -> Self;
    fn border_indigo_100(self) -> Self;
    fn border_indigo_200(self) -> Self;
    fn border_indigo_300(self) -> Self;
    fn border_indigo_400(self) -> Self;
    fn border_indigo_500(self) -> Self;
    fn border_indigo_600(self) -> Self;
    fn border_indigo_700(self) -> Self;
    fn border_indigo_800(self) -> Self;
    fn border_indigo_900(self) -> Self;
    fn border_indigo_950(self) -> Self;
    fn border_violet_50(self) -> Self;
    fn border_violet_100(self) -> Self;
    fn border_violet_200(self) -> Self;
    fn border_violet_300(self) -> Self;
    fn border_violet_400(self) -> Self;
    fn border_violet_500(self) -> Self;
    fn border_violet_600(self) -> Self;
    fn border_violet_700(self) -> Self;
    fn border_violet_800(self) -> Self;
    fn border_violet_900(self) -> Self;
    fn border_violet_950(self) -> Self;
    fn border_purple_50(self) -> Self;
    fn border_purple_100(self) -> Self;
    fn border_purple_200(self) -> Self;
    fn border_purple_300(self) -> Self;
    fn border_purple_400(self) -> Self;
    fn border_purple_500(self) -> Self;
    fn border_purple_600(self) -> Self;
    fn border_purple_700(self) -> Self;
    fn border_purple_800(self) -> Self;
    fn border_purple_900(self) -> Self;
    fn border_purple_950(self) -> Self;
    fn border_fuchsia_50(self) -> Self;
    fn border_fuchsia_100(self) -> Self;
    fn border_fuchsia_200(self) -> Self;
    fn border_fuchsia_300(self) -> Self;
    fn border_fuchsia_400(self) -> Self;
    fn border_fuchsia_500(self) -> Self;
    fn border_fuchsia_600(self) -> Self;
    fn border_fuchsia_700(self) -> Self;
    fn border_fuchsia_800(self) -> Self;
    fn border_fuchsia_900(self) -> Self;
    fn border_fuchsia_950(self) -> Self;
    fn border_pink_50(self) -> Self;
    fn border_pink_100(self) -> Self;
    fn border_pink_200(self) -> Self;
    fn border_pink_300(self) -> Self;
    fn border_pink_400(self) -> Self;
    fn border_pink_500(self) -> Self;
    fn border_pink_600(self) -> Self;
    fn border_pink_700(self) -> Self;
    fn border_pink_800(self) -> Self;
    fn border_pink_900(self) -> Self;
    fn border_pink_950(self) -> Self;
    fn border_rose_50(self) -> Self;
    fn border_rose_100(self) -> Self;
    fn border_rose_200(self) -> Self;
    fn border_rose_300(self) -> Self;
    fn border_rose_400(self) -> Self;
    fn border_rose_500(self) -> Self;
    fn border_rose_600(self) -> Self;
    fn border_rose_700(self) -> Self;
    fn border_rose_800(self) -> Self;
    fn border_rose_900(self) -> Self;
    fn border_rose_950(self) -> Self;
}

impl TailwindColorExt for Style {
    fn bg_neutral_50(mut self) -> Self {
        self = self.background(colors::neutral::C50);
        self
    }
    fn bg_neutral_100(mut self) -> Self {
        self = self.background(colors::neutral::C100);
        self
    }
    fn bg_neutral_200(mut self) -> Self {
        self = self.background(colors::neutral::C200);
        self
    }
    fn bg_neutral_300(mut self) -> Self {
        self = self.background(colors::neutral::C300);
        self
    }
    fn bg_neutral_400(mut self) -> Self {
        self = self.background(colors::neutral::C400);
        self
    }
    fn bg_neutral_500(mut self) -> Self {
        self = self.background(colors::neutral::C500);
        self
    }
    fn bg_neutral_600(mut self) -> Self {
        self = self.background(colors::neutral::C600);
        self
    }
    fn bg_neutral_700(mut self) -> Self {
        self = self.background(colors::neutral::C700);
        self
    }
    fn bg_neutral_800(mut self) -> Self {
        self = self.background(colors::neutral::C800);
        self
    }
    fn bg_neutral_900(mut self) -> Self {
        self = self.background(colors::neutral::C900);
        self
    }
    fn bg_neutral_950(mut self) -> Self {
        self = self.background(colors::neutral::C950);
        self
    }
    fn bg_stone_50(mut self) -> Self {
        self = self.background(colors::stone::C50);
        self
    }
    fn bg_stone_100(mut self) -> Self {
        self = self.background(colors::stone::C100);
        self
    }
    fn bg_stone_200(mut self) -> Self {
        self = self.background(colors::stone::C200);
        self
    }
    fn bg_stone_300(mut self) -> Self {
        self = self.background(colors::stone::C300);
        self
    }
    fn bg_stone_400(mut self) -> Self {
        self = self.background(colors::stone::C400);
        self
    }
    fn bg_stone_500(mut self) -> Self {
        self = self.background(colors::stone::C500);
        self
    }
    fn bg_stone_600(mut self) -> Self {
        self = self.background(colors::stone::C600);
        self
    }
    fn bg_stone_700(mut self) -> Self {
        self = self.background(colors::stone::C700);
        self
    }
    fn bg_stone_800(mut self) -> Self {
        self = self.background(colors::stone::C800);
        self
    }
    fn bg_stone_900(mut self) -> Self {
        self = self.background(colors::stone::C900);
        self
    }
    fn bg_stone_950(mut self) -> Self {
        self = self.background(colors::stone::C950);
        self
    }
    fn bg_amber_50(mut self) -> Self {
        self = self.background(colors::amber::C50);
        self
    }
    fn bg_amber_100(mut self) -> Self {
        self = self.background(colors::amber::C100);
        self
    }
    fn bg_amber_200(mut self) -> Self {
        self = self.background(colors::amber::C200);
        self
    }
    fn bg_amber_300(mut self) -> Self {
        self = self.background(colors::amber::C300);
        self
    }
    fn bg_amber_400(mut self) -> Self {
        self = self.background(colors::amber::C400);
        self
    }
    fn bg_amber_500(mut self) -> Self {
        self = self.background(colors::amber::C500);
        self
    }
    fn bg_amber_600(mut self) -> Self {
        self = self.background(colors::amber::C600);
        self
    }
    fn bg_amber_700(mut self) -> Self {
        self = self.background(colors::amber::C700);
        self
    }
    fn bg_amber_800(mut self) -> Self {
        self = self.background(colors::amber::C800);
        self
    }
    fn bg_amber_900(mut self) -> Self {
        self = self.background(colors::amber::C900);
        self
    }
    fn bg_amber_950(mut self) -> Self {
        self = self.background(colors::amber::C950);
        self
    }
    fn bg_lime_50(mut self) -> Self {
        self = self.background(colors::lime::C50);
        self
    }
    fn bg_lime_100(mut self) -> Self {
        self = self.background(colors::lime::C100);
        self
    }
    fn bg_lime_200(mut self) -> Self {
        self = self.background(colors::lime::C200);
        self
    }
    fn bg_lime_300(mut self) -> Self {
        self = self.background(colors::lime::C300);
        self
    }
    fn bg_lime_400(mut self) -> Self {
        self = self.background(colors::lime::C400);
        self
    }
    fn bg_lime_500(mut self) -> Self {
        self = self.background(colors::lime::C500);
        self
    }
    fn bg_lime_600(mut self) -> Self {
        self = self.background(colors::lime::C600);
        self
    }
    fn bg_lime_700(mut self) -> Self {
        self = self.background(colors::lime::C700);
        self
    }
    fn bg_lime_800(mut self) -> Self {
        self = self.background(colors::lime::C800);
        self
    }
    fn bg_lime_900(mut self) -> Self {
        self = self.background(colors::lime::C900);
        self
    }
    fn bg_lime_950(mut self) -> Self {
        self = self.background(colors::lime::C950);
        self
    }
    fn bg_green_50(mut self) -> Self {
        self = self.background(colors::green::C50);
        self
    }
    fn bg_green_100(mut self) -> Self {
        self = self.background(colors::green::C100);
        self
    }
    fn bg_green_200(mut self) -> Self {
        self = self.background(colors::green::C200);
        self
    }
    fn bg_green_300(mut self) -> Self {
        self = self.background(colors::green::C300);
        self
    }
    fn bg_green_400(mut self) -> Self {
        self = self.background(colors::green::C400);
        self
    }
    fn bg_green_500(mut self) -> Self {
        self = self.background(colors::green::C500);
        self
    }
    fn bg_green_600(mut self) -> Self {
        self = self.background(colors::green::C600);
        self
    }
    fn bg_green_700(mut self) -> Self {
        self = self.background(colors::green::C700);
        self
    }
    fn bg_green_800(mut self) -> Self {
        self = self.background(colors::green::C800);
        self
    }
    fn bg_green_900(mut self) -> Self {
        self = self.background(colors::green::C900);
        self
    }
    fn bg_green_950(mut self) -> Self {
        self = self.background(colors::green::C950);
        self
    }
    fn bg_emerald_50(mut self) -> Self {
        self = self.background(colors::emerald::C50);
        self
    }
    fn bg_emerald_100(mut self) -> Self {
        self = self.background(colors::emerald::C100);
        self
    }
    fn bg_emerald_200(mut self) -> Self {
        self = self.background(colors::emerald::C200);
        self
    }
    fn bg_emerald_300(mut self) -> Self {
        self = self.background(colors::emerald::C300);
        self
    }
    fn bg_emerald_400(mut self) -> Self {
        self = self.background(colors::emerald::C400);
        self
    }
    fn bg_emerald_500(mut self) -> Self {
        self = self.background(colors::emerald::C500);
        self
    }
    fn bg_emerald_600(mut self) -> Self {
        self = self.background(colors::emerald::C600);
        self
    }
    fn bg_emerald_700(mut self) -> Self {
        self = self.background(colors::emerald::C700);
        self
    }
    fn bg_emerald_800(mut self) -> Self {
        self = self.background(colors::emerald::C800);
        self
    }
    fn bg_emerald_900(mut self) -> Self {
        self = self.background(colors::emerald::C900);
        self
    }
    fn bg_emerald_950(mut self) -> Self {
        self = self.background(colors::emerald::C950);
        self
    }
    fn bg_teal_50(mut self) -> Self {
        self = self.background(colors::teal::C50);
        self
    }
    fn bg_teal_100(mut self) -> Self {
        self = self.background(colors::teal::C100);
        self
    }
    fn bg_teal_200(mut self) -> Self {
        self = self.background(colors::teal::C200);
        self
    }
    fn bg_teal_300(mut self) -> Self {
        self = self.background(colors::teal::C300);
        self
    }
    fn bg_teal_400(mut self) -> Self {
        self = self.background(colors::teal::C400);
        self
    }
    fn bg_teal_500(mut self) -> Self {
        self = self.background(colors::teal::C500);
        self
    }
    fn bg_teal_600(mut self) -> Self {
        self = self.background(colors::teal::C600);
        self
    }
    fn bg_teal_700(mut self) -> Self {
        self = self.background(colors::teal::C700);
        self
    }
    fn bg_teal_800(mut self) -> Self {
        self = self.background(colors::teal::C800);
        self
    }
    fn bg_teal_900(mut self) -> Self {
        self = self.background(colors::teal::C900);
        self
    }
    fn bg_teal_950(mut self) -> Self {
        self = self.background(colors::teal::C950);
        self
    }
    fn bg_cyan_50(mut self) -> Self {
        self = self.background(colors::cyan::C50);
        self
    }
    fn bg_cyan_100(mut self) -> Self {
        self = self.background(colors::cyan::C100);
        self
    }
    fn bg_cyan_200(mut self) -> Self {
        self = self.background(colors::cyan::C200);
        self
    }
    fn bg_cyan_300(mut self) -> Self {
        self = self.background(colors::cyan::C300);
        self
    }
    fn bg_cyan_400(mut self) -> Self {
        self = self.background(colors::cyan::C400);
        self
    }
    fn bg_cyan_500(mut self) -> Self {
        self = self.background(colors::cyan::C500);
        self
    }
    fn bg_cyan_600(mut self) -> Self {
        self = self.background(colors::cyan::C600);
        self
    }
    fn bg_cyan_700(mut self) -> Self {
        self = self.background(colors::cyan::C700);
        self
    }
    fn bg_cyan_800(mut self) -> Self {
        self = self.background(colors::cyan::C800);
        self
    }
    fn bg_cyan_900(mut self) -> Self {
        self = self.background(colors::cyan::C900);
        self
    }
    fn bg_cyan_950(mut self) -> Self {
        self = self.background(colors::cyan::C950);
        self
    }
    fn bg_sky_50(mut self) -> Self {
        self = self.background(colors::sky::C50);
        self
    }
    fn bg_sky_100(mut self) -> Self {
        self = self.background(colors::sky::C100);
        self
    }
    fn bg_sky_200(mut self) -> Self {
        self = self.background(colors::sky::C200);
        self
    }
    fn bg_sky_300(mut self) -> Self {
        self = self.background(colors::sky::C300);
        self
    }
    fn bg_sky_400(mut self) -> Self {
        self = self.background(colors::sky::C400);
        self
    }
    fn bg_sky_500(mut self) -> Self {
        self = self.background(colors::sky::C500);
        self
    }
    fn bg_sky_600(mut self) -> Self {
        self = self.background(colors::sky::C600);
        self
    }
    fn bg_sky_700(mut self) -> Self {
        self = self.background(colors::sky::C700);
        self
    }
    fn bg_sky_800(mut self) -> Self {
        self = self.background(colors::sky::C800);
        self
    }
    fn bg_sky_900(mut self) -> Self {
        self = self.background(colors::sky::C900);
        self
    }
    fn bg_sky_950(mut self) -> Self {
        self = self.background(colors::sky::C950);
        self
    }
    fn bg_blue_50(mut self) -> Self {
        self = self.background(colors::blue::C50);
        self
    }
    fn bg_blue_100(mut self) -> Self {
        self = self.background(colors::blue::C100);
        self
    }
    fn bg_blue_200(mut self) -> Self {
        self = self.background(colors::blue::C200);
        self
    }
    fn bg_blue_300(mut self) -> Self {
        self = self.background(colors::blue::C300);
        self
    }
    fn bg_blue_400(mut self) -> Self {
        self = self.background(colors::blue::C400);
        self
    }
    fn bg_blue_500(mut self) -> Self {
        self = self.background(colors::blue::C500);
        self
    }
    fn bg_blue_600(mut self) -> Self {
        self = self.background(colors::blue::C600);
        self
    }
    fn bg_blue_700(mut self) -> Self {
        self = self.background(colors::blue::C700);
        self
    }
    fn bg_blue_800(mut self) -> Self {
        self = self.background(colors::blue::C800);
        self
    }
    fn bg_blue_900(mut self) -> Self {
        self = self.background(colors::blue::C900);
        self
    }
    fn bg_blue_950(mut self) -> Self {
        self = self.background(colors::blue::C950);
        self
    }
    fn bg_indigo_50(mut self) -> Self {
        self = self.background(colors::indigo::C50);
        self
    }
    fn bg_indigo_100(mut self) -> Self {
        self = self.background(colors::indigo::C100);
        self
    }
    fn bg_indigo_200(mut self) -> Self {
        self = self.background(colors::indigo::C200);
        self
    }
    fn bg_indigo_300(mut self) -> Self {
        self = self.background(colors::indigo::C300);
        self
    }
    fn bg_indigo_400(mut self) -> Self {
        self = self.background(colors::indigo::C400);
        self
    }
    fn bg_indigo_500(mut self) -> Self {
        self = self.background(colors::indigo::C500);
        self
    }
    fn bg_indigo_600(mut self) -> Self {
        self = self.background(colors::indigo::C600);
        self
    }
    fn bg_indigo_700(mut self) -> Self {
        self = self.background(colors::indigo::C700);
        self
    }
    fn bg_indigo_800(mut self) -> Self {
        self = self.background(colors::indigo::C800);
        self
    }
    fn bg_indigo_900(mut self) -> Self {
        self = self.background(colors::indigo::C900);
        self
    }
    fn bg_indigo_950(mut self) -> Self {
        self = self.background(colors::indigo::C950);
        self
    }
    fn bg_violet_50(mut self) -> Self {
        self = self.background(colors::violet::C50);
        self
    }
    fn bg_violet_100(mut self) -> Self {
        self = self.background(colors::violet::C100);
        self
    }
    fn bg_violet_200(mut self) -> Self {
        self = self.background(colors::violet::C200);
        self
    }
    fn bg_violet_300(mut self) -> Self {
        self = self.background(colors::violet::C300);
        self
    }
    fn bg_violet_400(mut self) -> Self {
        self = self.background(colors::violet::C400);
        self
    }
    fn bg_violet_500(mut self) -> Self {
        self = self.background(colors::violet::C500);
        self
    }
    fn bg_violet_600(mut self) -> Self {
        self = self.background(colors::violet::C600);
        self
    }
    fn bg_violet_700(mut self) -> Self {
        self = self.background(colors::violet::C700);
        self
    }
    fn bg_violet_800(mut self) -> Self {
        self = self.background(colors::violet::C800);
        self
    }
    fn bg_violet_900(mut self) -> Self {
        self = self.background(colors::violet::C900);
        self
    }
    fn bg_violet_950(mut self) -> Self {
        self = self.background(colors::violet::C950);
        self
    }
    fn bg_purple_50(mut self) -> Self {
        self = self.background(colors::purple::C50);
        self
    }
    fn bg_purple_100(mut self) -> Self {
        self = self.background(colors::purple::C100);
        self
    }
    fn bg_purple_200(mut self) -> Self {
        self = self.background(colors::purple::C200);
        self
    }
    fn bg_purple_300(mut self) -> Self {
        self = self.background(colors::purple::C300);
        self
    }
    fn bg_purple_400(mut self) -> Self {
        self = self.background(colors::purple::C400);
        self
    }
    fn bg_purple_500(mut self) -> Self {
        self = self.background(colors::purple::C500);
        self
    }
    fn bg_purple_600(mut self) -> Self {
        self = self.background(colors::purple::C600);
        self
    }
    fn bg_purple_700(mut self) -> Self {
        self = self.background(colors::purple::C700);
        self
    }
    fn bg_purple_800(mut self) -> Self {
        self = self.background(colors::purple::C800);
        self
    }
    fn bg_purple_900(mut self) -> Self {
        self = self.background(colors::purple::C900);
        self
    }
    fn bg_purple_950(mut self) -> Self {
        self = self.background(colors::purple::C950);
        self
    }
    fn bg_fuchsia_50(mut self) -> Self {
        self = self.background(colors::fuchsia::C50);
        self
    }
    fn bg_fuchsia_100(mut self) -> Self {
        self = self.background(colors::fuchsia::C100);
        self
    }
    fn bg_fuchsia_200(mut self) -> Self {
        self = self.background(colors::fuchsia::C200);
        self
    }
    fn bg_fuchsia_300(mut self) -> Self {
        self = self.background(colors::fuchsia::C300);
        self
    }
    fn bg_fuchsia_400(mut self) -> Self {
        self = self.background(colors::fuchsia::C400);
        self
    }
    fn bg_fuchsia_500(mut self) -> Self {
        self = self.background(colors::fuchsia::C500);
        self
    }
    fn bg_fuchsia_600(mut self) -> Self {
        self = self.background(colors::fuchsia::C600);
        self
    }
    fn bg_fuchsia_700(mut self) -> Self {
        self = self.background(colors::fuchsia::C700);
        self
    }
    fn bg_fuchsia_800(mut self) -> Self {
        self = self.background(colors::fuchsia::C800);
        self
    }
    fn bg_fuchsia_900(mut self) -> Self {
        self = self.background(colors::fuchsia::C900);
        self
    }
    fn bg_fuchsia_950(mut self) -> Self {
        self = self.background(colors::fuchsia::C950);
        self
    }
    fn bg_pink_50(mut self) -> Self {
        self = self.background(colors::pink::C50);
        self
    }
    fn bg_pink_100(mut self) -> Self {
        self = self.background(colors::pink::C100);
        self
    }
    fn bg_pink_200(mut self) -> Self {
        self = self.background(colors::pink::C200);
        self
    }
    fn bg_pink_300(mut self) -> Self {
        self = self.background(colors::pink::C300);
        self
    }
    fn bg_pink_400(mut self) -> Self {
        self = self.background(colors::pink::C400);
        self
    }
    fn bg_pink_500(mut self) -> Self {
        self = self.background(colors::pink::C500);
        self
    }
    fn bg_pink_600(mut self) -> Self {
        self = self.background(colors::pink::C600);
        self
    }
    fn bg_pink_700(mut self) -> Self {
        self = self.background(colors::pink::C700);
        self
    }
    fn bg_pink_800(mut self) -> Self {
        self = self.background(colors::pink::C800);
        self
    }
    fn bg_pink_900(mut self) -> Self {
        self = self.background(colors::pink::C900);
        self
    }
    fn bg_pink_950(mut self) -> Self {
        self = self.background(colors::pink::C950);
        self
    }
    fn bg_rose_50(mut self) -> Self {
        self = self.background(colors::rose::C50);
        self
    }
    fn bg_rose_100(mut self) -> Self {
        self = self.background(colors::rose::C100);
        self
    }
    fn bg_rose_200(mut self) -> Self {
        self = self.background(colors::rose::C200);
        self
    }
    fn bg_rose_300(mut self) -> Self {
        self = self.background(colors::rose::C300);
        self
    }
    fn bg_rose_400(mut self) -> Self {
        self = self.background(colors::rose::C400);
        self
    }
    fn bg_rose_500(mut self) -> Self {
        self = self.background(colors::rose::C500);
        self
    }
    fn bg_rose_600(mut self) -> Self {
        self = self.background(colors::rose::C600);
        self
    }
    fn bg_rose_700(mut self) -> Self {
        self = self.background(colors::rose::C700);
        self
    }
    fn bg_rose_800(mut self) -> Self {
        self = self.background(colors::rose::C800);
        self
    }
    fn bg_rose_900(mut self) -> Self {
        self = self.background(colors::rose::C900);
        self
    }
    fn bg_rose_950(mut self) -> Self {
        self = self.background(colors::rose::C950);
        self
    }
    fn text_neutral_50(mut self) -> Self {
        self = self.color(colors::neutral::C50);
        self
    }
    fn text_neutral_100(mut self) -> Self {
        self = self.color(colors::neutral::C100);
        self
    }
    fn text_neutral_200(mut self) -> Self {
        self = self.color(colors::neutral::C200);
        self
    }
    fn text_neutral_300(mut self) -> Self {
        self = self.color(colors::neutral::C300);
        self
    }
    fn text_neutral_400(mut self) -> Self {
        self = self.color(colors::neutral::C400);
        self
    }
    fn text_neutral_500(mut self) -> Self {
        self = self.color(colors::neutral::C500);
        self
    }
    fn text_neutral_600(mut self) -> Self {
        self = self.color(colors::neutral::C600);
        self
    }
    fn text_neutral_700(mut self) -> Self {
        self = self.color(colors::neutral::C700);
        self
    }
    fn text_neutral_800(mut self) -> Self {
        self = self.color(colors::neutral::C800);
        self
    }
    fn text_neutral_900(mut self) -> Self {
        self = self.color(colors::neutral::C900);
        self
    }
    fn text_neutral_950(mut self) -> Self {
        self = self.color(colors::neutral::C950);
        self
    }
    fn text_stone_50(mut self) -> Self {
        self = self.color(colors::stone::C50);
        self
    }
    fn text_stone_100(mut self) -> Self {
        self = self.color(colors::stone::C100);
        self
    }
    fn text_stone_200(mut self) -> Self {
        self = self.color(colors::stone::C200);
        self
    }
    fn text_stone_300(mut self) -> Self {
        self = self.color(colors::stone::C300);
        self
    }
    fn text_stone_400(mut self) -> Self {
        self = self.color(colors::stone::C400);
        self
    }
    fn text_stone_500(mut self) -> Self {
        self = self.color(colors::stone::C500);
        self
    }
    fn text_stone_600(mut self) -> Self {
        self = self.color(colors::stone::C600);
        self
    }
    fn text_stone_700(mut self) -> Self {
        self = self.color(colors::stone::C700);
        self
    }
    fn text_stone_800(mut self) -> Self {
        self = self.color(colors::stone::C800);
        self
    }
    fn text_stone_900(mut self) -> Self {
        self = self.color(colors::stone::C900);
        self
    }
    fn text_stone_950(mut self) -> Self {
        self = self.color(colors::stone::C950);
        self
    }
    fn text_amber_50(mut self) -> Self {
        self = self.color(colors::amber::C50);
        self
    }
    fn text_amber_100(mut self) -> Self {
        self = self.color(colors::amber::C100);
        self
    }
    fn text_amber_200(mut self) -> Self {
        self = self.color(colors::amber::C200);
        self
    }
    fn text_amber_300(mut self) -> Self {
        self = self.color(colors::amber::C300);
        self
    }
    fn text_amber_400(mut self) -> Self {
        self = self.color(colors::amber::C400);
        self
    }
    fn text_amber_500(mut self) -> Self {
        self = self.color(colors::amber::C500);
        self
    }
    fn text_amber_600(mut self) -> Self {
        self = self.color(colors::amber::C600);
        self
    }
    fn text_amber_700(mut self) -> Self {
        self = self.color(colors::amber::C700);
        self
    }
    fn text_amber_800(mut self) -> Self {
        self = self.color(colors::amber::C800);
        self
    }
    fn text_amber_900(mut self) -> Self {
        self = self.color(colors::amber::C900);
        self
    }
    fn text_amber_950(mut self) -> Self {
        self = self.color(colors::amber::C950);
        self
    }
    fn text_lime_50(mut self) -> Self {
        self = self.color(colors::lime::C50);
        self
    }
    fn text_lime_100(mut self) -> Self {
        self = self.color(colors::lime::C100);
        self
    }
    fn text_lime_200(mut self) -> Self {
        self = self.color(colors::lime::C200);
        self
    }
    fn text_lime_300(mut self) -> Self {
        self = self.color(colors::lime::C300);
        self
    }
    fn text_lime_400(mut self) -> Self {
        self = self.color(colors::lime::C400);
        self
    }
    fn text_lime_500(mut self) -> Self {
        self = self.color(colors::lime::C500);
        self
    }
    fn text_lime_600(mut self) -> Self {
        self = self.color(colors::lime::C600);
        self
    }
    fn text_lime_700(mut self) -> Self {
        self = self.color(colors::lime::C700);
        self
    }
    fn text_lime_800(mut self) -> Self {
        self = self.color(colors::lime::C800);
        self
    }
    fn text_lime_900(mut self) -> Self {
        self = self.color(colors::lime::C900);
        self
    }
    fn text_lime_950(mut self) -> Self {
        self = self.color(colors::lime::C950);
        self
    }
    fn text_green_50(mut self) -> Self {
        self = self.color(colors::green::C50);
        self
    }
    fn text_green_100(mut self) -> Self {
        self = self.color(colors::green::C100);
        self
    }
    fn text_green_200(mut self) -> Self {
        self = self.color(colors::green::C200);
        self
    }
    fn text_green_300(mut self) -> Self {
        self = self.color(colors::green::C300);
        self
    }
    fn text_green_400(mut self) -> Self {
        self = self.color(colors::green::C400);
        self
    }
    fn text_green_500(mut self) -> Self {
        self = self.color(colors::green::C500);
        self
    }
    fn text_green_600(mut self) -> Self {
        self = self.color(colors::green::C600);
        self
    }
    fn text_green_700(mut self) -> Self {
        self = self.color(colors::green::C700);
        self
    }
    fn text_green_800(mut self) -> Self {
        self = self.color(colors::green::C800);
        self
    }
    fn text_green_900(mut self) -> Self {
        self = self.color(colors::green::C900);
        self
    }
    fn text_green_950(mut self) -> Self {
        self = self.color(colors::green::C950);
        self
    }
    fn text_emerald_50(mut self) -> Self {
        self = self.color(colors::emerald::C50);
        self
    }
    fn text_emerald_100(mut self) -> Self {
        self = self.color(colors::emerald::C100);
        self
    }
    fn text_emerald_200(mut self) -> Self {
        self = self.color(colors::emerald::C200);
        self
    }
    fn text_emerald_300(mut self) -> Self {
        self = self.color(colors::emerald::C300);
        self
    }
    fn text_emerald_400(mut self) -> Self {
        self = self.color(colors::emerald::C400);
        self
    }
    fn text_emerald_500(mut self) -> Self {
        self = self.color(colors::emerald::C500);
        self
    }
    fn text_emerald_600(mut self) -> Self {
        self = self.color(colors::emerald::C600);
        self
    }
    fn text_emerald_700(mut self) -> Self {
        self = self.color(colors::emerald::C700);
        self
    }
    fn text_emerald_800(mut self) -> Self {
        self = self.color(colors::emerald::C800);
        self
    }
    fn text_emerald_900(mut self) -> Self {
        self = self.color(colors::emerald::C900);
        self
    }
    fn text_emerald_950(mut self) -> Self {
        self = self.color(colors::emerald::C950);
        self
    }
    fn text_teal_50(mut self) -> Self {
        self = self.color(colors::teal::C50);
        self
    }
    fn text_teal_100(mut self) -> Self {
        self = self.color(colors::teal::C100);
        self
    }
    fn text_teal_200(mut self) -> Self {
        self = self.color(colors::teal::C200);
        self
    }
    fn text_teal_300(mut self) -> Self {
        self = self.color(colors::teal::C300);
        self
    }
    fn text_teal_400(mut self) -> Self {
        self = self.color(colors::teal::C400);
        self
    }
    fn text_teal_500(mut self) -> Self {
        self = self.color(colors::teal::C500);
        self
    }
    fn text_teal_600(mut self) -> Self {
        self = self.color(colors::teal::C600);
        self
    }
    fn text_teal_700(mut self) -> Self {
        self = self.color(colors::teal::C700);
        self
    }
    fn text_teal_800(mut self) -> Self {
        self = self.color(colors::teal::C800);
        self
    }
    fn text_teal_900(mut self) -> Self {
        self = self.color(colors::teal::C900);
        self
    }
    fn text_teal_950(mut self) -> Self {
        self = self.color(colors::teal::C950);
        self
    }
    fn text_cyan_50(mut self) -> Self {
        self = self.color(colors::cyan::C50);
        self
    }
    fn text_cyan_100(mut self) -> Self {
        self = self.color(colors::cyan::C100);
        self
    }
    fn text_cyan_200(mut self) -> Self {
        self = self.color(colors::cyan::C200);
        self
    }
    fn text_cyan_300(mut self) -> Self {
        self = self.color(colors::cyan::C300);
        self
    }
    fn text_cyan_400(mut self) -> Self {
        self = self.color(colors::cyan::C400);
        self
    }
    fn text_cyan_500(mut self) -> Self {
        self = self.color(colors::cyan::C500);
        self
    }
    fn text_cyan_600(mut self) -> Self {
        self = self.color(colors::cyan::C600);
        self
    }
    fn text_cyan_700(mut self) -> Self {
        self = self.color(colors::cyan::C700);
        self
    }
    fn text_cyan_800(mut self) -> Self {
        self = self.color(colors::cyan::C800);
        self
    }
    fn text_cyan_900(mut self) -> Self {
        self = self.color(colors::cyan::C900);
        self
    }
    fn text_cyan_950(mut self) -> Self {
        self = self.color(colors::cyan::C950);
        self
    }
    fn text_sky_50(mut self) -> Self {
        self = self.color(colors::sky::C50);
        self
    }
    fn text_sky_100(mut self) -> Self {
        self = self.color(colors::sky::C100);
        self
    }
    fn text_sky_200(mut self) -> Self {
        self = self.color(colors::sky::C200);
        self
    }
    fn text_sky_300(mut self) -> Self {
        self = self.color(colors::sky::C300);
        self
    }
    fn text_sky_400(mut self) -> Self {
        self = self.color(colors::sky::C400);
        self
    }
    fn text_sky_500(mut self) -> Self {
        self = self.color(colors::sky::C500);
        self
    }
    fn text_sky_600(mut self) -> Self {
        self = self.color(colors::sky::C600);
        self
    }
    fn text_sky_700(mut self) -> Self {
        self = self.color(colors::sky::C700);
        self
    }
    fn text_sky_800(mut self) -> Self {
        self = self.color(colors::sky::C800);
        self
    }
    fn text_sky_900(mut self) -> Self {
        self = self.color(colors::sky::C900);
        self
    }
    fn text_sky_950(mut self) -> Self {
        self = self.color(colors::sky::C950);
        self
    }
    fn text_blue_50(mut self) -> Self {
        self = self.color(colors::blue::C50);
        self
    }
    fn text_blue_100(mut self) -> Self {
        self = self.color(colors::blue::C100);
        self
    }
    fn text_blue_200(mut self) -> Self {
        self = self.color(colors::blue::C200);
        self
    }
    fn text_blue_300(mut self) -> Self {
        self = self.color(colors::blue::C300);
        self
    }
    fn text_blue_400(mut self) -> Self {
        self = self.color(colors::blue::C400);
        self
    }
    fn text_blue_500(mut self) -> Self {
        self = self.color(colors::blue::C500);
        self
    }
    fn text_blue_600(mut self) -> Self {
        self = self.color(colors::blue::C600);
        self
    }
    fn text_blue_700(mut self) -> Self {
        self = self.color(colors::blue::C700);
        self
    }
    fn text_blue_800(mut self) -> Self {
        self = self.color(colors::blue::C800);
        self
    }
    fn text_blue_900(mut self) -> Self {
        self = self.color(colors::blue::C900);
        self
    }
    fn text_blue_950(mut self) -> Self {
        self = self.color(colors::blue::C950);
        self
    }
    fn text_indigo_50(mut self) -> Self {
        self = self.color(colors::indigo::C50);
        self
    }
    fn text_indigo_100(mut self) -> Self {
        self = self.color(colors::indigo::C100);
        self
    }
    fn text_indigo_200(mut self) -> Self {
        self = self.color(colors::indigo::C200);
        self
    }
    fn text_indigo_300(mut self) -> Self {
        self = self.color(colors::indigo::C300);
        self
    }
    fn text_indigo_400(mut self) -> Self {
        self = self.color(colors::indigo::C400);
        self
    }
    fn text_indigo_500(mut self) -> Self {
        self = self.color(colors::indigo::C500);
        self
    }
    fn text_indigo_600(mut self) -> Self {
        self = self.color(colors::indigo::C600);
        self
    }
    fn text_indigo_700(mut self) -> Self {
        self = self.color(colors::indigo::C700);
        self
    }
    fn text_indigo_800(mut self) -> Self {
        self = self.color(colors::indigo::C800);
        self
    }
    fn text_indigo_900(mut self) -> Self {
        self = self.color(colors::indigo::C900);
        self
    }
    fn text_indigo_950(mut self) -> Self {
        self = self.color(colors::indigo::C950);
        self
    }
    fn text_violet_50(mut self) -> Self {
        self = self.color(colors::violet::C50);
        self
    }
    fn text_violet_100(mut self) -> Self {
        self = self.color(colors::violet::C100);
        self
    }
    fn text_violet_200(mut self) -> Self {
        self = self.color(colors::violet::C200);
        self
    }
    fn text_violet_300(mut self) -> Self {
        self = self.color(colors::violet::C300);
        self
    }
    fn text_violet_400(mut self) -> Self {
        self = self.color(colors::violet::C400);
        self
    }
    fn text_violet_500(mut self) -> Self {
        self = self.color(colors::violet::C500);
        self
    }
    fn text_violet_600(mut self) -> Self {
        self = self.color(colors::violet::C600);
        self
    }
    fn text_violet_700(mut self) -> Self {
        self = self.color(colors::violet::C700);
        self
    }
    fn text_violet_800(mut self) -> Self {
        self = self.color(colors::violet::C800);
        self
    }
    fn text_violet_900(mut self) -> Self {
        self = self.color(colors::violet::C900);
        self
    }
    fn text_violet_950(mut self) -> Self {
        self = self.color(colors::violet::C950);
        self
    }
    fn text_purple_50(mut self) -> Self {
        self = self.color(colors::purple::C50);
        self
    }
    fn text_purple_100(mut self) -> Self {
        self = self.color(colors::purple::C100);
        self
    }
    fn text_purple_200(mut self) -> Self {
        self = self.color(colors::purple::C200);
        self
    }
    fn text_purple_300(mut self) -> Self {
        self = self.color(colors::purple::C300);
        self
    }
    fn text_purple_400(mut self) -> Self {
        self = self.color(colors::purple::C400);
        self
    }
    fn text_purple_500(mut self) -> Self {
        self = self.color(colors::purple::C500);
        self
    }
    fn text_purple_600(mut self) -> Self {
        self = self.color(colors::purple::C600);
        self
    }
    fn text_purple_700(mut self) -> Self {
        self = self.color(colors::purple::C700);
        self
    }
    fn text_purple_800(mut self) -> Self {
        self = self.color(colors::purple::C800);
        self
    }
    fn text_purple_900(mut self) -> Self {
        self = self.color(colors::purple::C900);
        self
    }
    fn text_purple_950(mut self) -> Self {
        self = self.color(colors::purple::C950);
        self
    }
    fn text_fuchsia_50(mut self) -> Self {
        self = self.color(colors::fuchsia::C50);
        self
    }
    fn text_fuchsia_100(mut self) -> Self {
        self = self.color(colors::fuchsia::C100);
        self
    }
    fn text_fuchsia_200(mut self) -> Self {
        self = self.color(colors::fuchsia::C200);
        self
    }
    fn text_fuchsia_300(mut self) -> Self {
        self = self.color(colors::fuchsia::C300);
        self
    }
    fn text_fuchsia_400(mut self) -> Self {
        self = self.color(colors::fuchsia::C400);
        self
    }
    fn text_fuchsia_500(mut self) -> Self {
        self = self.color(colors::fuchsia::C500);
        self
    }
    fn text_fuchsia_600(mut self) -> Self {
        self = self.color(colors::fuchsia::C600);
        self
    }
    fn text_fuchsia_700(mut self) -> Self {
        self = self.color(colors::fuchsia::C700);
        self
    }
    fn text_fuchsia_800(mut self) -> Self {
        self = self.color(colors::fuchsia::C800);
        self
    }
    fn text_fuchsia_900(mut self) -> Self {
        self = self.color(colors::fuchsia::C900);
        self
    }
    fn text_fuchsia_950(mut self) -> Self {
        self = self.color(colors::fuchsia::C950);
        self
    }
    fn text_pink_50(mut self) -> Self {
        self = self.color(colors::pink::C50);
        self
    }
    fn text_pink_100(mut self) -> Self {
        self = self.color(colors::pink::C100);
        self
    }
    fn text_pink_200(mut self) -> Self {
        self = self.color(colors::pink::C200);
        self
    }
    fn text_pink_300(mut self) -> Self {
        self = self.color(colors::pink::C300);
        self
    }
    fn text_pink_400(mut self) -> Self {
        self = self.color(colors::pink::C400);
        self
    }
    fn text_pink_500(mut self) -> Self {
        self = self.color(colors::pink::C500);
        self
    }
    fn text_pink_600(mut self) -> Self {
        self = self.color(colors::pink::C600);
        self
    }
    fn text_pink_700(mut self) -> Self {
        self = self.color(colors::pink::C700);
        self
    }
    fn text_pink_800(mut self) -> Self {
        self = self.color(colors::pink::C800);
        self
    }
    fn text_pink_900(mut self) -> Self {
        self = self.color(colors::pink::C900);
        self
    }
    fn text_pink_950(mut self) -> Self {
        self = self.color(colors::pink::C950);
        self
    }
    fn text_rose_50(mut self) -> Self {
        self = self.color(colors::rose::C50);
        self
    }
    fn text_rose_100(mut self) -> Self {
        self = self.color(colors::rose::C100);
        self
    }
    fn text_rose_200(mut self) -> Self {
        self = self.color(colors::rose::C200);
        self
    }
    fn text_rose_300(mut self) -> Self {
        self = self.color(colors::rose::C300);
        self
    }
    fn text_rose_400(mut self) -> Self {
        self = self.color(colors::rose::C400);
        self
    }
    fn text_rose_500(mut self) -> Self {
        self = self.color(colors::rose::C500);
        self
    }
    fn text_rose_600(mut self) -> Self {
        self = self.color(colors::rose::C600);
        self
    }
    fn text_rose_700(mut self) -> Self {
        self = self.color(colors::rose::C700);
        self
    }
    fn text_rose_800(mut self) -> Self {
        self = self.color(colors::rose::C800);
        self
    }
    fn text_rose_900(mut self) -> Self {
        self = self.color(colors::rose::C900);
        self
    }
    fn text_rose_950(mut self) -> Self {
        self = self.color(colors::rose::C950);
        self
    }
    fn border_neutral_50(mut self) -> Self {
        self = self.border_color(colors::neutral::C50);
        self
    }
    fn border_neutral_100(mut self) -> Self {
        self = self.border_color(colors::neutral::C100);
        self
    }
    fn border_neutral_200(mut self) -> Self {
        self = self.border_color(colors::neutral::C200);
        self
    }
    fn border_neutral_300(mut self) -> Self {
        self = self.border_color(colors::neutral::C300);
        self
    }
    fn border_neutral_400(mut self) -> Self {
        self = self.border_color(colors::neutral::C400);
        self
    }
    fn border_neutral_500(mut self) -> Self {
        self = self.border_color(colors::neutral::C500);
        self
    }
    fn border_neutral_600(mut self) -> Self {
        self = self.border_color(colors::neutral::C600);
        self
    }
    fn border_neutral_700(mut self) -> Self {
        self = self.border_color(colors::neutral::C700);
        self
    }
    fn border_neutral_800(mut self) -> Self {
        self = self.border_color(colors::neutral::C800);
        self
    }
    fn border_neutral_900(mut self) -> Self {
        self = self.border_color(colors::neutral::C900);
        self
    }
    fn border_neutral_950(mut self) -> Self {
        self = self.border_color(colors::neutral::C950);
        self
    }
    fn border_stone_50(mut self) -> Self {
        self = self.border_color(colors::stone::C50);
        self
    }
    fn border_stone_100(mut self) -> Self {
        self = self.border_color(colors::stone::C100);
        self
    }
    fn border_stone_200(mut self) -> Self {
        self = self.border_color(colors::stone::C200);
        self
    }
    fn border_stone_300(mut self) -> Self {
        self = self.border_color(colors::stone::C300);
        self
    }
    fn border_stone_400(mut self) -> Self {
        self = self.border_color(colors::stone::C400);
        self
    }
    fn border_stone_500(mut self) -> Self {
        self = self.border_color(colors::stone::C500);
        self
    }
    fn border_stone_600(mut self) -> Self {
        self = self.border_color(colors::stone::C600);
        self
    }
    fn border_stone_700(mut self) -> Self {
        self = self.border_color(colors::stone::C700);
        self
    }
    fn border_stone_800(mut self) -> Self {
        self = self.border_color(colors::stone::C800);
        self
    }
    fn border_stone_900(mut self) -> Self {
        self = self.border_color(colors::stone::C900);
        self
    }
    fn border_stone_950(mut self) -> Self {
        self = self.border_color(colors::stone::C950);
        self
    }
    fn border_amber_50(mut self) -> Self {
        self = self.border_color(colors::amber::C50);
        self
    }
    fn border_amber_100(mut self) -> Self {
        self = self.border_color(colors::amber::C100);
        self
    }
    fn border_amber_200(mut self) -> Self {
        self = self.border_color(colors::amber::C200);
        self
    }
    fn border_amber_300(mut self) -> Self {
        self = self.border_color(colors::amber::C300);
        self
    }
    fn border_amber_400(mut self) -> Self {
        self = self.border_color(colors::amber::C400);
        self
    }
    fn border_amber_500(mut self) -> Self {
        self = self.border_color(colors::amber::C500);
        self
    }
    fn border_amber_600(mut self) -> Self {
        self = self.border_color(colors::amber::C600);
        self
    }
    fn border_amber_700(mut self) -> Self {
        self = self.border_color(colors::amber::C700);
        self
    }
    fn border_amber_800(mut self) -> Self {
        self = self.border_color(colors::amber::C800);
        self
    }
    fn border_amber_900(mut self) -> Self {
        self = self.border_color(colors::amber::C900);
        self
    }
    fn border_amber_950(mut self) -> Self {
        self = self.border_color(colors::amber::C950);
        self
    }
    fn border_lime_50(mut self) -> Self {
        self = self.border_color(colors::lime::C50);
        self
    }
    fn border_lime_100(mut self) -> Self {
        self = self.border_color(colors::lime::C100);
        self
    }
    fn border_lime_200(mut self) -> Self {
        self = self.border_color(colors::lime::C200);
        self
    }
    fn border_lime_300(mut self) -> Self {
        self = self.border_color(colors::lime::C300);
        self
    }
    fn border_lime_400(mut self) -> Self {
        self = self.border_color(colors::lime::C400);
        self
    }
    fn border_lime_500(mut self) -> Self {
        self = self.border_color(colors::lime::C500);
        self
    }
    fn border_lime_600(mut self) -> Self {
        self = self.border_color(colors::lime::C600);
        self
    }
    fn border_lime_700(mut self) -> Self {
        self = self.border_color(colors::lime::C700);
        self
    }
    fn border_lime_800(mut self) -> Self {
        self = self.border_color(colors::lime::C800);
        self
    }
    fn border_lime_900(mut self) -> Self {
        self = self.border_color(colors::lime::C900);
        self
    }
    fn border_lime_950(mut self) -> Self {
        self = self.border_color(colors::lime::C950);
        self
    }
    fn border_green_50(mut self) -> Self {
        self = self.border_color(colors::green::C50);
        self
    }
    fn border_green_100(mut self) -> Self {
        self = self.border_color(colors::green::C100);
        self
    }
    fn border_green_200(mut self) -> Self {
        self = self.border_color(colors::green::C200);
        self
    }
    fn border_green_300(mut self) -> Self {
        self = self.border_color(colors::green::C300);
        self
    }
    fn border_green_400(mut self) -> Self {
        self = self.border_color(colors::green::C400);
        self
    }
    fn border_green_500(mut self) -> Self {
        self = self.border_color(colors::green::C500);
        self
    }
    fn border_green_600(mut self) -> Self {
        self = self.border_color(colors::green::C600);
        self
    }
    fn border_green_700(mut self) -> Self {
        self = self.border_color(colors::green::C700);
        self
    }
    fn border_green_800(mut self) -> Self {
        self = self.border_color(colors::green::C800);
        self
    }
    fn border_green_900(mut self) -> Self {
        self = self.border_color(colors::green::C900);
        self
    }
    fn border_green_950(mut self) -> Self {
        self = self.border_color(colors::green::C950);
        self
    }
    fn border_emerald_50(mut self) -> Self {
        self = self.border_color(colors::emerald::C50);
        self
    }
    fn border_emerald_100(mut self) -> Self {
        self = self.border_color(colors::emerald::C100);
        self
    }
    fn border_emerald_200(mut self) -> Self {
        self = self.border_color(colors::emerald::C200);
        self
    }
    fn border_emerald_300(mut self) -> Self {
        self = self.border_color(colors::emerald::C300);
        self
    }
    fn border_emerald_400(mut self) -> Self {
        self = self.border_color(colors::emerald::C400);
        self
    }
    fn border_emerald_500(mut self) -> Self {
        self = self.border_color(colors::emerald::C500);
        self
    }
    fn border_emerald_600(mut self) -> Self {
        self = self.border_color(colors::emerald::C600);
        self
    }
    fn border_emerald_700(mut self) -> Self {
        self = self.border_color(colors::emerald::C700);
        self
    }
    fn border_emerald_800(mut self) -> Self {
        self = self.border_color(colors::emerald::C800);
        self
    }
    fn border_emerald_900(mut self) -> Self {
        self = self.border_color(colors::emerald::C900);
        self
    }
    fn border_emerald_950(mut self) -> Self {
        self = self.border_color(colors::emerald::C950);
        self
    }
    fn border_teal_50(mut self) -> Self {
        self = self.border_color(colors::teal::C50);
        self
    }
    fn border_teal_100(mut self) -> Self {
        self = self.border_color(colors::teal::C100);
        self
    }
    fn border_teal_200(mut self) -> Self {
        self = self.border_color(colors::teal::C200);
        self
    }
    fn border_teal_300(mut self) -> Self {
        self = self.border_color(colors::teal::C300);
        self
    }
    fn border_teal_400(mut self) -> Self {
        self = self.border_color(colors::teal::C400);
        self
    }
    fn border_teal_500(mut self) -> Self {
        self = self.border_color(colors::teal::C500);
        self
    }
    fn border_teal_600(mut self) -> Self {
        self = self.border_color(colors::teal::C600);
        self
    }
    fn border_teal_700(mut self) -> Self {
        self = self.border_color(colors::teal::C700);
        self
    }
    fn border_teal_800(mut self) -> Self {
        self = self.border_color(colors::teal::C800);
        self
    }
    fn border_teal_900(mut self) -> Self {
        self = self.border_color(colors::teal::C900);
        self
    }
    fn border_teal_950(mut self) -> Self {
        self = self.border_color(colors::teal::C950);
        self
    }
    fn border_cyan_50(mut self) -> Self {
        self = self.border_color(colors::cyan::C50);
        self
    }
    fn border_cyan_100(mut self) -> Self {
        self = self.border_color(colors::cyan::C100);
        self
    }
    fn border_cyan_200(mut self) -> Self {
        self = self.border_color(colors::cyan::C200);
        self
    }
    fn border_cyan_300(mut self) -> Self {
        self = self.border_color(colors::cyan::C300);
        self
    }
    fn border_cyan_400(mut self) -> Self {
        self = self.border_color(colors::cyan::C400);
        self
    }
    fn border_cyan_500(mut self) -> Self {
        self = self.border_color(colors::cyan::C500);
        self
    }
    fn border_cyan_600(mut self) -> Self {
        self = self.border_color(colors::cyan::C600);
        self
    }
    fn border_cyan_700(mut self) -> Self {
        self = self.border_color(colors::cyan::C700);
        self
    }
    fn border_cyan_800(mut self) -> Self {
        self = self.border_color(colors::cyan::C800);
        self
    }
    fn border_cyan_900(mut self) -> Self {
        self = self.border_color(colors::cyan::C900);
        self
    }
    fn border_cyan_950(mut self) -> Self {
        self = self.border_color(colors::cyan::C950);
        self
    }
    fn border_sky_50(mut self) -> Self {
        self = self.border_color(colors::sky::C50);
        self
    }
    fn border_sky_100(mut self) -> Self {
        self = self.border_color(colors::sky::C100);
        self
    }
    fn border_sky_200(mut self) -> Self {
        self = self.border_color(colors::sky::C200);
        self
    }
    fn border_sky_300(mut self) -> Self {
        self = self.border_color(colors::sky::C300);
        self
    }
    fn border_sky_400(mut self) -> Self {
        self = self.border_color(colors::sky::C400);
        self
    }
    fn border_sky_500(mut self) -> Self {
        self = self.border_color(colors::sky::C500);
        self
    }
    fn border_sky_600(mut self) -> Self {
        self = self.border_color(colors::sky::C600);
        self
    }
    fn border_sky_700(mut self) -> Self {
        self = self.border_color(colors::sky::C700);
        self
    }
    fn border_sky_800(mut self) -> Self {
        self = self.border_color(colors::sky::C800);
        self
    }
    fn border_sky_900(mut self) -> Self {
        self = self.border_color(colors::sky::C900);
        self
    }
    fn border_sky_950(mut self) -> Self {
        self = self.border_color(colors::sky::C950);
        self
    }
    fn border_blue_50(mut self) -> Self {
        self = self.border_color(colors::blue::C50);
        self
    }
    fn border_blue_100(mut self) -> Self {
        self = self.border_color(colors::blue::C100);
        self
    }
    fn border_blue_200(mut self) -> Self {
        self = self.border_color(colors::blue::C200);
        self
    }
    fn border_blue_300(mut self) -> Self {
        self = self.border_color(colors::blue::C300);
        self
    }
    fn border_blue_400(mut self) -> Self {
        self = self.border_color(colors::blue::C400);
        self
    }
    fn border_blue_500(mut self) -> Self {
        self = self.border_color(colors::blue::C500);
        self
    }
    fn border_blue_600(mut self) -> Self {
        self = self.border_color(colors::blue::C600);
        self
    }
    fn border_blue_700(mut self) -> Self {
        self = self.border_color(colors::blue::C700);
        self
    }
    fn border_blue_800(mut self) -> Self {
        self = self.border_color(colors::blue::C800);
        self
    }
    fn border_blue_900(mut self) -> Self {
        self = self.border_color(colors::blue::C900);
        self
    }
    fn border_blue_950(mut self) -> Self {
        self = self.border_color(colors::blue::C950);
        self
    }
    fn border_indigo_50(mut self) -> Self {
        self = self.border_color(colors::indigo::C50);
        self
    }
    fn border_indigo_100(mut self) -> Self {
        self = self.border_color(colors::indigo::C100);
        self
    }
    fn border_indigo_200(mut self) -> Self {
        self = self.border_color(colors::indigo::C200);
        self
    }
    fn border_indigo_300(mut self) -> Self {
        self = self.border_color(colors::indigo::C300);
        self
    }
    fn border_indigo_400(mut self) -> Self {
        self = self.border_color(colors::indigo::C400);
        self
    }
    fn border_indigo_500(mut self) -> Self {
        self = self.border_color(colors::indigo::C500);
        self
    }
    fn border_indigo_600(mut self) -> Self {
        self = self.border_color(colors::indigo::C600);
        self
    }
    fn border_indigo_700(mut self) -> Self {
        self = self.border_color(colors::indigo::C700);
        self
    }
    fn border_indigo_800(mut self) -> Self {
        self = self.border_color(colors::indigo::C800);
        self
    }
    fn border_indigo_900(mut self) -> Self {
        self = self.border_color(colors::indigo::C900);
        self
    }
    fn border_indigo_950(mut self) -> Self {
        self = self.border_color(colors::indigo::C950);
        self
    }
    fn border_violet_50(mut self) -> Self {
        self = self.border_color(colors::violet::C50);
        self
    }
    fn border_violet_100(mut self) -> Self {
        self = self.border_color(colors::violet::C100);
        self
    }
    fn border_violet_200(mut self) -> Self {
        self = self.border_color(colors::violet::C200);
        self
    }
    fn border_violet_300(mut self) -> Self {
        self = self.border_color(colors::violet::C300);
        self
    }
    fn border_violet_400(mut self) -> Self {
        self = self.border_color(colors::violet::C400);
        self
    }
    fn border_violet_500(mut self) -> Self {
        self = self.border_color(colors::violet::C500);
        self
    }
    fn border_violet_600(mut self) -> Self {
        self = self.border_color(colors::violet::C600);
        self
    }
    fn border_violet_700(mut self) -> Self {
        self = self.border_color(colors::violet::C700);
        self
    }
    fn border_violet_800(mut self) -> Self {
        self = self.border_color(colors::violet::C800);
        self
    }
    fn border_violet_900(mut self) -> Self {
        self = self.border_color(colors::violet::C900);
        self
    }
    fn border_violet_950(mut self) -> Self {
        self = self.border_color(colors::violet::C950);
        self
    }
    fn border_purple_50(mut self) -> Self {
        self = self.border_color(colors::purple::C50);
        self
    }
    fn border_purple_100(mut self) -> Self {
        self = self.border_color(colors::purple::C100);
        self
    }
    fn border_purple_200(mut self) -> Self {
        self = self.border_color(colors::purple::C200);
        self
    }
    fn border_purple_300(mut self) -> Self {
        self = self.border_color(colors::purple::C300);
        self
    }
    fn border_purple_400(mut self) -> Self {
        self = self.border_color(colors::purple::C400);
        self
    }
    fn border_purple_500(mut self) -> Self {
        self = self.border_color(colors::purple::C500);
        self
    }
    fn border_purple_600(mut self) -> Self {
        self = self.border_color(colors::purple::C600);
        self
    }
    fn border_purple_700(mut self) -> Self {
        self = self.border_color(colors::purple::C700);
        self
    }
    fn border_purple_800(mut self) -> Self {
        self = self.border_color(colors::purple::C800);
        self
    }
    fn border_purple_900(mut self) -> Self {
        self = self.border_color(colors::purple::C900);
        self
    }
    fn border_purple_950(mut self) -> Self {
        self = self.border_color(colors::purple::C950);
        self
    }
    fn border_fuchsia_50(mut self) -> Self {
        self = self.border_color(colors::fuchsia::C50);
        self
    }
    fn border_fuchsia_100(mut self) -> Self {
        self = self.border_color(colors::fuchsia::C100);
        self
    }
    fn border_fuchsia_200(mut self) -> Self {
        self = self.border_color(colors::fuchsia::C200);
        self
    }
    fn border_fuchsia_300(mut self) -> Self {
        self = self.border_color(colors::fuchsia::C300);
        self
    }
    fn border_fuchsia_400(mut self) -> Self {
        self = self.border_color(colors::fuchsia::C400);
        self
    }
    fn border_fuchsia_500(mut self) -> Self {
        self = self.border_color(colors::fuchsia::C500);
        self
    }
    fn border_fuchsia_600(mut self) -> Self {
        self = self.border_color(colors::fuchsia::C600);
        self
    }
    fn border_fuchsia_700(mut self) -> Self {
        self = self.border_color(colors::fuchsia::C700);
        self
    }
    fn border_fuchsia_800(mut self) -> Self {
        self = self.border_color(colors::fuchsia::C800);
        self
    }
    fn border_fuchsia_900(mut self) -> Self {
        self = self.border_color(colors::fuchsia::C900);
        self
    }
    fn border_fuchsia_950(mut self) -> Self {
        self = self.border_color(colors::fuchsia::C950);
        self
    }
    fn border_pink_50(mut self) -> Self {
        self = self.border_color(colors::pink::C50);
        self
    }
    fn border_pink_100(mut self) -> Self {
        self = self.border_color(colors::pink::C100);
        self
    }
    fn border_pink_200(mut self) -> Self {
        self = self.border_color(colors::pink::C200);
        self
    }
    fn border_pink_300(mut self) -> Self {
        self = self.border_color(colors::pink::C300);
        self
    }
    fn border_pink_400(mut self) -> Self {
        self = self.border_color(colors::pink::C400);
        self
    }
    fn border_pink_500(mut self) -> Self {
        self = self.border_color(colors::pink::C500);
        self
    }
    fn border_pink_600(mut self) -> Self {
        self = self.border_color(colors::pink::C600);
        self
    }
    fn border_pink_700(mut self) -> Self {
        self = self.border_color(colors::pink::C700);
        self
    }
    fn border_pink_800(mut self) -> Self {
        self = self.border_color(colors::pink::C800);
        self
    }
    fn border_pink_900(mut self) -> Self {
        self = self.border_color(colors::pink::C900);
        self
    }
    fn border_pink_950(mut self) -> Self {
        self = self.border_color(colors::pink::C950);
        self
    }
    fn border_rose_50(mut self) -> Self {
        self = self.border_color(colors::rose::C50);
        self
    }
    fn border_rose_100(mut self) -> Self {
        self = self.border_color(colors::rose::C100);
        self
    }
    fn border_rose_200(mut self) -> Self {
        self = self.border_color(colors::rose::C200);
        self
    }
    fn border_rose_300(mut self) -> Self {
        self = self.border_color(colors::rose::C300);
        self
    }
    fn border_rose_400(mut self) -> Self {
        self = self.border_color(colors::rose::C400);
        self
    }
    fn border_rose_500(mut self) -> Self {
        self = self.border_color(colors::rose::C500);
        self
    }
    fn border_rose_600(mut self) -> Self {
        self = self.border_color(colors::rose::C600);
        self
    }
    fn border_rose_700(mut self) -> Self {
        self = self.border_color(colors::rose::C700);
        self
    }
    fn border_rose_800(mut self) -> Self {
        self = self.border_color(colors::rose::C800);
        self
    }
    fn border_rose_900(mut self) -> Self {
        self = self.border_color(colors::rose::C900);
        self
    }
    fn border_rose_950(mut self) -> Self {
        self = self.border_color(colors::rose::C950);
        self
    }
}
