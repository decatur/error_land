use std::fmt;

use tracing::{Event, Subscriber};
use tracing_subscriber::{
    fmt::{format, FmtContext, FormatEvent, FormatFields},
    registry::LookupSpan,
};
use tz::UtcDateTime;

use crate::Thing;

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
        let now = UtcDateTime::now().unwrap();
        // Format values from the event's's metadata:
        let metadata = event.metadata();
        write!(&mut writer, "{} {}: ", now, metadata.level())?;

        let mut visitor = PrettyVisitor {
            writer,
            msg: None,
            errors: vec![],
        };
        event.record(&mut visitor);
        let mut writer = visitor.writer;
        if let Some(msg) = visitor.msg {
            writer.write_str(&format!("\n    {}", msg))?;
        }

        for error in visitor.errors {
            writer.write_str(&format!("\n    {}", error))?;
        }
        write!(
            &mut writer,
            "\n    {}:{}",
            metadata.file().unwrap_or("None"),
            metadata.line().unwrap_or(0)
        )?;

        writeln!(writer)
    }
}

struct PrettyVisitor<'a> {
    writer: format::Writer<'a>,
    errors: Vec<String>,
    msg: Option<String>,
}

impl tracing::field::Visit for PrettyVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        self.writer
            .write_str(&format!("{}={:?}; ", field.name(), value))
            .unwrap();
    }

    fn record_error(
        &mut self,
        _field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        let r = value.downcast_ref::<Thing>();
        if let Some(r) = r {
            self.errors = r.inner.clone();
            self.msg = Some(r.msg.clone());
        } else {
            self.msg = Some(format!("{}", value));
        }
    }
}
