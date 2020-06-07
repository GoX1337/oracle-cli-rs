extern crate oracle;
use oracle::Connection;

pub fn exec() {
    let username = "system";
    let password = "oracle";
    let database = "localhost:49161/XE";
    let sql = "select sysdate from dual";

    let conn = Connection::connect(username, password, database).unwrap();
    let mut stmt = conn.prepare(sql, &[]).unwrap();
    let rows = stmt.query(&[]).unwrap();

    // print column types
    for (idx, info) in rows.column_info().iter().enumerate() {
        if idx != 0 {
            print!(",");
        }
        print!("{}", info);
    }
    println!("");

    for row_result in rows {
        // print column values
        for (idx, val) in row_result.unwrap().sql_values().iter().enumerate() {
            if idx != 0 {
                print!(",");
            }
            print!("{}", val);
        }
        println!("");
    }
}
