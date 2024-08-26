use elasticsearch::Elasticsearch;
use elasticsearch::http::transport::Transport;

pub fn es_client() -> Elasticsearch {
    let transport = Transport::single_node("http://127.0.0.1:9200").unwrap();
    Elasticsearch::new(transport)
}
