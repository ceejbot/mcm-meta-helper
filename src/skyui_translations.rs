//! The list of translation keys that SkyUI itself provides.
//! These are not translations mods need to provide, so we need
//! to include them when looking for all available translations.

use std::collections::HashSet;

use once_cell::sync::Lazy;

pub fn filter_skyui_translations(input: &[String]) -> Vec<String> {
    input
        .iter()
        .filter_map(|xs| {
            if !SKYUI_KEYS.contains(xs) {
                Some(xs.clone())
            } else {
                None
            }
        })
        .collect()
}

pub static SKYUI_KEYS: Lazy<HashSet<String>> = Lazy::new(|| {
    HashSet::from_iter(
        vec![
            "$3D",
            "$Active",
            "$Advanced",
            "$Aetherium",
            "$AID",
            "$Align",
            "$ALL",
            "$Alteration",
            "$AMMO",
            "$Amulet",
            "$ARM",
            "$Armor",
            "$Arrow",
            "$Artifact",
            "$B.ARM",
            "$B.DAM",
            "$Barter",
            "$BASE",
            "$BASE",
            "$Battleaxe",
            "$Body",
            "$Bolt",
            "$Bonemold",
            "$Book",
            "$Bottom",
            "$Bow",
            "$Brotherhood",
            "$Calves",
            "$Category",
            "$Center",
            "$Chitin",
            "$Circlet",
            "$CLASS",
            "$Claw",
            "$Clothing",
            "$Clutter",
            "$Column",
            "$Common",
            "$Confirm",
            "$Container",
            "$COOLDOWN",
            "$COVR",
            "$Crafting",
            "$Crossbow",
            "$Daedric",
            "$Dagger",
            "$DAM",
            "$Dawnguard",
            "$Deathbrand",
            "$Default",
            "$Defaults",
            "$Destruction",
            "$Disable",
            "$Dragonbone",
            "$Dragonplate",
            "$Dragonscale",
            "$Draugr",
            "$Drink",
            "$DUR",
            "$DURATION",
            "$Dwarven",
            "$Ears",
            "$Ebony",
            "$EFFECT",
            "$Elven",
            "$Enabled",
            "$ENCHANTED",
            "$Equip",
            "$EQUIPPED",
            "$EXPPROT",
            "$EXPPROTLOW",
            "$F.WRMT",
            "$Falmer",
            "$Favorite",
            "$FAVORITE",
            "$Favorites",
            "$Feet",
            "$FILTER",
            "$Find",
            "$Firewood",
            "$FIRST",
            "$Font",
            "$Food",
            "$Forearms",
            "$Forsworn",
            "$FORTIFY",
            "$FSForget",
            "$Fur",
            "$GEAR",
            "$Gem",
            "$General",
            "$Gift",
            "$Glass",
            "$Gold",
            "$Grand",
            "$Greater",
            "$Greater",
            "$Greater",
            "$Greatsword",
            "$Group",
            "$GROUP",
            "$Gun",
            "$Halberd",
            "$Hands",
            "$Head",
            "$Heavy",
            "$Hide",
            "$HNGR",
            "$Horizontal",
            "$House",
            "$HUNGER",
            "$Hunter",
            "$Icon",
            "$IJBag",
            "$IJBracelet",
            "$IJChoker",
            "$IJCrown",
            "$IJEar",
            "$IJEarrings",
            "$IJNecklace",
            "$IJTorc",
            "$Illusion",
            "$Imperial",
            "$Ingot",
            "$Ingredient",
            "$Input",
            "$Inventory",
            "$Iron",
            "$Item",
            "$Javelin",
            "$Jewelry",
            "$Katana",
            "$Key",
            "$Leather",
            "$Left",
            "$Lesser",
            "$Lesser",
            "$Light",
            "$Lockpick",
            "$Mace",
            "$MAG",
            "$Magic",
            "$Magical",
            "$MAGNITUDE",
            "$Map",
            "$Mask",
            "$MAT",
            "$MATERIAL",
            "$MCMMenuName",
            "$Melee",
            "$Minimum",
            "$Misc",
            "$MOD",
            "$Morag",
            "$Next",
            "$Nightingale",
            "$None",
            "$Nordic",
            "$Note",
            "$Off",
            "$On",
            "$Open",
            "$Orcish",
            "$Order",
            "$Ore",
            "$Orientation",
            "$Other",
            "$Petty",
            "$Pick",
            "$Pickaxe",
            "$Pike",
            "$Poison",
            "$Potion",
            "$Preferences",
            "$Previous",
            "$Quantity",
            "$Quarterstaff",
            "$R.COLD",
            "$RAINPROT",
            "$RAINPROTLOW",
            "$Rapier",
            "$RCH",
            "$REACH",
            "$Ready",
            "$Recipe",
            "$Remains",
            "$Remap",
            "$Restoration",
            "$RESTORE",
            "$Right",
            "$Ring",
            "$Save",
            "$Scale",
            "$Scaled",
            "$SCHOOL",
            "$Scroll",
            "$Scythe",
            "$Search",
            "$SECOND",
            "$Select",
            "$Set",
            "$Shield",
            "$SHOUTS",
            "$Show",
            "$Silver",
            "$SKI_INFO1{}",
            "$SKI_INFO2{}",
            "$SKI_INFO3{}",
            "$SKI_INFO4{}",
            "$SKI_INFO5{}",
            "$SKI_INFO6",
            "$SKI_INFO7{}",
            "$SKI_INFO8{}",
            "$SKI_INFO9{}",
            "$SKI_MSG1",
            "$SKI_MSG2{}",
            "$SKILL",
            "$SLOT",
            "$Soul",
            "$SOURCE",
            "$SPD",
            "$Spear",
            "$SPEED",
            "$Spell",
            "$SPELL",
            "$Staff",
            "$STAGGER",
            "$Stalhrim",
            "$Steel",
            "$STGR",
            "$STOLEN",
            "$Stormcloak",
            "$Strips",
            "$Studded",
            "$SWF",
            "$Switch",
            "$Sword",
            "$T.WGT",
            "$Tail",
            "$THIRD",
            "$THIRST",
            "$TIME",
            "$Toggle",
            "$Tool",
            "$Top",
            "$Torch",
            "$TOTAL",
            "$Toy",
            "$TRST",
            "$TYPE",
            "$Unequip",
            "$Ungroup",
            "$Unmap",
            "$V/W",
            "$VAL",
            "$VALUE/WEIGHT",
            "$Vampire",
            "$Vertical",
            "$War",
            "$Warhammer",
            "$WARMTH",
            "$Weapon",
            "$WEAPONS",
            "$WGT",
            "$Whip",
            "$Wood",
            "$WRMT",
        ]
        .iter()
        .map(|xs| xs.to_string()),
    )
});
