use crate::resources::Resources;

type Position = (f32, f32, f32);
type UV = (f32, f32);
type Normal = (f32, f32, f32);
//type Face = (u32, u32, u32);

#[derive(Debug)]
pub struct Face {
    p: u32,
    u: u32,
    n: u32,
}

#[derive(Debug)]
pub struct MeshObj {
    positions: Vec<Position>,
    uvs: Vec<UV>,
    normals: Vec<Normal>,
    smoothing: bool,
    group: String,
    mat_file: String,
    mat_ids: Vec<String>,
    faces: Vec<Vec<Face>>,
}

#[derive(Debug)]
pub struct MeshMat {

}

#[derive(Debug)]
pub struct MeshVertex {
    pos: (f32, f32, f32),
    uv: (f32, f32),
    norms: (f32, f32, f32),
}

#[derive(Debug)]
pub struct MeshTriangle(Vec<MeshVertex>);

#[derive(Debug)]
pub struct MeshVertices(Vec<MeshTriangle>);

#[derive(Debug)]
pub struct Mesh {
    obj: MeshObj,
    //mat: MeshMat,
}

impl Mesh {
    pub fn from_source(source: &str) -> Mesh {
        Mesh {
            obj: parse_obj_source(&source),
            //mat,
        }
    }

    pub fn from_file(res: &Resources, path: &str) -> Result<Mesh, failure::Error> {
        let source = res.load_obj(path)?;

        Ok(Mesh {
            obj: parse_obj_source(&source),
            //mat,
        })
    }

    // TODO: either need to enfoce only 3 vertices per face or triangulate faces to triangles
    pub fn get_vertices(&self) -> MeshVertices {
        let mut verts = Vec::new();

        // TODO: this needs to be reworked
        for f in &self.obj.faces {
            let mut mesh_tri = Vec::new();
            for t in f {
                let Face { p: pos, u: uv, n: norm } = t;
                let pp = *pos as i32 - 1;
                let uu = *uv as i32 - 1;
                let nn = *norm as i32 - 1;
                let p = pp as usize;
                let n = nn as usize;


                let mesh_uv = if uu < 0 {
                    (0.0, 0.0)
                } else {
                    (self.obj.uvs[uu as usize].0, self.obj.uvs[uu as usize].1)
                };

                mesh_tri.push(MeshVertex {
                    pos: (self.obj.positions[p].0, self.obj.positions[p].1, self.obj.positions[p].2),
                    uv: mesh_uv,//(self.obj.uvs[u].0, self.obj.uvs[u].1),
                    norms: (self.obj.normals[n].0, self.obj.normals[n].1, self.obj.normals[n].2),
                });
            }
            verts.push(MeshTriangle(mesh_tri));
        }

        MeshVertices(verts)
    }
}

fn sanitize_line(key: &str, line: &str) -> String {
    line.replace(key, "")
}

fn collect_on<T>(sep: &str) -> impl Fn(&str) -> Vec<T> + '_
where
    T: std::str::FromStr + std::default::Default,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    move |line| {
        let v: Vec<T> = line.split(sep).map(|s| s.parse::<T>().unwrap_or_default()).collect();
        v
    }
}

fn collect_on_multispace<T>(line: &str) -> Vec<T>
where
    T: std::str::FromStr + std::default::Default,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let v: Vec<T> = line.split_whitespace().map(|s| s.parse::<T>().unwrap_or_default()).collect();
    v
}

fn parse_obj_source(source: &str) -> MeshObj {
    let mut positions: Vec<Position> = Vec::new();
    let mut uvs: Vec<UV> = Vec::new();
    let mut normals: Vec<Normal> = Vec::new();
    let mut smoothing: bool = false;
    let mut group: String = String::new();
    let mut mat_file: String = String::new();
    let mut mat_ids: Vec<String> = Vec::new();
    let mut faces: Vec<Vec<Face>> = Vec::new();

    let collect_on_space = collect_on(" ");
    let collect_str_on_space = collect_on::<String>(" ");
    let collect_on_slash = collect_on::<u32>("/");

    for line in source.lines() {
        match line {
            l if line.starts_with("v ") => {
                let v = collect_on_space(&sanitize_line("v ", line));
                positions.push((v[0], v[1], v[2]));
            },
            l if line.starts_with("vt ") => {
                let v = collect_on_space(&sanitize_line("vt ", line));
                uvs.push((v[0], v[1]));
            },
            l if line.starts_with("vn ") => {
                let v = collect_on_space(&sanitize_line("vn ", line));
                normals.push((v[0], v[1], v[2]));
            },
            l if line.starts_with("s ") => {
                let v = sanitize_line("s ", line);
                if v.trim() == "off" {
                    smoothing = false;
                } else {
                    smoothing = true;
                }
            }
            l if line.starts_with("g ") => {
                let v = sanitize_line("g ", line);
                group = v;
            },
            l if line.starts_with("mtllib ") => {
                let v = sanitize_line("mtllib ", line);
                mat_file = v;
            }
            l if line.starts_with("usemtl ") => {
                let v = sanitize_line("usemtl ", line);
                mat_ids.push(v);
            },
            l if line.starts_with("f ") => {
                //let v = collect_str_on_space(&sanitize_line("f ", line));
                let v = collect_on_multispace::<String>(&sanitize_line("f ", line));
                let mut f = Vec::new();
                for s in v {
                    let t: Vec<u32> = collect_on_slash(&s);
                    f.push(Face {
                        p: t[0],
                        u: t[1],
                        n: t[2],
                    });
                }
                faces.push(f);
            },
            _ => {},
        }
    }

    MeshObj {
        positions,
        uvs,
        normals,
        smoothing,
        group,
        mat_file,
        mat_ids,
        faces,
    }
}

