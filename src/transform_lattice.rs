use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

fn from_str(x: &str, y: &str, z: &str) -> Point {
    Point { x: x.parse().unwrap(), y: y.parse().unwrap(), z: z.parse().unwrap() }
}

fn subtract(p1: &Point, p2: &Point) -> Point {
    Point { x: p1.x - p2.x, y: p1.y - p2.y, z: p1.z - p2.z}
}

fn square_norm(p1: &Point, p2: &Point) -> f64 {
    (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)
}

fn normalize(p: &Point, norm: f64) -> Point {
    Point { x: p.x / norm, y: p.y / norm, z: p.z / norm }
}

fn parse_xml(data: &str) {
    let mut reader = Reader::from_str(data);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut is_atom_record = false;
    let mut residues: Vec<Point> = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"PDBx:atom_record" => {
                        is_atom_record = true
                    }
                    _ => (),
                }
            }
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => {
                if is_atom_record {
                    let x = e.unescape_and_decode(&reader).unwrap();
                    let cols: Vec<&str> = x.split_whitespace().collect();

                    let atom = cols[11];
                    if atom == "CA" {
                        let p = from_str(cols[13], cols[14], cols[15]);
                        residues.push(p);
                    }
                    is_atom_record = false;
                }
            }
            Ok(Event::Eof) => break,
            _ => (),
        }
    }

    let mut p0: &Point = &residues[0];
    let mut arrows: Vec<Point> = Vec::new();
    let mut square_norm_total = 0.0;
    for (i, residue) in residues.iter().enumerate() {
        if i == 0 {
            p0 = residue;
            continue;
        }

        let norm = square_norm(p0, residue);
        println!("{}", &norm);
        square_norm_total += norm;
        p0 = residue;
    }

    let square_norm_avg: f64 = square_norm_total / residues.len() as f64;
    let norm_avg = square_norm_avg.sqrt();
    for residue in residues.iter() {
        let p = normalize(residue, norm_avg);
        dbg!(p);
    }
}

pub async fn fetch_protein_structure() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://data.pdbjbk1.pdbj.org/pub/pdb/data/structures/divided/XML-extatom/sh/1shg-extatom.xml")
        .await?;
    parse_xml(&res.text().await?);

    Ok(())
}