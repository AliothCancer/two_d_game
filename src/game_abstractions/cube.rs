#![allow(unused)]

pub struct Cube {
    cube_type: CubeType,
    lenght: f32,
}

pub struct CubeBuilder {
    cube_type: Option<CubeType>,
    lenght: Option<f32>,
}

impl CubeBuilder {
    pub fn build(self) -> Cube {
        Cube {
            cube_type: self.cube_type.unwrap(),
            lenght: self.lenght.unwrap(),
        }
    }
    pub fn new() -> Self {
        Self {
            cube_type: None,
            lenght: None,
        }
    }
    pub fn cube_type(mut self, cube_type: CubeType) -> Self {
        self.cube_type = Some(cube_type);
        self
    }
    pub fn lenght(mut self, lenght: f32) -> Self {
        self.lenght = Some(lenght);
        self
    }
}

pub enum CubeType {
    Neutron,
    Positron,
    Negatron,
}

impl Default for Cube {
    fn default() -> Self {
        CubeBuilder::new()
            .cube_type(CubeType::Neutron)
            .lenght(25.0)
            .build()
    }
}
