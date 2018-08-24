use std::fmt::{self, Display, Formatter};

use serde::de::{self, Deserialize, Deserializer, Visitor};

#[derive(Clone)]
pub struct ConditionField(pub String);
pub struct ConditionList(pub Vec<Condition>);

struct FieldVisitor;

const  STATS: [&'static str; 13] = ["Strength", "Intelligence", "Presence",
                           "Dexterity", "Wits", "Manipulation",
                           "Stamina", "Resolve", "Composure",
                           "Defense", "Willpower", "Blood Potency",
                           "Humanity"];

#[derive(Clone, Deserialize)]
pub struct Condition {
    name: ConditionField,
    flavor: ConditionField,
    effect: ConditionField,
    beat: Option<ConditionField>,
    resolution: ConditionField,
    num: Option<u32>
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let num = match self.num {
            Some(i) => i,
            None => 1
        };

        for _ in 0..num {
            write!(f, "<div class='condition'>")?;
            write!(f, "<span class='name'>{}</span>", self.name)?;
            write!(f, "<span class='flavor'>{}</span>", self.flavor)?;
            write!(f, "<span class='effect'><b>Effect: </b>{}</span>", self.effect)?;
            if let Some(ref beat) = self.beat {
                write!(f, "<span class='beat'><b>Beat: </b>{}</span>", beat)?;
            }
            write!(f, "<span class='resolution'><b>Resolution: </b>{}</span>", self.resolution)?;
            write!(f, "</div>")?;
        }

        Ok(())
    }
}

impl Display for ConditionList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for condition in &self.0 {
            write!(f, "{}", condition)?
        }
        Ok(())
    }
}

impl Display for ConditionField {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut text = self.0.replace("_", "<span class='empty'></span>");
        for stat in STATS.iter() {
            text = text.replace(stat, &format!("<b>{}</b>", stat))
        }
        write!(f, "{}", text)
    }
}

impl<'de> Visitor<'de> for FieldVisitor {
    type Value = ConditionField;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<ConditionField, E>
    where
        E: de::Error,
    {
        Ok(ConditionField(String::from(value)))
    }
}

impl<'de> Deserialize<'de> for ConditionField {
    fn deserialize<D>(deserializer: D) -> Result<ConditionField, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FieldVisitor)
    }
}