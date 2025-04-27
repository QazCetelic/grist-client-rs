use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateColumnsColumnsInnerFields {
    /// Column type, by default Any. Ref, RefList and DateTime types requires a postfix, e.g. <code>DateTime:America/New_York</code>, <code>Ref:Users</code>
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Column label.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// A python formula, e.g.: <code>$A + Table1.lookupOne(B=$B)</code>
    #[serde(rename = "formula", skip_serializing_if = "Option::is_none")]
    pub formula: Option<String>,
    /// Use \"true\" to indicate that the column is a formula column. Use \"false\" for trigger formula column.
    #[serde(rename = "isFormula", skip_serializing_if = "Option::is_none")]
    pub is_formula: Option<bool>,
    /// A JSON object with widget options, e.g.: <code>{\"choices\": [\"cat\", \"dog\"], \"alignment\": \"right\"}</code>
    #[serde(rename = "widgetOptions", skip_serializing_if = "Option::is_none")]
    pub widget_options: Option<String>,
    /// Use \"true\" to indicate that the column label should not be used as the column identifier. Use \"false\" to use the label as the identifier.
    #[serde(rename = "untieColIdFromLabel", skip_serializing_if = "Option::is_none")]
    pub untie_col_id_from_label: Option<bool>,
    /// A number indicating when the column should be recalculated. <ol start='0'><li>On new records or when any field in recalcDeps changes, it's a 'data-cleaning'.</li><li>Never.</li><li>Calculate on new records and on manual updates to any data field.</li></ol>
    #[serde(rename = "recalcWhen", skip_serializing_if = "Option::is_none")]
    pub recalc_when: Option<i32>,
    /// For Ref and RefList columns, the colRef of a column to display
    #[serde(rename = "visibleCol", skip_serializing_if = "Option::is_none")]
    pub visible_col: Option<GristId>,
    /// An encoded array of column identifiers (colRefs) that this column depends on. If any of these columns change, the column will be recalculated. E.g.: <code>[2, 3]</code>
    #[serde(rename = "recalcDeps", skip_serializing_if = "Option::is_none")]
    pub recalc_deps: Option<String>,
    /// Set it to the new column ID when you want to change it.
    #[serde(rename = "colId", skip_serializing_if = "Option::is_none")]
    pub col_id: Option<String>,
}

impl UpdateColumnsColumnsInnerFields {
    pub fn new() -> UpdateColumnsColumnsInnerFields {
        UpdateColumnsColumnsInnerFields {
            r#type: None,
            label: None,
            formula: None,
            is_formula: None,
            widget_options: None,
            untie_col_id_from_label: None,
            recalc_when: None,
            visible_col: None,
            recalc_deps: None,
            col_id: None,
        }
    }
}
/// Column type, by default Any. Ref, RefList and DateTime types requires a postfix, e.g. <code>DateTime:America/New_York</code>, <code>Ref:Users</code>
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Any")]
    Any,
    #[serde(rename = "Text")]
    Text,
    #[serde(rename = "Numeric")]
    Numeric,
    #[serde(rename = "Int")]
    Int,
    #[serde(rename = "Bool")]
    Bool,
    #[serde(rename = "Date")]
    Date,
    #[serde(rename = "DateTime:<timezone>")]
    DateTimeColonLessThanTimezoneGreaterThan,
    #[serde(rename = "Choice")]
    Choice,
    #[serde(rename = "ChoiceList")]
    ChoiceList,
    #[serde(rename = "Ref:<tableId>")]
    RefColonLessThanTableIdGreaterThan,
    #[serde(rename = "RefList:<tableId>")]
    RefListColonLessThanTableIdGreaterThan,
    #[serde(rename = "Attachments")]
    Attachments,
}

impl Default for Type {
    fn default() -> Type {
        Self::Any
    }
}

