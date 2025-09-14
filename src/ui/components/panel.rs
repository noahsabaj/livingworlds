use super::super::{colors, dimensions};
use bevy::prelude::*;

/// Component for panels/containers
#[derive(Component, Debug)]
pub struct Panel {
    pub style: PanelStyle,
}

/// Panel style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PanelStyle {
    #[default]
    Default, // Standard panel
    Elevated,    // With shadow/depth
    Transparent, // No background
    Dark,        // Dark background
    Light,       // Light background
    Bordered,    // With visible border
}

impl PanelStyle {
    pub fn background_color(&self) -> Color {
        match self {
            PanelStyle::Default => colors::BACKGROUND_MEDIUM,
            PanelStyle::Elevated => colors::BACKGROUND_LIGHT,
            PanelStyle::Transparent => Color::NONE,
            PanelStyle::Dark => colors::BACKGROUND_DARK,
            PanelStyle::Light => colors::BACKGROUND_LIGHT,
            PanelStyle::Bordered => colors::BACKGROUND_MEDIUM,
        }
    }

    pub fn border_color(&self) -> Color {
        match self {
            PanelStyle::Bordered => colors::BORDER_DEFAULT,
            _ => Color::NONE,
        }
    }

    pub fn border_width(&self) -> Val {
        match self {
            PanelStyle::Bordered => Val::Px(dimensions::BORDER_WIDTH),
            _ => Val::Px(0.0),
        }
    }
}

/// Builder for creating panels with consistent styling
pub struct PanelBuilder {
    style: PanelStyle,
    width: Val,
    height: Val,
    padding: UiRect,
    margin: UiRect,
    flex_direction: FlexDirection,
    justify_content: JustifyContent,
    align_items: AlignItems,
    position_type: PositionType,
    display: Display,
    custom_background: Option<Color>,
    title: Option<String>,
}

impl PanelBuilder {
    pub fn new() -> Self {
        Self {
            style: PanelStyle::Default,
            width: Val::Auto,
            height: Val::Auto,
            padding: UiRect::all(Val::Px(dimensions::PANEL_PADDING)),
            margin: UiRect::all(Val::Px(0.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            position_type: PositionType::Relative,
            display: Display::Flex,
            custom_background: None,
            title: None,
        }
    }

    pub fn style(mut self, style: PanelStyle) -> Self {
        self.style = style;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }

    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }

    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.flex_direction = direction;
        self
    }

    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }

    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }

    pub fn position_type(mut self, position: PositionType) -> Self {
        self.position_type = position;
        self
    }

    pub fn display(mut self, display: Display) -> Self {
        self.display = display;
        self
    }

    /// Set a custom background color
    pub fn custom_background(mut self, color: Color) -> Self {
        self.custom_background = Some(color);
        self
    }

    /// Set a title for the panel
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let background_color = self
            .custom_background
            .unwrap_or_else(|| self.style.background_color());

        let mut panel_entity = parent.spawn((
            Node {
                width: self.width,
                height: self.height,
                padding: self.padding,
                margin: self.margin,
                border: UiRect::all(self.style.border_width()),
                flex_direction: self.flex_direction,
                justify_content: self.justify_content,
                align_items: self.align_items,
                position_type: self.position_type,
                display: self.display,
                ..default()
            },
            BackgroundColor(background_color),
            BorderColor(self.style.border_color()),
            Panel { style: self.style },
        ));

        // Add title if provided
        if let Some(title) = self.title {
            panel_entity.with_children(|parent| {
                super::label::LabelBuilder::new(title)
                    .style(super::label::LabelStyle::Title)
                    .margin(UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)))
                    .build(parent);
            });
        }

        panel_entity.id()
    }

    pub fn build_with_children(
        self,
        parent: &mut ChildSpawnerCommands,
        children: impl FnOnce(&mut ChildSpawnerCommands),
    ) -> Entity {
        let background_color = self
            .custom_background
            .unwrap_or_else(|| self.style.background_color());

        let title = self.title.clone(); // Clone title for use in closure

        parent
            .spawn((
                Node {
                    width: self.width,
                    height: self.height,
                    padding: self.padding,
                    margin: self.margin,
                    border: UiRect::all(self.style.border_width()),
                    flex_direction: self.flex_direction,
                    justify_content: self.justify_content,
                    align_items: self.align_items,
                    position_type: self.position_type,
                    display: self.display,
                    ..default()
                },
                BackgroundColor(background_color),
                BorderColor(self.style.border_color()),
                Panel { style: self.style },
            ))
            .with_children(|parent| {
                // Add title first if provided
                if let Some(title_text) = title {
                    super::label::LabelBuilder::new(title_text)
                        .style(super::label::LabelStyle::Title)
                        .margin(UiRect::bottom(Val::Px(dimensions::MARGIN_SMALL)))
                        .build(parent);
                }

                // Then add user-provided children
                children(parent);
            })
            .id()
    }
}
