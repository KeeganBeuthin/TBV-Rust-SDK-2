pub fn generate_balance_query(account: &str) -> String {
    format!(
        "PREFIX ex: <http://example.org/>
        SELECT ?balance
        WHERE {{
          ex:{} ex:hasBalance ?balance .
        }}",
        account
    )
}