use sailfish::TemplateOnce;

pub type List = (i32, String, String, String);
pub type BangEntry = (String, String);

/// Index page without list credentials
#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index;

/// List view with edit credentials
#[derive(TemplateOnce)]
#[template(path = "editview.stpl")]
pub struct EditView {
    pub list: List,
    pub bangs: Vec<BangEntry>,
}

/// List view with read credentials
#[derive(TemplateOnce)]
#[template(path = "readview.stpl")]
pub struct ReadView {
    pub list: List,
    pub bangs: Vec<BangEntry>,
}

/// Generic view for error pages
#[derive(TemplateOnce)]
#[template(path = "error.stpl")]
pub struct BangleError {
    pub err: &'static str,
}