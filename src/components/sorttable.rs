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

#[derive(Debug, Clone, PartialEq)]
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

impl std::fmt::Debug for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "None",
                Self::Element(_) => "Element",
                Self::String(_) => "String",
                Self::Integer(_) => "Integer",
                Self::UnsignedInteger(_) => "UnsignedInteger",
                _ => "Object(_)",
            },
        )
    }
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

    /// Provides a handle to the current sorted column index.
    /// Can be set to 0, will be updated to Self::default_sort if provided and valid
    #[props(default = use_signal(|| 0), into)]
    sorted_col_index: Signal<usize>,

    headers: Vec<String>,

    data: ReadSignal<Vec<SortableRow>>,
}

pub struct SortTableState {
    headers: Vec<String>,
    data: Vec<SortableRow>,
    sorted_col_index: Signal<usize>,
    sort_ascending: bool,
}

impl SortTableState {
    pub fn new(
        headers: Vec<String>,
        data: Vec<SortableRow>,
        current_sort_index: Signal<usize>,
    ) -> Self {
        SortTableState {
            headers,
            data,
            sort_ascending: true,
            sorted_col_index: current_sort_index,
        }
    }

    pub fn set_sorted_col_index(&mut self, sorted_col_index: usize) {
        self.sorted_col_index.set(sorted_col_index);
    }

    pub fn get_sorted_col_index(&self) -> usize {
        *self.sorted_col_index.read()
    }

    pub fn reverse_data(&mut self) {
        self.data.reverse();
    }

    pub fn toggle_sort_direction(&mut self) {
        self.sort_ascending = !self.sort_ascending;
    }

    pub fn set_sort_direction(&mut self, ascending: bool) {
        self.sort_ascending = ascending;
    }

    pub fn is_sort_ascending(&self) -> bool {
        self.sort_ascending
    }

    fn is_column_sortable(&self, column_index: usize) -> bool {
        self.data
            .first()
            .and_then(|row| row.get(column_index))
            .is_some_and(|cell| cell.sort_by != KeyType::None)
    }

    /// Set the default sort column based on its name
    ///
    /// If None or the column is not found, the first column will be sorted
    ///
    /// Else, the column will be
    pub fn set_default_sort(mut self, column_name: Option<String>) -> Self {
        let column_index = column_name
            .and_then(|col| self.headers.iter().position(|h| h == &col))
            .filter(|&idx| self.is_column_sortable(idx))
            .unwrap_or(0);

        self.sorted_col_index.set(column_index);

        if self.is_column_sortable(column_index) {
            sort_table_keytype(&mut self.data, |t: &SortableRow| {
                t.to_keytype()[column_index].clone()
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
        SortTableState::new(
            props.headers.clone(),
            props.data.read().clone(),
            props.sorted_col_index,
        )
        .set_default_sort(props.default_sort.clone())
    });
    use_effect(move || {
        state.set(
            SortTableState::new(
                props.headers.clone(),
                props.data.read().clone(),
                props.sorted_col_index,
            )
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
                                if !state.peek().is_column_sortable(index) {
                                    return;
                                }
                                let sorted_col_index = state.read().get_sorted_col_index();
                                if sorted_col_index == index {
                                    state.write().reverse_data();
                                    state.write().toggle_sort_direction();
                                } else {
                                    sort_table_keytype(
                                        &mut state.write().data,
                                        |t: &SortableRow| t.to_keytype()[index].clone(),
                                    );
                                    state.write().set_sort_direction(true);
                                }
                                state.write().set_sorted_col_index(index);
                            },
                            div { class: "sorttable-header-content",
                                p { {head.to_string()} }
                                if state.read().is_column_sortable(index)
                                    && state.read().get_sorted_col_index() == index
                                {
                                    Icon {
                                        class: "sorttable-icon",
                                        style: if state.read().is_sort_ascending() { "rotate: -180deg;" },
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
