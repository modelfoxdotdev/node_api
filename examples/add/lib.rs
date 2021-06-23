node_api::init!(init);

fn init<'a>(
	env: node_api::Env<'a>,
	exports: node_api::Value<'a>,
) -> node_api::Result<node_api::Value<'a>> {
	let mut exports = exports.as_object()?;
	let key = node_api::String::new(env, "add")?;
	let value = node_api::Function::new(env, "add", add)?;
	exports.set(key, value)?;
	Ok(exports.value())
}

#[node_api::function]
fn add(env: node_api::Env, a: u64, b: u64) -> node_api::Result<u64> {
	Ok(a + b)
}
