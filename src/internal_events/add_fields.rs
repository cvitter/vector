use super::InternalEvent;
use metrics::counter;

#[derive(Debug)]
pub struct AddFieldsEventProcessed;

impl InternalEvent for AddFieldsEventProcessed {
    fn emit_metrics(&self) {
        counter!("processed_events_total", 1);
    }

    fn emit_metrics_wrapped(&self) {}
}

#[derive(Debug)]
pub struct AddFieldsTemplateRenderingError<'a> {
    pub field: &'a str,
}

impl<'a> InternalEvent for AddFieldsTemplateRenderingError<'a> {
    fn emit_logs(&self) {
        error!(message = "Failed to render templated value; discarding value.", field = %self.field, rate_limit_secs = 30);
    }

    fn emit_metrics(&self) {
        counter!("processing_errors_total", 1);
    }
}

#[derive(Debug)]
pub struct AddFieldsTemplateInvalid<'a> {
    pub error: crate::template::TemplateError,
    pub field: &'a str,
}

impl<'a> InternalEvent for AddFieldsTemplateInvalid<'a> {
    fn emit_logs(&self) {
        error!(message = "Invalid template; using as string.", field = %self.field, error = ?self.error, rate_limit_secs = 30);
    }

    fn emit_metrics(&self) {
        counter!("processing_errors_total", 1);
    }
}

#[derive(Debug)]
pub struct AddFieldsFieldOverwritten<'a> {
    pub field: &'a str,
}

impl<'a> InternalEvent for AddFieldsFieldOverwritten<'a> {
    fn emit_logs(&self) {
        debug!(message = "Field overwritten.", field = %self.field, rate_limit_secs = 30);
    }
}

#[derive(Debug)]
pub struct AddFieldsFieldNotOverwritten<'a> {
    pub field: &'a str,
}

impl<'a> InternalEvent for AddFieldsFieldNotOverwritten<'a> {
    fn emit_logs(&self) {
        debug!(message = "Field not overwritten.", field = %self.field, rate_limit_secs = 30);
    }
}
