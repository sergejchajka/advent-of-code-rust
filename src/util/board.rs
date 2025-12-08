use crate::util::point_3d::Point3D;

#[derive(Debug)]
pub struct Board {
    pub circuits: Vec<Vec<Point3D>>,
}

impl Board {

    pub(crate) fn new(_boxes: Vec<Point3D>) -> Self {
        Self {
            circuits: Vec::new(),
        }
    }

    pub fn join(&mut self, point1: &Point3D, point2: &Point3D) {
        let p1_circuit = self.circuits.iter().position(|c| c.contains(&point1));
        let p2_circuit = self.circuits.iter().position(|c| c.contains(&point2));
        match (p1_circuit, p2_circuit) {
            (Some(circuit1), Some(circuit2)) => {
                if circuit1 == circuit2 { return; }

                let mut second = self.circuits.remove(circuit1.max(circuit2));
                self.circuits[circuit1.min(circuit2)].append(&mut second);
            },
            (Some(circuit), None)  => self.circuits[circuit].push(point2.clone()),
            (None, Some(circuit))  => self.circuits[circuit].push(point1.clone()),
            (None, None) => self.circuits.push(vec![point1.clone(), point2.clone()]),
        }
    }
}
