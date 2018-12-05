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

    pub fn is_position_inside(&self, position: (f32, f32)) -> bool {
        position.0 > self.position.0
            && position.0 < self.position.0 + self.size.0
            && position.1 > self.position.1
            && position.1 < self.position.1 + self.size.1
    }
}

pub struct Element<'a> {
    node: Node,
    background_color: [f32; 3],
    children: Vec<Element<'a>>,
    on_mouse_enter_fn: Option<&'a Fn()>,
}

impl<'a> Element<'a> {
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

    pub fn dispatch_on_mouse_enter(&self) {
        if let Some(on_mouse_enter_fn) = self.on_mouse_enter_fn {
            (*on_mouse_enter_fn)();
        }
    }
}

pub struct ElementBuilder<'a> {
    style: std::vec::Vec<yoga::FlexStyle>,
    background_color: Option<State<[f32; 3]>>,
    children: Vec<ElementBuilder<'a>>,
    on_mouse_enter_fn: Option<&'a Fn()>,
}

#[allow(clippy::new_without_default_derive)]
impl<'a> ElementBuilder<'a> {
    pub fn new() -> Self {
        Self {
            style: Vec::new(),
            background_color: None,
            children: Vec::with_capacity(0),
            on_mouse_enter_fn: None,
        }
    }

    pub fn background_color(mut self, background_color: Arg<[f32; 3]>) -> Self {
        match background_color {
            Arg::Value(value) => self.background_color = Some(State::new(value)),
            Arg::State(state) => self.background_color = Some(state)
        }
        self
    }

    pub fn style(mut self, style: &mut std::vec::Vec<yoga::FlexStyle>) -> Self {
        self.style.append(style);
        self
    }

    pub fn child(mut self, element_builder: ElementBuilder<'a>) -> Self {
        self.children.push(element_builder);
        self
    }

    pub fn on_mouse_enter(mut self, callback: &'a Fn()) -> Self {
        self.on_mouse_enter_fn = Some(callback);
        self
    }

    pub fn build(self) -> Element<'a> {
        let mut node = Node::new();

        node.apply_styles(&self.style);

        let background_color = match self.background_color {
            Some(background_color) => background_color.value,
            None => [1.0, 1.0, 1.0],
        };

        let mut i = 0;
        let children = self
            .children
            .into_iter()
            .map(|child| {
                let mut child_element = child.build();
                let child_element_node = child_element.get_node_mut();
                node.insert_child(child_element_node, i as u32);
                i += 1;

                child_element
            })
            .collect();

        Element {
            node,
            background_color,
            children,
            on_mouse_enter_fn: self.on_mouse_enter_fn,
        }
    }
}

pub enum Arg<T> {
    Value(T),
    State(State<T>)
}

pub struct State<T> {
    value: T,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
        }
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }
}

pub struct Gui {
    // states: Vec<State<>>
    mouse_position: (f32, f32),
}

#[allow(clippy::new_without_default_derive)]
impl Gui {
    pub fn new() -> Self {
        Self {
            mouse_position: (0.0, 0.0),
        }
    }

    pub fn render(&self, mut renderer: &mut Renderer, pipe: &UIMeshPipe, mut element: Element) {
        element.calculate_layout();
        self.render_children(&mut renderer, pipe, element);
    }

    pub fn render_children(&self, mut renderer: &mut Renderer, pipe: &UIMeshPipe, element: Element) {
        let layout = element.get_layout();
        let rect = Rect {
            position: (layout.left(), layout.top()),
            size: (layout.width(), layout.height()),
        };

        if rect.is_position_inside(self.mouse_position) {
            element.dispatch_on_mouse_enter();
        }

        let mut mesh = UIMesh::new(&mut renderer, &rect, element.background_color);
        renderer.draw(&mut mesh, pipe);

        for child in element.children {
            self.render_children(&mut renderer, pipe, child);
        }
    }

    pub fn create_element<'a>() -> ElementBuilder<'a> {
        ElementBuilder::new()
    }

    pub fn use_state<T>(&mut self, value: T) -> State<T> {
        State::new(value)
    }

    // pub fn set_state<T>(&mut self, value: T) {

    // }

    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = (x, y);
    }
}
