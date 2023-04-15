pub enum Stage {
    Production,
    Integration,
    Dev,
}

pub struct SdkConfig {
    pub stage: Stage,
}
