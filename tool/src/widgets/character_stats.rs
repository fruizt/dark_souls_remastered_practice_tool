use libdsr::prelude::*;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::stats_editor::{Datum, Stats, StatsEditor};
use practice_tool_core::widgets::Widget;

#[derive(Debug)]
struct CharacterStatsEdit {
    ptr: PointerChain<CharacterStats>,
    stats: Option<CharacterStats>,
}

impl Stats for CharacterStatsEdit {
    fn data(&mut self) -> Option<impl Iterator<Item = Datum>> {
        self.stats.as_mut().map(|s| {
            [
                Datum::int("Level", &mut s.level, 1, i32::MAX),
                Datum::int("Souls", &mut s.souls, 1, i32::MAX),
                Datum::int("Vitality", &mut s.vitality, 1, 99),
                Datum::int("Attunement", &mut s.attunement, 1, 99),
                Datum::int("Endurance", &mut s.endurance, 1, 99),
                Datum::int("Strength", &mut s.strength, 1, 99),
                Datum::int("Dexterity", &mut s.dexterity, 1, 99),
                Datum::int("Resistance", &mut s.resistance, 1, 99),
                Datum::int("Intelligence", &mut s.intelligence, 1, 99),
                Datum::int("Faith", &mut s.faith, 1, 99),
                Datum::int("Humanity", &mut s.humanity, 1, 99),
            ]
            .into_iter()
        })
    }

    fn read(&mut self) {
        self.stats = self.ptr.read();
    }

    fn write(&mut self) {
        if let Some(stats) = self.stats.clone() {
            self.ptr.write(stats);
        }
    }

    fn clear(&mut self) {
        self.stats = None;
    }
}

pub(crate) fn character_stats_edit(
    character_stats: PointerChain<CharacterStats>,
    key_open: Option<Key>,
    key_close: Key,
) -> Box<dyn Widget> {
    Box::new(StatsEditor::new(
        CharacterStatsEdit {
            ptr: character_stats,
            stats: None,
        },
        key_open,
        Some(key_close),
    ))
}
