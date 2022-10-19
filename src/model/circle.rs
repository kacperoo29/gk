use std::{f64::consts, str::FromStr};

use super::shape::{Shape, ShapeState, ShapeType, ORIGIN_X_KEY, ORIGIN_Y_KEY};

pub const RADIUS_KEY: &str = "Radius";

pub struct Circle {
    origin: Option<(f64, f64)>,
    radius: f64,
    state: ShapeState,
}

impl Circle {
    pub fn new() -> Self {
        Self {
            origin: None,
            radius: 0.0,
            state: ShapeState::New,
        }
    }
}

impl Shape for Circle {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        if !self.is_drawable() {
            return;
        }

        let (ox, oy) = self.origin.unwrap();
        ctx.begin_path();
        ctx.arc(ox, oy, self.radius, 0.0, 2.0 * consts::PI)
            .expect("Couldn't arc!");
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
        ShapeType::Circle
    }

    fn init_from_points(&mut self, origin: (f64, f64), end: (f64, f64)) {
        self.origin = Some(origin);
        self.radius = ((origin.0 - end.0).powf(2.0) + (origin.1 - end.1).powf(2.0)).sqrt();
    }

    fn set_end(&mut self, x: f64, y: f64) {
        let (ox, oy) = self.origin.unwrap();
        self.radius = ((ox - x).powf(2.0) + (oy - y).powf(2.0)).sqrt();
    }

    fn get_prop_str(&self) -> String {
        let mut string = String::new();
        string += &format!("Type: {:?}\n", self.get_type());
        if self.origin.is_some() {
            string += &format!("Origin: {:.0?}\n", self.origin.unwrap());
        }
        string += &format!("Radius: {:.2}\n", self.radius);

        return string;
    }

    fn get_state(&self) -> ShapeState {
        self.state
    }

    fn is_drawable(&self) -> bool {
        (self.state == ShapeState::Complete || self.state == ShapeState::Drawing)
            && self.radius != 0.0
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        if self.origin.is_none() {
            return false;
        }

        let (ox, oy) = self.origin.unwrap();
        let distance = ((ox - x).powf(2.0) + (oy - y).powf(2.0)).sqrt();
        distance <= self.radius
    }

    fn get_origin(&self) -> Option<(f64, f64)> {
        self.origin
    }

    fn get_end(&self) -> Option<(f64, f64)> {
        if self.origin.is_none() {
            return None;
        }

        let (ox, oy) = self.origin.unwrap();
        Some((ox + self.radius, oy))
    }

    fn get_props(&self) -> Vec<(String, String)> {
        let mut map = Vec::new();
        let (ox, oy) = self.origin.unwrap_or((0.0, 0.0));
        map.push((ORIGIN_X_KEY.to_string(), ox.to_string()));
        map.push((ORIGIN_Y_KEY.to_string(), oy.to_string()));
        map.push((RADIUS_KEY.to_string(), self.radius.to_string()));

        map
    }

    fn set_prop(&mut self, key: &str, value: &str) {
        match key {
            ORIGIN_X_KEY => {
                if let Some(origin) = self.origin {
                    self.origin = Some((value.parse().unwrap(), origin.1));
                } else {
                    self.origin = Some((value.parse().unwrap(), 0.0));
                }
            }
            ORIGIN_Y_KEY => {
                if let Some(origin) = self.origin {
                    self.origin = Some((origin.0, value.parse().unwrap()));
                } else {
                    self.origin = Some((0.0, value.parse().unwrap()));
                }
            }
            RADIUS_KEY => {
                self.radius = value.parse().unwrap();
            }
            _ => {}
        }
    }

    fn move_by(&mut self, x: f64, y: f64) {
        if let Some(origin) = self.origin {
            self.origin = Some((origin.0 + x, origin.1 + y));
        }
    }

    fn resize(&mut self, change: (f64, f64), origin: (f64, f64)) {
        if let Some((ox, oy)) = self.origin {
            let (cx, cy) = change;
            let (ox, oy) = (ox - origin.0, oy - origin.1);
            let (nx, ny) = (ox + cx, oy + cy);
            let new_radius = ((nx.powf(2.0) + ny.powf(2.0)).sqrt()).abs();
            self.radius = new_radius;
        }
    }

    fn set_state(&mut self, state: ShapeState) {
        self.state = state;
    }

    fn get_json(&self) -> String {
        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), serde_json::Value::String("circle".to_string()));
        map.insert("state".to_string(), serde_json::Value::String(self.state.to_string()));
        map.insert(
            "origin".to_string(),
            serde_json::Value::Array(vec![
                serde_json::Value::Number(serde_json::Number::from_f64(self.origin.unwrap().0).unwrap()),
                serde_json::Value::Number(serde_json::Number::from_f64(self.origin.unwrap().1).unwrap()),
            ]),
        );
        map.insert(
            "radius".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.radius).unwrap()),
        );

        serde_json::to_string(&map).unwrap()
    }

    fn from_json(&mut self, json: &str) {
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        if let Some(origin) = map.get("origin") {
            if let Some(origin) = origin.as_array() {
                let x = origin[0].as_f64().unwrap();
                let y = origin[1].as_f64().unwrap();
                self.origin = Some((x, y));
            }
        }

        if let Some(radius) = map.get("radius") {
            if let Some(radius) = radius.as_f64() {
                self.radius = radius;
            }
        }

        if let Some(state) = map.get("state") {
            if let Some(state) = state.as_str() {
                self.state = ShapeState::from_str(state).unwrap();
            }
        }
    }
}
