use std::str::FromStr;

use super::shape::{Shape, ShapeState, ShapeType, ORIGIN_X_KEY, ORIGIN_Y_KEY};

pub const END_X_KEY: &str = "End x";
pub const END_Y_KEY: &str = "End y";

pub struct Line {
    origin: Option<(f64, f64)>,
    end: Option<(f64, f64)>,
    state: ShapeState,
}

impl Line {
    pub fn new() -> Self {
        Self {
            origin: None,
            end: None,
            state: ShapeState::New,
        }
    }
}

impl Shape for Line {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        if !self.is_drawable() {
            return;
        }

        let (ox, oy) = self.origin.unwrap();
        let (ex, ey) = self.end.unwrap();
        ctx.begin_path();
        ctx.line_to(ox, oy);
        ctx.line_to(ex, ey);
        ctx.stroke();
    }

    fn add_point(&mut self, x: f64, y: f64) {
        match self.origin {
            Some(_) => {
                self.end = Some((x, y));

                self.state = ShapeState::Complete;
            }
            None => {
                self.origin = Some((x, y));

                self.state = ShapeState::Drawing;
            }
        }
    }

    fn get_type(&self) -> ShapeType {
        return ShapeType::Line;
    }

    fn init_from_points(&mut self, origin: (f64, f64), end: (f64, f64)) {
        self.origin = Some(origin);
        self.end = Some(end);
    }

    fn set_end(&mut self, x: f64, y: f64) {
        self.end = Some((x, y));
    }

    fn get_prop_str(&self) -> String {
        let mut string = String::new();
        string += &format!("Type: {:?}\n", self.get_type());
        if self.origin.is_some() {
            string += format!("Origin: {:.0?}\n", self.origin.unwrap()).as_str();
        }
        if self.end.is_some() {
            string += format!("End: {:.0?}\n", self.end.unwrap()).as_str();
        }

        return string;
    }

    fn get_state(&self) -> super::shape::ShapeState {
        self.state
    }

    fn is_drawable(&self) -> bool {
        (self.state == ShapeState::Complete || self.state == ShapeState::Drawing)
            && self.end.is_some()
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        if !self.is_drawable() {
            return false;
        }

        let (ox, oy) = self.origin.unwrap();
        let (ex, ey) = self.end.unwrap();

        let dx = ex - ox;
        let dy = ey - oy;

        let dist = (dx * dx + dy * dy).sqrt();

        let t = ((x - ox) * dx + (y - oy) * dy) / (dist * dist);

        if t < 0.0 || t > 1.0 {
            return false;
        }

        let px = ox + t * dx;
        let py = oy + t * dy;

        let d = ((x - px) * (x - px) + (y - py) * (y - py)).sqrt();

        return d <= 5.0;
    }

    fn get_origin(&self) -> Option<(f64, f64)> {
        self.origin
    }

    fn get_end(&self) -> Option<(f64, f64)> {
        self.end
    }

    fn get_props(&self) -> Vec<(String, String)> {
        let mut map = Vec::new();
        let (ox, oy) = self.origin.unwrap_or((0.0, 0.0));
        let (ex, ey) = self.end.unwrap_or((0.0, 0.0));
        map.push((ORIGIN_X_KEY.to_string(), ox.to_string()));
        map.push((ORIGIN_Y_KEY.to_string(), oy.to_string()));
        map.push((END_X_KEY.to_string(), ex.to_string()));
        map.push((END_Y_KEY.to_string(), ey.to_string()));

        return map;
    }

    fn set_prop(&mut self, key: &str, value: &str) {
        if key == ORIGIN_X_KEY {
            if let Some((_, y)) = self.origin {
                self.origin = Some((value.parse().unwrap(), y));
            } else {
                self.origin = Some((value.parse().unwrap(), 0.0));
            }
        } else if key == ORIGIN_Y_KEY {
            if let Some((x, _)) = self.origin {
                self.origin = Some((x, value.parse().unwrap()));
            } else {
                self.origin = Some((0.0, value.parse().unwrap()));
            }
        } else if key == END_X_KEY {
            if let Some((_, y)) = self.end {
                self.end = Some((value.parse().unwrap(), y));
            } else {
                self.end = Some((value.parse().unwrap(), 0.0));
            }
        } else if key == END_Y_KEY {
            if let Some((x, _)) = self.end {
                self.end = Some((x, value.parse().unwrap()));
            } else {
                self.end = Some((0.0, value.parse().unwrap()));
            }
        }
    }

    fn move_by(&mut self, x: f64, y: f64) {
        if let Some((ox, oy)) = self.origin {
            self.origin = Some((ox + x, oy + y));
        }
        if let Some((ex, ey)) = self.end {
            self.end = Some((ex + x, ey + y));
        }
    }

    fn resize(&mut self, change: (f64, f64), origin: (f64, f64)) {
        let epsilon = 5.0;
        if let Some((ox, oy)) = self.origin {
            if (ox - origin.0).abs() < epsilon && (oy - origin.1).abs() < epsilon {
                self.origin = Some((ox + change.0, oy + change.1));
            }
        }
        if let Some((ex, ey)) = self.end {
            if (ex - origin.0).abs() < epsilon && (ey - origin.1).abs() < epsilon {
                self.end = Some((ex + change.0, ey + change.1));
            }
        }
    }

    fn set_state(&mut self, state: ShapeState) {
        self.state = state;
    }

    fn get_json(&self) -> String {
        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), serde_json::Value::String("line".to_string()));
        map.insert("state".to_string(), serde_json::Value::String(self.state.to_string()));
        if let Some((ox, oy)) = self.origin {
            map.insert("origin_x".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(ox).unwrap()));
            map.insert("origin_y".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(oy).unwrap()));
        }
        if let Some((ex, ey)) = self.end {
            map.insert("end_x".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(ex).unwrap()));
            map.insert("end_y".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(ey).unwrap()));
        }

        return serde_json::to_string(&map).unwrap();
    }

    fn from_json(&mut self, json: &str) {
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        if let Some(serde_json::Value::Number(ox)) = map.get("origin_x") {
            if let Some(serde_json::Value::Number(oy)) = map.get("origin_y") {
                self.origin = Some((ox.as_f64().unwrap(), oy.as_f64().unwrap()));
            }
        }
        if let Some(serde_json::Value::Number(ex)) = map.get("end_x") {
            if let Some(serde_json::Value::Number(ey)) = map.get("end_y") {
                self.end = Some((ex.as_f64().unwrap(), ey.as_f64().unwrap()));
            }
        }
        if let Some(serde_json::Value::String(state)) = map.get("state") {
            self.state = ShapeState::from_str(state).unwrap();
        }
    }
}
