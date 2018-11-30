mod mesh;
pub use self::mesh::{UIMesh, UIMeshPipe};
use super::Renderer;

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
    rect: Rect,
    color: [f32; 3],
    children: Vec<Element>,
}

pub struct ElementBuilder {
    rect: Rect,
    color: Option<[f32; 3]>,
    children: Vec<Element>,
}

impl ElementBuilder {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            color: None,
            children: Vec::with_capacity(0),
        }
    }

    pub fn color(mut self, color: [f32; 3]) -> Self {
        self.color = Some(color);
        self
    }

    pub fn child(mut self, element: Element) -> Self {
        self.children.push(element);
        self
    }

    pub fn build(self) -> Element {
        let color = match self.color {
            Some(color) => color,
            None => [1.0, 1.0, 1.0],
        };

        Element {
            rect: self.rect,
            color,
            children: self.children,
        }
    }
}

pub struct Gui {}

impl Gui {
    pub fn render(mut renderer: &mut Renderer, pipe: &UIMeshPipe, element: Element) {
        let mut mesh = UIMesh::new(&mut renderer, &element.rect, element.color);
        renderer.draw(&mut mesh, pipe);

        for child in element.children {
            Gui::render(&mut renderer, pipe, child);
        }
    }

    pub fn create_element(position: (f32, f32), size: (f32, f32)) -> ElementBuilder {
        let rect = Rect::new(position, size);

        ElementBuilder::new(rect)
    }
}
