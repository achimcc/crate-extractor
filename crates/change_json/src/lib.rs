mod change_json;
mod crate_graph_json;

pub use change_json::ChangeJson;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}