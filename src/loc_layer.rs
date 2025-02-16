// This module implements a workaround for the tracing crate which allows to use <https://doc.rust-lang.org/reference/attributes/codegen.html#r-attributes.codegen.track_caller>
// A tracing::Event (such as created by the event! macro) does hold metadata and fields. The metadata contains level and name.
// The name includes the line. Sadly. tis location information cannot be overwritten. Also you cannot just instantiate a tracing::Event and fire it.
// The workaround adds caller=std::panic::Location::caller() whenever an event is fired via a tracing macro.
// Then custom layers in this modules overwrite the metadata name with the caller field whenever present.
// Probably, the performance characteristics of the tracing crate is responsible for its idiosyncratic design.
// You may be better off not using the tracing crate.

use std::{
    fmt,
    io::{BufWriter, Write},
};

use tracing::Level;
use tracing_subscriber::Layer;
use tz::UtcDateTime;

fn write(buf: &mut BufWriter<Vec<u8>>, args: fmt::Arguments<'_>) {
    if let Err(e) = buf.write_fmt(args) {
        panic!("failed writing to buffer: {e}");
    }
}
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
        let now = UtcDateTime::now().unwrap();
        let level = event.metadata().level().to_string();

        let mut buf = BufWriter::new(Vec::new());

        write(&mut buf, format_args!("{{\"time\":\"{}\"", now));
        write(&mut buf, format_args!(", \"level\":\"{}\"", level));

        let mut buf = if event.fields().any(|field| field.name() == "caller") {
            let mut visitor = MessageVisitor {
                message: None,
                caller: None,
            };
            event.record(&mut visitor);
            let name = visitor.caller.unwrap();
            write(&mut buf, format_args!(", \"name\":\"{}\"", name));

            if let Some(message) = visitor.message {
                write(&mut buf, format_args!(", \"message\":\"{}\"", message));
            }
            buf
        } else {
            write(
                &mut buf,
                format_args!(", \"name\":\"{}\"", event.metadata().name()),
            );
            let mut visitor = JsonVisitor { buf };
            event.record(&mut visitor);
            visitor.buf
        };
        buf.write_all(b"}\n").unwrap();
        if let Err(e) = std::io::stdout()
            .lock()
            .write_all(&buf.into_inner().expect("flushing buffer"))
        {
            panic!("failed writing to stdout: {}", e);
        }
    }
}

struct JsonVisitor {
    buf: BufWriter<Vec<u8>>,
}

impl tracing::field::Visit for JsonVisitor {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        write(
            &mut self.buf,
            format_args!(", \"{}\":{}", field.name(), value),
        );
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        write(
            &mut self.buf,
            format_args!(", \"{}\":{}", field.name(), value),
        );
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        write(
            &mut self.buf,
            format_args!("\"{}\":{}", field.name(), value),
        );
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        write(
            &mut self.buf,
            format_args!(", \"{}\":{}", field.name(), value),
        );
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        write(
            &mut self.buf,
            format_args!(", \"{}\":\"{}\"", field.name(), value),
        );
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        write(
            &mut self.buf,
            format_args!(", \"{}\":\"{}\"", field.name(), value),
        );
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        write(
            &mut self.buf,
            format_args!(", \"{}\":\"{:?}\"", field.name(), value),
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

        let mut buf = BufWriter::new(Vec::new());

        let mut buf = if event.fields().any(|field| field.name() == "caller") {
            let mut visitor = MessageVisitor {
                message: None,
                caller: None,
            };
            event.record(&mut visitor);
            let name = visitor.caller.unwrap();
            if let Some(message) = visitor.message {
                write(
                    &mut buf,
                    format_args!("{} {} {} {}", now, level, name, message),
                );
            } else {
                write(&mut buf, format_args!("{} {} {}", now, level, name));
            }

            buf
        } else {
            let name = event.metadata().name();
            write(&mut buf, format_args!("{} {} {}", now, level, name));
            let mut visitor = PrettyVisitor { buf };
            event.record(&mut visitor);
            visitor.buf
        };

        buf.write_all(b"\n").unwrap();
        if let Err(e) = std::io::stdout()
            .lock()
            .write_all(&buf.into_inner().expect("flushing buffer"))
        {
            panic!("failed writing to stdout: {}", e);
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

struct PrettyVisitor {
    buf: BufWriter<Vec<u8>>,
}

impl tracing::field::Visit for PrettyVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        write(&mut self.buf, format_args!(" {}={};", field.name(), value));
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        write(
            &mut self.buf,
            format_args!(" {}={:?};", field.name(), value),
        );
    }
}
