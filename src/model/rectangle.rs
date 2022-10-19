use std::str::FromStr;

use super::shape::{Shape, ShapeState, ShapeType, ORIGIN_X_KEY, ORIGIN_Y_KEY};

pub const WIDTH_KEY: &str = "Width";
pub const HEIGHT_KEY: &str = "Height";

pub struct Rectangle {
    origin: Option<(f64, f64)>,
    width: f64,
    height: f64,
    state: ShapeState,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            origin: None,
            width: 0.0,
            height: 0.0,
            state: ShapeState::New,
        }
    }
}

impl Shape for Rectangle {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        if !self.is_drawable() {
            return;
        }

        let (ox, oy) = self.origin.unwrap();
        ctx.begin_path();
        ctx.rect(ox, oy, self.width, self.height);
        ctx.stroke();
    }

    fn add_point(&mut self, x: f64, y: f64) {
        match self.origin {
            Some(_) => {
                self.set_end(x, y);
                self.state = ShapeState::Complete;
            }
            None => {
                self.origin = Some((x, y));
                self.state = ShapeState::Drawing;
            }
        }
    }

    fn get_type(&self) -> super::shape::ShapeType {
        return ShapeType::Rectangle;
    }

    fn init_from_points(&mut self, origin: (f64, f64), end: (f64, f64)) {
        self.origin = Some(origin);
        self.set_end(end.0, end.1);
    }

    fn set_end(&mut self, x: f64, y: f64) {
        let (ox, oy) = self.origin.unwrap();
        self.width = x - ox;
        self.height = y - oy;
    }

    fn get_state(&self) -> super::shape::ShapeState {
        self.state
    }

    fn get_prop_str(&self) -> String {
        let mut string = String::new();
        string += &format!("Type: {:?}\n", self.get_type());
        if self.origin.is_some() {
            string += format!("Origin: {:.0?}\n", self.origin.unwrap()).as_str();
        }
        string += format!("Width: {}\n", self.width).as_str();
        string += format!("Height: {}\n", self.height).as_str();

        return string;
    }

    fn is_drawable(&self) -> bool {
        (self.state == ShapeState::Complete || self.state == ShapeState::Drawing)
            && self.width != 0.0
            && self.height != 0.0
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        if !self.is_drawable() {
            return false;
        }

        let (ox, oy) = self.origin.unwrap();
        let (ex, ey) = (ox + self.width, oy + self.height);

        let (x1, x2) = if ex > ox { (ox, ex) } else { (ex, ox) };

        let (y1, y2) = if ey > oy { (oy, ey) } else { (ey, oy) };

        x >= x1 && x <= x2 && y >= y1 && y <= y2
    }

    fn get_origin(&self) -> Option<(f64, f64)> {
        self.origin
    }

    fn get_end(&self) -> Option<(f64, f64)> {
        if self.origin.is_none() {
            return None;
        }

        let (ox, oy) = self.origin.unwrap();
        Some((ox + self.width, oy + self.height))
    }

    fn get_props(&self) -> Vec<(String, String)> {
        let mut map = Vec::new();
        let (ox, oy) = self.origin.unwrap_or((0.0, 0.0));
        map.push((ORIGIN_X_KEY.to_string(), ox.to_string()));
        map.push((ORIGIN_Y_KEY.to_string(), oy.to_string()));

        map.push((WIDTH_KEY.to_string(), self.width.to_string()));
        map.push((HEIGHT_KEY.to_string(), self.height.to_string()));

        return map;
    }

    fn set_prop(&mut self, key: &str, value: &str) {
        match key {
            ORIGIN_X_KEY => {
                if let Some((_, oy)) = self.origin {
                    self.origin = Some((value.parse().unwrap(), oy));
                } else {
                    self.origin = Some((value.parse().unwrap(), 0.0));
                }
            }
            ORIGIN_Y_KEY => {
                if let Some((ox, _)) = self.origin {
                    self.origin = Some((ox, value.parse().unwrap()));
                } else {
                    self.origin = Some((0.0, value.parse().unwrap()));
                }
            }
            WIDTH_KEY => {
                self.width = value.parse().unwrap();
            }
            HEIGHT_KEY => {
                self.height = value.parse().unwrap();
            }
            _ => {}
        }
    }

    fn move_by(&mut self, x: f64, y: f64) {
        if let Some((ox, oy)) = self.origin {
            self.origin = Some((ox + x, oy + y));
        }
    }

    fn resize(&mut self, change: (f64, f64), origin: (f64, f64)) {
        if self.origin.is_none() {
            return;
        }

        let (ox, oy) = self.origin.unwrap();
        let (ex, ey) = (ox + self.width, oy + self.height);

        let (x1, x2) = if ex > ox { (ox, ex) } else { (ex, ox) };

        let (y1, y2) = if ey > oy { (oy, ey) } else { (ey, oy) };

        let epsilon = 5.0;
        let (x, y) = origin;
        // detect which corner is being dragged
        if (x - x1).abs() < epsilon && (y - y1).abs() < epsilon {
            // top left
            self.origin = Some((ox + change.0, oy + change.1));
            self.width -= change.0;
            self.height -= change.1;
            return;
        } else if (x - x2).abs() < epsilon && (y - y1).abs() < epsilon {
            // top right
            self.origin = Some((ox, oy + change.1));
            self.width += change.0;
            self.height -= change.1;
            return;
        } else if (x - x1).abs() < epsilon && (y - y2).abs() < epsilon {
            // bottom left
            self.origin = Some((ox + change.0, oy));
            self.width -= change.0;
            self.height += change.1;
            return;
        } else if (x - x2).abs() < epsilon && (y - y2).abs() < epsilon {
            // bottom right
            self.width += change.0;
            self.height += change.1;
            return;
        }

        // detect which edge is being dragged
        if (x - x1).abs() < epsilon {
            // left
            self.origin = Some((ox + change.0, oy));
            self.width -= change.0;
            return;
        } else if (x - x2).abs() < epsilon {
            // right
            self.width += change.0;
            return;
        } else if (y - y1).abs() < epsilon {
            // top
            self.origin = Some((ox, oy + change.1));
            self.height -= change.1;
            return;
        } else if (y - y2).abs() < epsilon {
            // bottom
            self.height += change.1;
            return;
        }
    }

    fn set_state(&mut self, state: ShapeState) {
        self.state = state;
    }

    fn get_json(&self) -> String {
        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), "rectangle".to_string().into());
        map.insert("state".to_string(), self.state.to_string().into());
        if let Some((ox, oy)) = self.origin {
            map.insert("origin_x".to_string(), ox.into());
            map.insert("origin_y".to_string(), oy.into());
        }
        map.insert("width".to_string(), self.width.into());
        map.insert("height".to_string(), self.height.into());

        return serde_json::to_string(&map).unwrap();
    }

    fn from_json(&mut self, json: &str) {
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        if let Some(serde_json::Value::String(state)) = map.get("state") {
            self.state = ShapeState::from_str(state).unwrap();
        }
        if let Some(serde_json::Value::Number(ox)) = map.get("origin_x") {
            if let Some(serde_json::Value::Number(oy)) = map.get("origin_y") {
                self.origin = Some((ox.as_f64().unwrap(), oy.as_f64().unwrap()));
            }
        }
        if let Some(serde_json::Value::Number(width)) = map.get("width") {
            self.width = width.as_f64().unwrap();
        }
        if let Some(serde_json::Value::Number(height)) = map.get("height") {
            self.height = height.as_f64().unwrap();
        }
    }
}
