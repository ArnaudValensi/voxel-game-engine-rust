mod mesh;
pub use self::mesh::{UIMesh, UIMeshPipe};
use super::Renderer;
use yoga::{Layout, Node};

#[derive(Debug)]
pub struct Rect {
    pub position: (f32, f32),
    pub size: (f32, f32),
}

impl Rect {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Self {
        Self { position, size }
    }
}

pub struct Element {
    node: Node,
    background_color: [f32; 3],
    children: Vec<Element>,
}

impl Element {
    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    pub fn calculate_layout(&mut self) {
        self.node
            .calculate_layout(512.0, 512.0, yoga::Direction::LTR);
    }

    pub fn get_layout(&self) -> Layout {
        self.node.get_layout()
    }
}

pub struct ElementBuilder {
    style: std::vec::Vec<yoga::FlexStyle>,
    background_color: Option<[f32; 3]>,
    children: Vec<ElementBuilder>,
}

#[allow(clippy::new_without_default_derive)]
impl ElementBuilder {
    pub fn new() -> Self {
        Self {
            style: Vec::new(),
            background_color: None,
            children: Vec::with_capacity(0),
        }
    }

    pub fn background_color(mut self, background_color: [f32; 3]) -> Self {
        self.background_color = Some(background_color);
        self
    }

    pub fn style(mut self, style: &mut std::vec::Vec<yoga::FlexStyle>) -> Self {
        self.style.append(style);
        self
    }

    pub fn child(mut self, element_builder: ElementBuilder) -> Self {
        self.children.push(element_builder);
        self
    }

    pub fn build(self) -> Element {
        let mut node = Node::new();

        node.apply_styles(&self.style);

        let background_color = match self.background_color {
            Some(background_color) => background_color,
            None => [1.0, 1.0, 1.0],
        };

        let mut i = 0;
        let children = self.children.into_iter().map(|child| {
            let mut child_element = child.build();
            let child_element_node = child_element.get_node_mut();
            node.insert_child(child_element_node, i as u32);
            i += 1;

            child_element
        }).collect();

        Element {
            node,
            background_color,
            children,
        }
    }
}

pub struct Gui {}

impl Gui {
    pub fn render(mut renderer: &mut Renderer, pipe: &UIMeshPipe, mut element: Element) {
        println!("=========");
        element.calculate_layout();
        Gui::render_children(&mut renderer, pipe, element);
    }

    pub fn render_children(mut renderer: &mut Renderer, pipe: &UIMeshPipe, element: Element) {
        println!(
            "Layout is {:#?}, background_color: {:?}",
            element.get_layout(),
            element.background_color
        );

        let layout = element.get_layout();
        let rect = Rect {
            position: (layout.left(), layout.top()),
            size: (layout.width(), layout.height()),
        };

        let mut mesh = UIMesh::new(&mut renderer, &rect, element.background_color);
        renderer.draw(&mut mesh, pipe);

        for child in element.children {
            Gui::render_children(&mut renderer, pipe, child);
        }
    }

    pub fn create_element() -> ElementBuilder {
        ElementBuilder::new()
    }
}
