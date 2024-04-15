// TODO a special shroom_fmt! proc macro would be really helpful
// to ensure the \r\n line breaks and format options which are actually checked

use crate::id::{FieldId, ItemId, MobId, NpcId, SkillId};

macro_rules! impl_shroom_fmt {
    ($t:ty, $fmt:expr, id) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $fmt, self.0 .0)
            }
        }

        impl ShroomDisplay for $t {}
    };

    ($t:ty, $fmt:expr) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $fmt, self.0)
            }
        }

        impl ShroomDisplay for $t {}
    };
}

macro_rules! shroom_fmt_color {
    ($color:ident, $fmt:expr, $end_fmt:expr) => {
        pub struct $color<T>(pub T);

        impl<T: ShroomDisplay> std::fmt::Display for $color<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "#{}{}#{}", $fmt, self.0, $end_fmt)
            }
        }

        impl<T: ShroomDisplay> ShroomDisplay for $color<T> {}
    };
    ($color:ident, $fmt:expr) => {
        pub struct $color<T>(pub T);

        impl<T: ShroomDisplay> std::fmt::Display for $color<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "#{}{}", $fmt, self.0)
            }
        }

        impl<T: ShroomDisplay> ShroomDisplay for $color<T> {}
    };
}

pub trait ShroomDisplay: std::fmt::Display + Sized {
    fn black(self) -> Black<Self> {
        Black(self)
    }

    fn purple(self) -> Purple<Self> {
        Purple(self)
    }

    fn blue(self) -> Blue<Self> {
        Blue(self)
    }

    fn green(self) -> Green<Self> {
        Green(self)
    }

    fn red(self) -> Red<Self> {
        Red(self)
    }

    fn bold(self) -> Bold<Self> {
        Bold(self)
    }

    fn normal(self) -> Normal<Self> {
        Normal(self)
    }

    fn to_shroom_string(&self) -> String {
        self.to_string().replace('\n', "\r\n")
    }
}

impl<'a> ShroomDisplay for &'a str {}
impl ShroomDisplay for String {}
impl ShroomDisplay for f32 {}
impl ShroomDisplay for f64 {}
impl ShroomDisplay for u32 {}
impl ShroomDisplay for i32 {}
impl ShroomDisplay for usize {}
impl ShroomDisplay for isize {}

impl<'a, T> ShroomDisplay for &'a T where T: ShroomDisplay {}

/*

    Missing ones:
    * t,v Item Icon/Name
    * B progress bar
    * F image location from wz files
    * f other image from wz
    * x
*/

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum MenuStyle {
    List,
    Inline,
}

pub struct ShroomMenuList<T> {
    items: Vec<T>,
    style: MenuStyle,
}

impl<T: ShroomDisplay> From<Vec<T>> for ShroomMenuList<T> {
    fn from(value: Vec<T>) -> Self {
        Self::from_iter(value)
    }
}

impl<T: ShroomDisplay> FromIterator<T> for ShroomMenuList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
            style: MenuStyle::List,
        }
    }
}

impl<T> ShroomMenuList<T> {
    pub fn new(items: Vec<T>, style: MenuStyle) -> Self {
        Self { items, style }
    }

    pub fn list(items: Vec<T>) -> Self {
        Self::new(items, MenuStyle::List)
    }

    pub fn inline(items: Vec<T>) -> Self {
        Self::new(items, MenuStyle::Inline)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: ShroomDisplay> std::fmt::Display for ShroomMenuList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.style == MenuStyle::Inline {
            for (i, item) in self.items.iter().enumerate() {
                write!(f, "{}", ShroomMenuItem(i, item))?;
            }
        } else {
            for (i, item) in self.items.iter().enumerate() {
                writeln!(f, "{}", ShroomMenuItem(i, item))?;
            }
        }

        Ok(())
    }
}
impl<T: ShroomDisplay> ShroomDisplay for ShroomMenuList<T> {}

pub struct ShroomMenuItem<T>(pub usize, pub T);

impl<T: ShroomDisplay> std::fmt::Display for ShroomMenuItem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#L{}#{}#l", self.0, self.1)
    }
}
impl<T: ShroomDisplay> ShroomDisplay for ShroomMenuItem<T> {}

pub struct PlayerName;
impl std::fmt::Display for PlayerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#h#")
    }
}
impl ShroomDisplay for PlayerName {}

pub struct ItemCount(pub ItemId);
impl_shroom_fmt!(ItemCount, "#c{}#", id);

/// Regular icon, with name tooltip
pub struct ItemIcon(pub ItemId);
impl_shroom_fmt!(ItemIcon, "#i{}#", id);

/// Detailed icon, with stats tooltip
pub struct ItemDetailIcon(pub ItemId);
impl_shroom_fmt!(ItemDetailIcon, "#z{}#", id);

/// Item name
pub struct ItemName(pub ItemId);
impl_shroom_fmt!(ItemName, "#t{}#", id);

impl std::fmt::Display for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() {
            ItemDetailIcon(*self).fmt(f)
        } else {
            ItemIcon(*self).fmt(f)
        }
    }
}

// TODO item v

pub struct MapName(pub FieldId);
impl_shroom_fmt!(MapName, "#m{}#", id);

impl std::fmt::Display for FieldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        MapName(*self).fmt(f)
    }
}

pub struct SkillIcon(pub SkillId);
impl_shroom_fmt!(SkillIcon, "#s{}#", id);

pub struct SkillName(pub SkillId);
impl_shroom_fmt!(SkillName, "#q{}#", id);

impl std::fmt::Display for SkillId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() {
            SkillIcon(*self).fmt(f)
        } else {
            SkillName(*self).fmt(f)
        }
    }
}

pub struct NpcName(pub NpcId);
impl_shroom_fmt!(NpcName, "#p{}#");

pub struct MobName(pub MobId);
impl_shroom_fmt!(MobName, "#o{}#");

shroom_fmt_color!(Black, "k");
shroom_fmt_color!(Blue, "b", "k");
shroom_fmt_color!(Green, "g", "k");
shroom_fmt_color!(Purple, "d", "k");
shroom_fmt_color!(Red, "r", "k");
shroom_fmt_color!(Bold, "e", "n");
shroom_fmt_color!(Normal, "n");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu() {
        let menu: ShroomMenuList<u32> = (0..2).collect();
        assert_eq!(menu.to_shroom_string(), "#L0#0#l\r\n#L1#1#l\r\n");
    }

    #[test]
    fn multi_line() {
        let item = ItemId(100);
        let n = 7u32;
        let fmt = format!(
            r#"{item}
Hello
{n}"#
        );

        assert_eq!(fmt.to_shroom_string(), "#i100#\r\nHello\r\n7");
    }
}
