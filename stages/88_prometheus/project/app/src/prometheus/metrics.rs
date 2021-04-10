use prometheus::{ HistogramOpts, HistogramVec, IntCounterVec, Opts, Registry };




lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    pub static ref INCOMING_REQUESTS: IntCounterVec = IntCounterVec::new(Opts::new("incoming_requests", "Incoming Requests"), &["method", "path"])
        .expect("metric can be created");

    pub static ref RESPONSE_TIME_COLLECTOR: HistogramVec = HistogramVec::new(HistogramOpts::new("response_time", "Response Times"), &["method", "path"])
        .expect("metric can be created");

    pub static ref RESPONSE_CODE_COLLECTOR: IntCounterVec = IntCounterVec::new(Opts::new("response_code", "Response Codes"), &["method", "path", "code"])
        .expect("metric can be created");
}



pub fn register_custom_metrics() {
    REGISTRY.register(Box::new(INCOMING_REQUESTS.clone()))
        .expect("collector can be registered");

    REGISTRY.register(Box::new(RESPONSE_CODE_COLLECTOR.clone()))
        .expect("collector can be registered");

    REGISTRY.register(Box::new(RESPONSE_TIME_COLLECTOR.clone()))
        .expect("collector can be registered");
}
