use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf>  {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut value = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                value = &sub_str[i+1..];
            }

            data.entry(key)
            .and_modify(|current_value| {
                match current_value {
                    Value::Single(prev_value) => {

                        // first solution
                        // let mut vec = Vec::new();
                        // vec.push(value);
                        // vec.push(prev_value);
                        // ...

                        // better solution
                        *current_value = Value::Multiple(vec![value,prev_value]);


                    },
                    Value::Multiple(vec) => vec.push(value)
                }
            })
            .or_insert(Value::Single(value));
        }

        QueryString { data }
    }
}