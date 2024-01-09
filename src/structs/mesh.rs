use tobj::Mesh;

use super::{shape::Shape, appearance::Appearance, vec3::Vec3, ray::Ray};

type Triangle = [Vec3; 3];

pub struct ColoredMesh {
    pub mesh: Mesh,
    pub location: Vec3,
    pub appearance: Appearance
}

impl Shape for ColoredMesh {
    fn intersections(&self, ray: &Ray) -> Vec<f64> {
        let mut res: Vec<f64> = Vec::new();

        for tri in self.triangles() {
            if let Some((_intersection_point, scalar)) = ColoredMesh::intersect_triangle(ray, &tri) {
                res.push(scalar);
            }
        }

        res
    }

    fn normal_at(&self, _point: &Vec3, ray: &Ray) -> Vec3 {
        // Get the normal of the (closest) triangle
        let triangle: Triangle = 
            self.triangles()
                .into_iter()
                .filter(|tri| ColoredMesh::intersect_triangle(ray, tri).is_some())    // Only grab tris that intersect the ray
                .reduce(|best, cur| {                                       // Find the tri with the vertex closest to the ray's origin
                    // If any of the current tri's vertexes are closer than all of the known closest tri's vertexes...
                    if cur.iter().any(|cur_vert| best.iter().all(|best_vert| Vec3::between(&ray.origin, cur_vert).length() < Vec3::between(&ray.origin, best_vert).length())) {
                        cur
                    } else {
                        best
                    }
                }).expect("Trying to get a normal on the mesh, but the ray doesn't intersect?");

        Vec3::between(&triangle[0], &triangle[1]).cross(&Vec3::between(&triangle[1], &triangle[2])).unit()
    }

    fn appearance(&self) -> Appearance {
        self.appearance.clone()
    }
}

impl ColoredMesh {
    pub fn new(mesh_url: &str, location: Vec3, appearance: Appearance) -> Self {
        let (models, _materials) = tobj::load_obj(mesh_url, &tobj::GPU_LOAD_OPTIONS).expect(&format!("Unable to load object file {}", mesh_url));
        let mesh = models.into_iter().nth(0).expect(&format!("No models defined in {}", mesh_url)).mesh;

        Self {
            mesh,
            location,
            appearance
        }
    }

    pub fn intersect_triangle(ray: &Ray, tri: &Triangle) -> Option<(Vec3, f64)>
    {
        let [a,b,c] = tri;

        let e1 = *b - *a;
        let e2 = *c - *a;
        let n = e1.cross(&e2);

        let det = -1.0 * ray.direction.dot(&n);
        let invdet = 1.0 / det;

        let ao = ray.origin - *a;
        let dao = ao.cross(&ray.direction);

        let u = e2.dot(&dao) * invdet;
        let v = -1.0 * e1.dot(&dao) * invdet;
        let t = ao.dot(&n) * invdet;

        if det >= 1e-6 && t >= 0.0 && u >= 0.0 && v >= 0.0 && (u+v) <= 1.0 {
            Some((ray.origin + ray.direction * t, t))
        } else {
            None
        }
    }

    pub fn triangles(&self) -> Vec<Triangle> {
        let mut res: Vec<Triangle> = Vec::new();

        // mesh.indices is a *flattened* vector of indices into mesh.positions
        for idx in self.mesh.indices.as_slice().chunks(3) {
            let [vert_index_a, vert_index_b, vert_index_c] = [idx[0], idx[1], idx[2]];
            res.push([
                self.mesh.positions[(vert_index_a as usize) * 3..(vert_index_a as usize + 1)*3].into(),
                self.mesh.positions[(vert_index_b as usize) * 3..(vert_index_b as usize + 1)*3].into(),
                self.mesh.positions[(vert_index_c as usize) * 3..(vert_index_c as usize + 1)*3].into()
            ]);
        }

        res
    }
}