use crate::{
	Array, ArrayBuffer, BigInt, Boolean, Buffer, DataView, Date, Env, Error, External, Function,
	Null, Number, Object, Result, String, Symbol, TypedArray, Undefined, Value,
};
use num::{FromPrimitive, ToPrimitive};

#[allow(clippy::wrong_self_convention, clippy::upper_case_acronyms)]
pub trait IntoNodeApi<'a>: 'a {
	fn into_node_api(self, env: Env<'a>) -> Result<Value<'a>>;
}

#[allow(clippy::wrong_self_convention, clippy::upper_case_acronyms)]
pub trait FromNodeAPI<'a>: 'a + Sized {
	fn from_node_api(value: Value<'a>) -> Result<Self>;
}

impl<'a> IntoNodeApi<'a> for () {
	fn into_node_api(self, env: Env<'a>) -> Result<Value<'a>> {
		Ok(Null::new(env)?.value())
	}
}

impl<'a> FromNodeAPI<'a> for () {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_null()?;
		Ok(())
	}
}

impl<'a> IntoNodeApi<'a> for bool {
	fn into_node_api(self, env: Env<'a>) -> Result<Value<'a>> {
		Ok(Boolean::new(env, self)?.value())
	}
}

impl<'a> FromNodeAPI<'a> for bool {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		let value = value.as_boolean()?;
		let value = value.get()?;
		Ok(value)
	}
}

macro_rules! impl_to_from_for_number_type {
	($ty:ty) => {
		impl<'a> IntoNodeApi<'a> for $ty {
			fn into_node_api(self, env: Env<'a>) -> Result<Value<'a>> {
				let value =
					<$ty>::to_f64(&self).ok_or_else(|| Error::message("number out of bounds"))?;
				let number = Number::new(env, value)?;
				Ok(number.value())
			}
		}
		impl<'a> FromNodeAPI<'a> for $ty {
			fn from_node_api(value: Value<'a>) -> Result<Self> {
				let number = value.as_number()?;
				let value = number.get()?;
				let value =
					<$ty>::from_f64(value).ok_or_else(|| Error::message("number out of bounds"))?;
				Ok(value)
			}
		}
	};
}

impl_to_from_for_number_type!(usize);
impl_to_from_for_number_type!(u8);
impl_to_from_for_number_type!(u16);
impl_to_from_for_number_type!(u32);
impl_to_from_for_number_type!(u64);
impl_to_from_for_number_type!(isize);
impl_to_from_for_number_type!(i8);
impl_to_from_for_number_type!(i16);
impl_to_from_for_number_type!(i32);
impl_to_from_for_number_type!(i64);
impl_to_from_for_number_type!(f32);
impl_to_from_for_number_type!(f64);

impl<'a> IntoNodeApi<'a> for char {
	fn into_node_api(self, env: Env<'a>) -> Result<Value<'a>> {
		Ok(String::new(env, &self.to_string())?.value())
	}
}

impl<'a> FromNodeAPI<'a> for char {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		Ok(value.as_string()?.get()?.chars().next().unwrap())
	}
}

impl<'a> IntoNodeApi<'a> for &'a str {
	fn into_node_api(self, env: Env<'a>) -> Result<Value<'a>> {
		Ok(String::new(env, self)?.value())
	}
}

impl<'a> IntoNodeApi<'a> for std::string::String {
	fn into_node_api(self, env: Env<'a>) -> Result<Value<'a>> {
		Ok(String::new(env, self.as_str())?.value())
	}
}

impl<'a> FromNodeAPI<'a> for std::string::String {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_string()?.get()
	}
}

impl<'a, T> IntoNodeApi<'a> for Option<T>
where
	T: IntoNodeApi<'a>,
{
	fn into_node_api(self, env: Env<'a>) -> Result<Value<'a>> {
		match self {
			None => Ok(Null::new(env)?.value()),
			Some(value) => Ok(value.into_node_api(env)?),
		}
	}
}

impl<'a, T> FromNodeAPI<'a> for Option<T>
where
	T: FromNodeAPI<'a>,
{
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		if value.as_null().is_ok() || value.as_undefined().is_ok() {
			Ok(None)
		} else {
			Ok(Some(T::from_node_api(value)?))
		}
	}
}

impl<'a, T> IntoNodeApi<'a> for Vec<T>
where
	T: IntoNodeApi<'a>,
{
	fn into_node_api(self, env: Env<'a>) -> Result<Value<'a>> {
		let mut array = Array::new(env)?;
		for (i, value) in self.into_iter().enumerate() {
			array.set(i, value.into_node_api(env)?)?;
		}
		Ok(array.value())
	}
}

impl<'a, T> FromNodeAPI<'a> for Vec<T>
where
	T: FromNodeAPI<'a>,
{
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		let value = value.as_array()?;
		let mut vec = Vec::with_capacity(value.size()?);
		for value in value.iter()? {
			vec.push(T::from_node_api(value?)?);
		}
		Ok(vec)
	}
}

impl<'a> IntoNodeApi<'a> for Value<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self)
	}
}

impl<'a> FromNodeAPI<'a> for Value<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		Ok(value)
	}
}

impl<'a> IntoNodeApi<'a> for Array<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for Array<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_array()
	}
}

impl<'a> IntoNodeApi<'a> for ArrayBuffer<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for ArrayBuffer<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_arraybuffer()
	}
}

impl<'a> IntoNodeApi<'a> for BigInt<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for BigInt<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_bigint()
	}
}

impl<'a> IntoNodeApi<'a> for Boolean<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for Boolean<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_boolean()
	}
}

impl<'a> IntoNodeApi<'a> for Buffer<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for Buffer<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_buffer()
	}
}

impl<'a> IntoNodeApi<'a> for DataView<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a, 'b: 'a> FromNodeAPI<'a> for DataView<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_dataview()
	}
}

impl<'a> IntoNodeApi<'a> for Date<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for Date<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_date()
	}
}

impl<'a, T: 'a> IntoNodeApi<'a> for External<'a, T> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a, T: 'a> FromNodeAPI<'a> for External<'a, T> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_external()
	}
}

impl<'a> IntoNodeApi<'a> for Function<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for Function<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_function()
	}
}

impl<'a> IntoNodeApi<'a> for Null<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for Null<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_null()
	}
}

impl<'a> IntoNodeApi<'a> for Number<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for Number<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_number()
	}
}

impl<'a> IntoNodeApi<'a> for Object<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for Object<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_object()
	}
}

impl<'a> IntoNodeApi<'a> for String<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for String<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		let value = value.as_string()?;
		Ok(value)
	}
}

impl<'a> IntoNodeApi<'a> for Symbol<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for Symbol<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_symbol()
	}
}

impl<'a> IntoNodeApi<'a> for TypedArray<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for TypedArray<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_typedarray()
	}
}

impl<'a> IntoNodeApi<'a> for Undefined<'a> {
	fn into_node_api(self, _env: Env<'a>) -> Result<Value<'a>> {
		Ok(self.value())
	}
}

impl<'a> FromNodeAPI<'a> for Undefined<'a> {
	fn from_node_api(value: Value<'a>) -> Result<Self> {
		value.as_undefined()
	}
}
