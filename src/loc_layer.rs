// Source: https://github.com/bryanburgers/tracing-blog-post/blob/main/examples/figure_3/custom_layer.rs

use std::collections::BTreeMap;
use tracing::Level;
use tracing_subscriber::Layer;
use tz::UtcDateTime;

pub struct JsonLayer;

impl<S> Layer<S> for JsonLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Covert the values into a JSON object
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        let name = if let Some(caller) = fields.remove("caller") {
            caller.as_str().unwrap().to_owned()
        } else {
            event.metadata().name().to_owned()
        };

        let level = event.metadata().level();
        let output = serde_json::json!({
            //"target": event.metadata().target(),
            "name": name,
            "level": level.as_str(),
            "fields": fields,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }
}

struct JsonVisitor<'a>(&'a mut BTreeMap<String, serde_json::Value>);

impl<'a> tracing::field::Visit for JsonVisitor<'a> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(value.to_string()),
        );
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(format!("{:?}", value)),
        );
    }
}

pub struct PrettyLayer;

impl<S> Layer<S> for PrettyLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let now = UtcDateTime::now().unwrap();
        let level = event.metadata().level();
        let level = if *level == Level::ERROR {
            format!("\x1b[91m{}\x1b[0m", level)
        } else {
            format!("\x1b[92m{:^5}\x1b[0m", level)
        };

        if event.fields().any(|field| field.name() == "caller") {
            let mut visitor = MessageVisitor {
                message: None,
                caller: None,
            };
            event.record(&mut visitor);
            let name = visitor.caller.unwrap();
            if let Some(message) = visitor.message {
                println!("{} {} {} {}", now, level, name, message);
            } else {
                println!("{} {} {}", now, level, name);
            }
        } else {
            let name = event.metadata().name();
            print!("{} {} {}", now, level, name);
            let mut visitor = PrettyVisitor();
            event.record(&mut visitor);
            println!("");
        }
    }
}

struct MessageVisitor {
    message: Option<String>,
    caller: Option<String>,
}

impl tracing::field::Visit for MessageVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = Some(value.to_owned());
        } else if field.name() == "caller" {
            self.caller = Some(value.to_owned());
        }
    }

    fn record_debug(&mut self, _field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {}
}

struct PrettyVisitor();

impl tracing::field::Visit for PrettyVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        print!(" {}={};", field.name(), value);
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        print!(" {}={:?};", field.name(), value);
    }
}
