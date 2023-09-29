use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};
use serde_json::{json, Value};

use super::{helpers, WampMessage};

#[derive(Debug)]
pub struct Call {
    pub request_id: u64,
    pub options: Value,
    pub procedure: String,
    pub args: Value,
    pub kwargs: Value
}

impl WampMessage for Call {
    const ID: u64 = 48;
}

impl Serialize for Call {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let options = helpers::ser_value_is_object::<S, _>(&self.options, "Options must be object like.")?;
        let args = helpers::ser_value_is_args::<S, _>(&self.args, "Args must be Array like or Null.")?;
        let kwargs = helpers::ser_value_is_kwargs::<S, _>(&self.kwargs, "Kwargs must be Object like or Null.")?;
        if args.is_null() {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, options, &self.procedure).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, options, &self.procedure, json!([]), kwargs).serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, options, &self.procedure, args).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, options, &self.procedure, args, kwargs).serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Call {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct CallVisitor(PhantomData<u8>, PhantomData<u64>, PhantomData<Value>, PhantomData<String>, PhantomData<Value>, PhantomData<Value>);
        
        impl<'vi> Visitor<'vi> for CallVisitor {
            type Value = Call;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Call components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Call, A, _>(&message_id, "Call")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "Request ID must be present and type u64.")?;
                let options: Value = helpers::deser_seq_element(&mut seq, "Options must be present and object like.")?;
                helpers::deser_value_is_object::<A, _>(&options, "Options must be object like.")?;
                let procedure: String = helpers::deser_seq_element(&mut seq, "Procedure must be present and object like.")?;
                let args: Value = helpers::deser_seq_element(&mut seq, "Args must be array like or null.")?;
                let kwargs: Value = helpers::deser_seq_element(&mut seq, "Kwargs must be object like or null.")?;
                Ok(Call {
                    request_id,
                    options,
                    procedure,
                    args,
                    kwargs
                })
            }
        }

        deserializer.deserialize_struct("Call", &["request_id", "message_id", "options", "procedure", "args", "kwargs"], CallVisitor(PhantomData, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, json, to_string};

    use super::*;

    #[test]
    fn raw_str() {
        let data = r#"[48,7814135,{},"com.myapp.user.new",["johnny"],{"firstname":"John","surname":"Doe"}]"#;
        let a: Call = from_str(data).unwrap();
        println!("{:#?}", a);
        //assert_eq!(a.request_id, "wamp.error.no_such_realm");
    }

    #[test]
    fn obj_to_str() {
        let a = Call {
            request_id: 7814135,
            options: json!({}),
            procedure: "com.myapp.user.new".to_string(),
            args: json!(["johnny"]),
            kwargs: json!({"firstname":"John","surname":"Doe"})
        };
        let data = r#"[48,7814135,{},"com.myapp.user.new",["johnny"],{"firstname":"John","surname":"Doe"}]"#;
        let an = to_string(&a).unwrap();
        assert_eq!(data, an)
    }
}