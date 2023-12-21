pub mod tool_errors {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    pub enum ToolError {
        EmptyCoordinates,
        Other(String),
    }

    impl Debug for ToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    impl Display for ToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return match self {
                ToolError::EmptyCoordinates => write!(f,"{}","Empty Coordinates".to_string()),
                ToolError::Other(message) => write!(f, "{}", message),
            };
        }
    }

    impl Error for ToolError {}
}
