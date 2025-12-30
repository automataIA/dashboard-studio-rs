/// Data structure for dataset items in the sidebar
#[derive(Clone, Debug, PartialEq)]
pub struct DatasetItemData {
    pub id: String,
    pub name: String,
    pub metadata: String,
    pub active: bool,
    pub fields: Vec<FieldData>,
}

/// Data structure for field items within datasets
#[derive(Clone, Debug, PartialEq)]
pub struct FieldData {
    pub label: String,
    pub icon: String, // CSS class for icon color
}

/// Data structure for template cards
#[derive(Clone, Debug, PartialEq)]
pub struct TemplateData {
    pub id: String,              // Direct template ID for easier access
    pub title: String,
    pub preview_type: String,
    pub category_name: String,   // For filtering
    pub has_kpi: bool,           // KPI detection flag for Quick Actions
}

/// Data structure for layer items in the right sidebar
#[derive(Clone, Debug, PartialEq)]
pub struct LayerData {
    pub id: String,
    pub label: String,
    pub icon: &'static str, // IconName reference as string
    pub active: bool,
}

/// Data structure for widgets in the canvas grid
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetData {
    pub widget_type: WidgetType,
    pub title: String,
    pub subtitle: Option<String>,
    pub editing: bool,
}

/// Widget type enumeration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WidgetType {
    LineChart,
    KPI,
    PieChart,
    AddNew,
}
