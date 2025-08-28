pub struct Agent {
    name: String,
    description: String,
    tools: Vec<Tool>,
}

pub struct Tool {
    name: String,
    description: String,
    function: fn(&str) -> String,
}
