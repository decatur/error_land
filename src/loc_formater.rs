use std::fmt;

use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{
    fmt::{format, FmtContext, FormatEvent, FormatFields},
    registry::LookupSpan,
};
use tz::UtcDateTime;

use crate::{loc_error::StackItem, CoreError};

pub struct PrettyFormatter;

impl<S, N> FormatEvent<S, N> for PrettyFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let now = &UtcDateTime::now().unwrap().to_string()[0..19];
        let metadata = event.metadata();
        let level = metadata.level();
        let level = if *level == Level::ERROR {
            format!("\x1b[91m{}\x1b[0m", level)
        } else if *level == Level::WARN {
            format!("\x1b[93m{}\x1b[0m", level)
        } else {
            format!("\x1b[92m{:^5}\x1b[0m", level)
        };

        write!(&mut writer, "{}Z {}: ", now, level)?;

        let mut visitor = PrettyVisitor {
            writer,
            msg: None,
            errors: vec![],
        };
        event.record(&mut visitor);
        let mut writer = visitor.writer;
        // if let Some(msg) = visitor.msg {
        //     writer.write_str(&format!("\n    {}", msg))?;
        // }

        for error in visitor.errors {
            writer.write_str(&format!("\n    {}", error))?;
        }
        write!(
            &mut writer,
            "\n    {}:{} {}",
            metadata.file().unwrap_or("None"),
            metadata.line().unwrap_or(0),
            visitor.msg.unwrap_or("None".to_owned()),
        )?;

        writeln!(writer)
    }
}

struct PrettyVisitor<'a> {
    writer: format::Writer<'a>,
    errors: Vec<StackItem>,
    msg: Option<String>,
}

impl tracing::field::Visit for PrettyVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.msg = Some(format!("{:?}", value));
        } else {
        self.writer
            .write_str(&format!("{}={:?}; ", field.name(), value))
            .unwrap();
        }
    }

    fn record_error(
        &mut self,
        _field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        
        let r = value.downcast_ref::<CoreError>();
        if let Some(r) = r {
            self.errors = r.inner.clone();
            //self.msg = Some(r.msg.clone());
        } else {
            self.msg = Some(format!("Rumpelstilzchen {}", value));
        }
    }
}

pub struct JsonFormatter;

impl<S, N> FormatEvent<S, N> for JsonFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let now = &UtcDateTime::now().unwrap().to_string()[0..19];
        let metadata = event.metadata();
        let level = metadata.level();

        write!(
            &mut writer,
            "{{\"timestamp\":\"{}Z\", \"level\":\"{}\"",
            now, level
        )?;

        let mut visitor = JsonVisitor {
            writer,
            msg: None,
            errors: vec![],
        };
        event.record(&mut visitor);

        let mut writer = visitor.writer;
        if let Some(msg) = visitor.msg {
            writer.write_str(&format!(", \"msg\":\"{}\"", msg))?;
        }

        writer.write_str(", \"stack\":[")?;
        for error in visitor.errors {
            writer.write_str(&format!("\"{}\",", error))?;
        }
        write!(
            &mut writer,
            "\"{}:{}\"]}}",
            metadata.file().unwrap_or("None"),
            metadata.line().unwrap_or(0)
        )?;

        writeln!(writer)
    }
}

struct JsonVisitor<'a> {
    writer: format::Writer<'a>,
    errors: Vec<StackItem>,
    msg: Option<String>,
}

impl tracing::field::Visit for JsonVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        let s = format!("{:?}", value); //.replace("\"", "\\\"");
        if s.starts_with("{") && s.ends_with("}") {
            self.writer
                .write_str(&format!(", \"{}\":{s}", field.name()))
                .unwrap();
        } else {
            self.writer
                .write_str(&format!(", \"{}\":\"{s}\"", field.name()))
                .unwrap();
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.writer
            .write_str(&format!(
                ", \"{}\":\"{}\"",
                field.name(),
                value.replace("\"", "\\\"")
            ))
            .unwrap();
    }

    fn record_error(
        &mut self,
        _field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        assert!(self.msg.is_none());
        let r = value.downcast_ref::<CoreError>();
        if let Some(r) = r {
            self.errors = r.inner.clone();
            self.msg = Some(r.msg.clone());
        } else {
            self.msg = Some(format!("{}", value));
        }
    }
}
