use core::fmt;

use wasm_bindgen::JsValue;

use super::{circle::Circle, line::Line, rectangle::Rectangle};

pub const ORIGIN_X_KEY: &str = "Origin x";
pub const ORIGIN_Y_KEY: &str = "Origin y";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeType {
    Line,
    Rectangle,
    Circle,
}

impl fmt::Display for ShapeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShapeType::Line => write!(f, "Line"),
            ShapeType::Rectangle => write!(f, "Rectangle"),
            ShapeType::Circle => write!(f, "Circle"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeState {
    New,
    Drawing,
    Complete,
}

pub trait Shape {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d);
    fn draw_highlighted(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_line_width(3.0);
        self.draw(ctx);
        ctx.set_line_width(1.0);
    }

    fn draw_selected(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_line_width(3.0);
        ctx.set_stroke_style(&JsValue::from_str("red"));
        self.draw(ctx);
        ctx.set_line_width(1.0);
        ctx.set_stroke_style(&JsValue::from_str("black"));
    }

    fn get_prop_str(&self) -> String;
    fn get_type(&self) -> ShapeType;
    fn get_state(&self) -> ShapeState;
    fn get_origin(&self) -> Option<(f64, f64)>;
    fn get_end(&self) -> Option<(f64, f64)>;
    fn get_props(&self) -> Vec<(String, String)>;
    fn is_drawable(&self) -> bool;
    fn contains(&self, x: f64, y: f64) -> bool;

    fn add_point(&mut self, x: f64, y: f64);
    fn set_end(&mut self, x: f64, y: f64);
    fn set_prop(&mut self, key: &str, value: &str);
    fn init_from_points(&mut self, origin: (f64, f64), end: (f64, f64));
    fn move_by(&mut self, x: f64, y: f64);
    fn resize(&mut self, change: (f64, f64), origin: (f64, f64));
    fn set_state(&mut self, state: ShapeState);
}

pub struct ShapeStorage {
    shapes: Vec<Box<dyn Shape>>,
    current_shape_idx: usize,
    highlighted_shape_idx: Option<usize>,
    selected_shape_idx: Option<usize>,
}

impl ShapeStorage {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            current_shape_idx: 0,
            highlighted_shape_idx: None,
            selected_shape_idx: None,
        }
    }

    pub fn get_or_create_shape(&mut self, shape_type: ShapeType) -> &mut dyn Shape {
        if self.shapes.is_empty() {
            self.shapes.push(ShapeStorage::create_helper(shape_type));

            return self.shapes[self.current_shape_idx].as_mut();
        }

        if self.shapes[self.current_shape_idx].get_type() != shape_type {
            return self.create_shape(shape_type);
        }

        match self.shapes[self.current_shape_idx].get_state() {
            ShapeState::Complete => self.create_shape(shape_type),
            _ => {
                self.selected_shape_idx = Some(self.current_shape_idx);
                return self.shapes[self.current_shape_idx].as_mut();
            }
        }
    }

    pub fn get_current_mut(&mut self) -> Option<&mut dyn Shape> {
        if self.shapes.is_empty() {
            return None;
        }

        return Some(self.shapes[self.current_shape_idx].as_mut());
    }

    pub fn get_shapes(&self) -> impl Iterator<Item = &Box<dyn Shape>> {
        return self.shapes.iter();
    }

    pub fn clear(&mut self) {
        self.current_shape_idx = 0;
        self.highlighted_shape_idx = None;
        self.selected_shape_idx = None;
        self.shapes.clear();
    }

    pub fn intersect_and_highlight(&mut self, x: f64, y: f64) -> Option<&dyn Shape> {
        for (i, shape) in self.shapes.iter().enumerate() {
            if shape.contains(x, y) {
                self.highlighted_shape_idx = Some(i);
                return Some(shape.as_ref());
            }
        }

        self.highlighted_shape_idx = None;
        return None;
    }

    pub fn intersect_and_select(&mut self, x: f64, y: f64) -> Option<&dyn Shape> {
        for (i, shape) in self.shapes.iter().enumerate() {
            if shape.contains(x, y) {
                self.selected_shape_idx = Some(i);
                return Some(shape.as_ref());
            }
        }

        self.selected_shape_idx = None;
        return None;
    }

    pub fn get_highlighted(&self) -> Option<&dyn Shape> {
        if let Some(idx) = self.highlighted_shape_idx {
            return Some(self.shapes[idx].as_ref());
        }

        return None;
    }

    pub fn get_selected(&self) -> Option<&dyn Shape> {
        if let Some(idx) = self.selected_shape_idx {
            return Some(self.shapes[idx].as_ref());
        }

        return None;
    }

    pub fn get_selected_mut(&mut self) -> Option<&mut dyn Shape> {
        if let Some(idx) = self.selected_shape_idx {
            return Some(self.shapes[idx].as_mut());
        }

        return None;
    }

    pub fn new_shape(&mut self, shape_type: ShapeType) {
        self.shapes.push(ShapeStorage::create_helper(shape_type));
        self.current_shape_idx = self.shapes.len() - 1;
        self.selected_shape_idx = Some(self.current_shape_idx);
        self.highlighted_shape_idx = None;
    }

    pub fn submit_shape(&mut self) {
        if let Some(idx) = self.selected_shape_idx {
            let shape = self.shapes[idx].as_mut();
            if shape.get_end().is_some() && shape.get_origin().is_some() {
                shape.set_state(ShapeState::Complete);            
            }
        }
    }

    fn create_shape(&mut self, shape_type: ShapeType) -> &mut dyn Shape {
        self.current_shape_idx += 1;
        self.shapes.push(ShapeStorage::create_helper(shape_type));

        return self.shapes[self.current_shape_idx].as_mut();
    }

    fn create_helper(shape_type: ShapeType) -> Box<dyn Shape> {
        match shape_type {
            ShapeType::Line => Box::new(Line::new()),
            ShapeType::Rectangle => Box::new(Rectangle::new()),
            ShapeType::Circle => Box::new(Circle::new()),
        }
    }
}
