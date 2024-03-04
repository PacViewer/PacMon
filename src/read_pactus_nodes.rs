use rusqlite::Connection;
#[derive(Debug)]
pub struct Node {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub website: String,
    pub address: String,
}

pub fn get_pactus_nodes() -> Vec<Node> {
    let mut pactus_nodes: Vec<Node> = vec![];

    let conn = Connection::open("pactus_nodes.db").expect("could not connect");

    let mut stmt = conn
        .prepare("SELECT id, name, email, website, address FROM pactus_nodes")
        .expect("could not connect");
    let nodes = stmt
        .query_map([], |row| {
            Ok(Node {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                website: row.get(3)?,
                address: row.get(4)?,
            })
        })
        .expect("could not connect");
    for node in nodes {
        dbg!(&node);

        let node = node.unwrap();
        pactus_nodes.push(node);
    }

    pactus_nodes
}
