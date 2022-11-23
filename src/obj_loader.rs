use crate::{Triangle, MAX_OBJECTS};
#[allow(unused)]
pub fn load_obj(path: &std::path::Path, color: [f32; 4]) -> [Triangle; MAX_OBJECTS] {
    let triangle: Triangle = Default::default();
    let mut triangles = [triangle; MAX_OBJECTS];
    let models = tobj::load_obj(path, &tobj::LoadOptions::default()).unwrap();
    for model in models.0.iter() {
        for i_ in 0..model.mesh.indices.len() / 3 {
            let i = 3 * i_;
            let positions = &model.mesh.positions;
            let indices = &model.mesh.indices;
            let triangle = Triangle {
                color,
                vertex0: [
                    positions[indices[i] as usize * 3],
                    positions[indices[i] as usize * 3 + 1],
                    positions[indices[i] as usize * 3 + 2],
                ],
                vertex1: [
                    positions[indices[i + 1] as usize * 3],
                    positions[indices[i + 1] as usize * 3 + 1],
                    positions[indices[i + 1] as usize * 3 + 2],
                ],
                vertex2: [
                    positions[indices[i + 2] as usize * 3],
                    positions[indices[i + 2] as usize * 3 + 1],
                    positions[indices[i + 2] as usize * 3 + 2],
                ],
            };
            triangles[i_] = triangle;
        }
    }
    return triangles;
}
