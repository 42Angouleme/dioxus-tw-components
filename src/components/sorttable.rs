use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SortableRow(Vec<SortableCell>);
impl SortableRow {
    pub fn new(cells: Vec<SortableCell>) -> Self {
        SortableRow(cells)
    }
}
impl std::ops::Deref for SortableRow {
    type Target = Vec<SortableCell>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for SortableRow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ToTableData for SortableRow {
    fn headers_to_strings() -> Vec<impl ToString> {
        vec![""]
    }

    fn to_keytype(&self) -> Vec<&KeyType> {
        self.iter().map(|cell| &cell.sort_by).collect()
    }
}

#[derive(Clone, PartialEq)]
pub struct SortableCell {
    content: Element,
    style: String,
    sort_by: KeyType,
}
impl SortableCell {
    pub fn new(content: Element) -> Self {
        SortableCell {
            content,
            style: String::new(),
            sort_by: KeyType::None,
        }
    }

    pub fn sort_by(mut self, sort_by: KeyType) -> Self {
        self.sort_by = sort_by;
        self
    }

    pub fn style(mut self, style: impl ToString) -> Self {
        self.style = style.to_string();
        self
    }
}

pub trait Sortable: ToString + Clonable {
    fn to_sortable(&self) -> KeyType {
        KeyType::String(self.to_string())
    }
}

impl Clone for Box<dyn Sortable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait Clonable {
    fn clone_box(&self) -> Box<dyn Sortable>;
}

impl<T: Clone + Sortable + 'static> Clonable for T {
    fn clone_box(&self) -> Box<dyn Sortable> {
        Box::new(self.clone())
    }
}

pub trait ToTableData {
    fn headers_to_strings() -> Vec<impl ToString>;
    fn to_keytype(&self) -> Vec<&KeyType>;
}

// Used to change the sorting type of the data (eg if a field is number we will not sort the same way as string)
#[derive(Clone)]
pub enum KeyType {
    None,
    Element(Element),
    String(String),
    Integer(i128),
    UnsignedInteger(u128),
    Object(Box<dyn Sortable>),
}

impl From<&str> for KeyType {
    fn from(str: &str) -> Self {
        KeyType::String(str.to_string())
    }
}

impl From<String> for KeyType {
    fn from(str: String) -> Self {
        KeyType::String(str)
    }
}

impl From<i128> for KeyType {
    fn from(nb: i128) -> Self {
        KeyType::Integer(nb)
    }
}

impl From<u128> for KeyType {
    fn from(nb: u128) -> Self {
        KeyType::UnsignedInteger(nb)
    }
}

impl From<i64> for KeyType {
    fn from(nb: i64) -> Self {
        KeyType::Integer(nb.into())
    }
}

impl From<u64> for KeyType {
    fn from(nb: u64) -> Self {
        KeyType::UnsignedInteger(nb.into())
    }
}

impl From<i32> for KeyType {
    fn from(nb: i32) -> Self {
        KeyType::Integer(nb.into())
    }
}

impl From<u32> for KeyType {
    fn from(nb: u32) -> Self {
        KeyType::UnsignedInteger(nb.into())
    }
}

impl From<i16> for KeyType {
    fn from(nb: i16) -> Self {
        KeyType::Integer(nb.into())
    }
}

impl From<u16> for KeyType {
    fn from(nb: u16) -> Self {
        KeyType::UnsignedInteger(nb.into())
    }
}

impl From<i8> for KeyType {
    fn from(nb: i8) -> Self {
        KeyType::Integer(nb.into())
    }
}

impl From<u8> for KeyType {
    fn from(nb: u8) -> Self {
        KeyType::UnsignedInteger(nb.into())
    }
}

impl PartialEq for KeyType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KeyType::None, KeyType::None) => true,
            (KeyType::String(a), KeyType::String(b)) => a == b,
            (KeyType::Integer(a), KeyType::Integer(b)) => a == b,
            (KeyType::UnsignedInteger(a), KeyType::UnsignedInteger(b)) => a == b,
            (KeyType::Object(a), KeyType::Object(b)) => a.to_sortable() == b.to_sortable(),
            _ => false,
        }
    }
}

impl Eq for KeyType {}

impl PartialOrd for KeyType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KeyType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (KeyType::String(a), KeyType::String(b)) => a.cmp(b),
            (KeyType::Integer(a), KeyType::Integer(b)) => b.cmp(a),
            (KeyType::UnsignedInteger(a), KeyType::UnsignedInteger(b)) => b.cmp(a),
            (KeyType::Object(a), KeyType::Object(b)) => a.to_sortable().cmp(&b.to_sortable()),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyType::None => {
                write!(f, "None")
            }
            KeyType::String(str) => {
                write!(f, "{str}")
            }
            KeyType::Integer(nb) => {
                write!(f, "{nb}")
            }
            KeyType::UnsignedInteger(nb) => {
                write!(f, "{nb}")
            }
            KeyType::Object(obj) => {
                write!(f, "{}", obj.to_string())
            }
            _ => write!(f, ""),
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SortTableProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, into)]
    header_class: Option<String>,

    #[props(optional, into)]
    row_class: Option<String>,

    #[props(optional, into)]
    cell_class: Option<String>,

    /// The default sort column (header name)
    /// If not set, the first column will be sorted
    #[props(optional, into)]
    default_sort: Option<String>,

    headers: Vec<String>,

    data: ReadOnlySignal<Vec<SortableRow>>,
}

pub struct SortTableState {
    headers: Vec<String>,
    data: Vec<SortableRow>,
    sorted_col_index: usize,
    sort_ascending: bool,
}

impl SortTableState {
    pub fn new(headers: Vec<String>, data: Vec<SortableRow>) -> Self {
        SortTableState {
            headers,
            data,
            sorted_col_index: 0,
            sort_ascending: true,
        }
    }

    pub fn set_sorted_col_index(&mut self, sorted_col_index: usize) {
        self.sorted_col_index = sorted_col_index;
    }

    pub fn get_sorted_col_index(&self) -> usize {
        self.sorted_col_index
    }

    pub fn reverse_data(&mut self) {
        self.data.reverse();
    }

    pub fn toggle_sort_ascending(&mut self) {
        self.sort_ascending = !self.sort_ascending;
    }

    pub fn set_sort_ascending(&mut self, sort_ascending: bool) {
        self.sort_ascending = sort_ascending;
    }

    pub fn get_sort_ascending(&self) -> bool {
        self.sort_ascending
    }

    /// Set the default sort column
    ///
    /// If the column is not found, the first column will be sorted
    ///
    /// Else, the column will be sorted
    pub fn set_default_sort(mut self, default_sorted_column: Option<String>) -> Self {
        if let Some(default_sorted_column) = default_sorted_column
            && let Some(index) = self
                .headers
                .iter()
                .position(|header| header == &default_sorted_column)
        {
            self.sorted_col_index = index;
            sort_table_keytype(&mut self.data, |t: &SortableRow| {
                t.to_keytype()[index].clone()
            });
        }
        self
    }
}

fn sort_table_keytype<F>(data: &mut [SortableRow], key_extractor: F)
where
    F: Fn(&SortableRow) -> KeyType,
{
    data.sort_by_key(key_extractor);
}

#[component]
pub fn SortTable(mut props: SortTableProps) -> Element {
    let default_classes = "sorttable";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let mut state = use_signal(|| {
        SortTableState::new(props.headers.clone(), props.data.read().clone())
            .set_default_sort(props.default_sort.clone())
    });
    use_effect(move || {
        state.set(
            SortTableState::new(props.headers.clone(), props.data.read().clone())
                .set_default_sort(props.default_sort.clone()),
        );
    });

    let header_class = format!(
        "sorttable-header {}",
        props.header_class.unwrap_or("".to_string())
    );

    rsx! {
        table {..props.attributes,
            TableHeader {
                TableRow {
                    for (index , head) in state.read().headers.iter().enumerate() {
                        th {
                            class: "table-head {header_class}",
                            onclick: move |_| {
                                if state
                                    .read()
                                    .data
                                    .first()
                                    .is_some_and(|row| {
                                        row.get(index).is_some_and(|cell| cell.sort_by == KeyType::None)
                                    })
                                {
                                    return;
                                }
                                let sorted_col_index = state.read().get_sorted_col_index();
                                if sorted_col_index == index {
                                    state.write().reverse_data();
                                    state.write().toggle_sort_ascending();
                                } else {
                                    sort_table_keytype(
                                        &mut state.write().data,
                                        |t: &SortableRow| t.to_keytype()[index].clone(),
                                    );
                                    state.write().set_sort_ascending(true);
                                }
                                state.write().set_sorted_col_index(index);
                            },
                            div { class: "sorttable-header-content",
                                p { {head.to_string()} }
                                if state
                                    .read()
                                    .data
                                    .first()
                                    .is_some_and(|row| {
                                        row.get(index).is_some_and(|cell| cell.sort_by != KeyType::None)
                                    }) && state.read().get_sorted_col_index() == index
                                {
                                    Icon {
                                        class: "sorttable-icon",
                                        style: if state.read().get_sort_ascending() { "rotate: -180deg;" },
                                        icon: Icons::ExpandMore,
                                    }
                                }
                            }
                        }
                    }
                }
            }
            TableBody {
                for data in state.read().data.iter() {
                    TableRow { class: r#"{props.row_class.clone().unwrap_or("".to_string())}"#,
                        for field in data.iter() {
                            TableCell { class: format!("{} {}", props.cell_class.clone().unwrap_or("".to_string()), field.style),
                                {field.content.clone()}
                            }
                        }
                    }
                }
            }
        }
    }
}
