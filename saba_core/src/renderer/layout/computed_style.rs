use crate::error::Error;
use crate::renderer::dom::node::ElementKind;
use crate::renderer::dom::node::Node;
use crate::renderer::dom::node::NodeKind;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use core::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedStyle {
    background_color: Option<Color>,
    color: Option<Color>,
    display: Option<DisplayType>,
    font_size: Option<FontSize>,
    text_decoration: Option<TextDecoration>,
    height: Option<i64>,
    width: Option<i64>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        Self {
            background_color: None,
            color: None,
            display: None,
            font_size: None,
            text_decoration: None,
            height: None,
            width: None,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = Some(color);
    }

    pub fn background_color(&self) -> Color {
        self.background_color
            .clone()
            .expect("failed to access CSS property: background-color")
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn color(&self) -> Color {
        self.color
            .clone()
            .expect("failed to access CSS property: color")
    }

    pub fn set_display(&mut self, display: DisplayType) {
        self.display = Some(display);
    }

    pub fn display(&self) -> DisplayType {
        self.display
            .clone()
            .expect("failed to access CSS property: display")
    }

    pub fn font_size(&self) -> FontSize {
        self.font_size
            .clone()
            .expect("failed to access CSS property: font-size")
    }

    pub fn text_decoration(&self) -> TextDecoration {
        self.text_decoration
            .clone()
            .expect("failed to access CSS property: text-decoration")
    }

    pub fn set_height(&mut self, height: i64) {
        self.height = Some(height);
    }

    pub fn height(&self) -> i64 {
        self.height
            .clone()
            .expect("failed to access CSS property: height")
    }

    pub fn set_width(&mut self, width: i64) {
        self.width = Some(width);
    }

    pub fn width(&self) -> i64 {
        self.width
            .clone()
            .expect("failed to access CSS property: width")
    }

    pub fn defaulting(&mut self, node: &Rc<RefCell<Node>>, parent_style: Option<ComputedStyle>) {
        if let Some(parent_style) = parent_style {
            if self.background_color.is_none() && parent_style.background_color() != Color::white()
            {
                self.background_color = Some(parent_style.background_color());
            }
            if self.color.is_none() && parent_style.color() != Color::black() {
                self.color = Some(parent_style.color());
            }
            if self.font_size.is_none() && parent_style.font_size() != FontSize::Medium {
                self.font_size = Some(parent_style.font_size());
            }
            if self.text_decoration.is_none()
                && parent_style.text_decoration() != TextDecoration::None
            {
                self.text_decoration = Some(parent_style.text_decoration());
            }
        }

        // 各プロパティに対して、初期値を設定する
        if self.background_color.is_none() {
            self.background_color = Some(Color::white());
        }

        if self.color.is_none() {
            self.color = Some(Color::black());
        }

        if self.display.is_none() {
            self.display = Some(DisplayType::deault(node));
        }

        if self.font_size.is_none() {
            self.font_size = Some(FontSize::default(node));
        }

        if self.text_decoration.is_none() {
            self.text_decoration = Some(TextDecoration::default(node));
        }

        if self.height.is_none() {
            self.height = Some(0);
        }

        if self.width.is_none() {
            self.width = Some(0);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    name: Option<String>,
    code: String,
}

impl Color {
    pub fn from_name(name: &str) -> Result<Self, Error> {
        let code = match name {
            "black" => "#000000".to_string(),
            "silver" => "#C0C0C0".to_string(),
            "gray" => "#808080".to_string(),
            "white" => "#FFFFFF".to_string(),
            "maroon" => "#800000".to_string(),
            "red" => "#FF0000".to_string(),
            "purple" => "#800080".to_string(),
            "fuchsia" => "#FF00FF".to_string(),
            "green" => "#008000".to_string(),
            "lime" => "#00FF00".to_string(),
            "olive" => "#808000".to_string(),
            "yellow" => "#FFFF00".to_string(),
            "navy" => "#000080".to_string(),
            "blue" => "#0000FF".to_string(),
            "teal" => "#008080".to_string(),
            "aqua" => "#00FFFF".to_string(),
            "orange" => "#FFA500".to_string(),
            "lightgray" => "#D3D3D3".to_string(),
            _ => {
                return Err(Error::UnexpectedInput(format!(
                    "color name: {} is not supported yet",
                    name
                )))
            }
        };

        Ok(Self {
            name: Some(name.to_string()),
            code,
        })
    }

    pub fn from_code(code: &str) -> Result<Self, Error> {
        if code.chars().nth(0) != Some('#') || code.len() != 7 {
            return Err(Error::UnexpectedInput(format!(
                "color code: {} is invalid",
                code
            )));
        }

        let name = match code {
            "#000000" => "black".to_string(),
            "#C0C0C0" => "silver".to_string(),
            "#808080" => "gray".to_string(),
            "#FFFFFF" => "white".to_string(),
            "#800000" => "maroon".to_string(),
            "#FF0000" => "red".to_string(),
            "#800080" => "purple".to_string(),
            "#FF00FF" => "fuchsia".to_string(),
            "#008000" => "green".to_string(),
            "#00FF00" => "lime".to_string(),
            "#808000" => "olive".to_string(),
            "#FFFF00" => "yellow".to_string(),
            "#000080" => "navy".to_string(),
            "#0000FF" => "blue".to_string(),
            "#008080" => "teal".to_string(),
            "#00FFFF" => "aqua".to_string(),
            "#FFA500" => "orange".to_string(),
            "#D3D3D3" => "lightgray".to_string(),
            _ => {
                return Err(Error::UnexpectedInput(format!(
                    "color code: {} is not supported yet",
                    code
                )))
            }
        };

        Ok(Self {
            name: Some(name),
            code: code.to_string(),
        })
    }

    pub fn white() -> Self {
        Self {
            name: Some("white".to_string()),
            code: "#FFFFFF".to_string(),
        }
    }

    pub fn black() -> Self {
        Self {
            name: Some("black".to_string()),
            code: "#000000".to_string(),
        }
    }

    pub fn code_u32(&self) -> u32 {
        u32::from_str_radix(&self.code.trim_start_matches('#'), 16).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontSize {
    Medium,
    XLarge,
    XXLarge,
}

impl FontSize {
    fn default(node: &Rc<RefCell<Node>>) -> Self {
        match &node.borrow().kind() {
            NodeKind::Element(element) => match element.kind() {
                ElementKind::H1 => FontSize::XXLarge,
                ElementKind::H2 => FontSize::XLarge,
                _ => FontSize::Medium,
            },
            _ => FontSize::Medium,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayType {
    Block,
    Inline,
    DisplayNone,
}

impl DisplayType {
    fn deault(node: &Rc<RefCell<Node>>) -> Self {
        match &node.borrow().kind() {
            NodeKind::Document => DisplayType::Block,
            NodeKind::Element(e) => {
                if e.is_block_element() {
                    DisplayType::Block
                } else {
                    DisplayType::Inline
                }
            }
            NodeKind::Text(_) => DisplayType::Inline,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "block" => Ok(Self::Block),
            "inline" => Ok(Self::Inline),
            "none" => Ok(Self::DisplayNone),
            _ => Err(Error::UnexpectedInput(format!(
                "display type: {} is not supported yet",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextDecoration {
    None,
    Underline,
}

impl TextDecoration {
    fn default(node: &Rc<RefCell<Node>>) -> Self {
        match &node.borrow().kind() {
            NodeKind::Element(element) => match element.kind() {
                ElementKind::A => TextDecoration::Underline,
                _ => TextDecoration::None,
            },
            _ => TextDecoration::None,
        }
    }
}
