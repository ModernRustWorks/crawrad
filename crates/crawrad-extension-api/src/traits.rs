use crate::error::ExtensionError;
use crate::services::{DesignerService, EditorService, ProjectService};

/// Notification severity level shown in the IDE chrome.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::MessageLevel;
///
/// let level = MessageLevel::Info;
/// assert_eq!(level, MessageLevel::Info);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageLevel {
    Info,
    Warning,
    Error,
}

/// Dynamic callback for executing a contributed command.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::CommandHandler;
///
/// let handler: CommandHandler = Box::new(|| {});
/// handler();
/// ```
pub type CommandHandler = Box<dyn Fn() + Send + Sync + 'static>;

/// Host context provided by the IDE to an active extension.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{CommandHandler, ExtensionContext, ExtensionError, MessageLevel};
///
/// struct DummyCtx;
/// impl ExtensionContext for DummyCtx {
///     fn register_command(&mut self, _: &str, _: CommandHandler) -> Result<(), ExtensionError> { Ok(()) }
///     fn show_message(&self, _: MessageLevel, _: &str) {}
///     fn active_workspace_path(&self) -> Option<String> { None }
/// }
///
/// let ctx = DummyCtx;
/// assert!(ctx.active_workspace_path().is_none());
/// ```
pub trait ExtensionContext: Send + Sync {
    /// Register an executable handler for an action ID declared in the manifest.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{CommandHandler, ExtensionContext, ExtensionError, MessageLevel};
    ///
    /// struct DummyCtx;
    /// impl ExtensionContext for DummyCtx {
    ///     fn register_command(&mut self, _: &str, _: CommandHandler) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn show_message(&self, _: MessageLevel, _: &str) {}
    ///     fn active_workspace_path(&self) -> Option<String> { None }
    /// }
    ///
    /// let mut ctx = DummyCtx;
    /// assert!(ctx.register_command("ext.cmd", Box::new(|| {})).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if the command registration fails.
    fn register_command(
        &mut self,
        action_id: &str,
        handler: CommandHandler,
    ) -> Result<(), ExtensionError>;

    /// Display a message notification in the IDE status bar or toast area.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{CommandHandler, ExtensionContext, ExtensionError, MessageLevel};
    ///
    /// struct DummyCtx;
    /// impl ExtensionContext for DummyCtx {
    ///     fn register_command(&mut self, _: &str, _: CommandHandler) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn show_message(&self, _: MessageLevel, _: &str) {}
    ///     fn active_workspace_path(&self) -> Option<String> { None }
    /// }
    ///
    /// let ctx = DummyCtx;
    /// ctx.show_message(MessageLevel::Info, "Ready");
    /// ```
    fn show_message(&self, level: MessageLevel, text: &str);

    /// Get the current root workspace path if one is open.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{CommandHandler, ExtensionContext, ExtensionError, MessageLevel};
    ///
    /// struct DummyCtx;
    /// impl ExtensionContext for DummyCtx {
    ///     fn register_command(&mut self, _: &str, _: CommandHandler) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn show_message(&self, _: MessageLevel, _: &str) {}
    ///     fn active_workspace_path(&self) -> Option<String> { Some("/ws".into()) }
    /// }
    ///
    /// let ctx = DummyCtx;
    /// assert_eq!(ctx.active_workspace_path().as_deref(), Some("/ws"));
    /// ```
    fn active_workspace_path(&self) -> Option<String>;

    /// Access code editor services (analogous to `IOTAEditorServices`).
    fn editor(&self) -> Option<&dyn EditorService> {
        None
    }

    /// Access mutable code editor services.
    fn editor_mut(&mut self) -> Option<&mut dyn EditorService> {
        None
    }

    /// Access project and module management services.
    ///
    /// Access project and module management services (analogous to `IOTAProjectServices`).
    fn project(&self) -> Option<&dyn ProjectService> {
        None
    }

    /// Access mutable project services.
    fn project_mut(&mut self) -> Option<&mut dyn ProjectService> {
        None
    }

    /// Access visual form designer services (analogous to `IOTAFormEditorServices`).
    fn designer(&self) -> Option<&dyn DesignerService> {
        None
    }

    /// Access mutable visual form designer services.
    fn designer_mut(&mut self) -> Option<&mut dyn DesignerService> {
        None
    }
}

/// The core trait implemented by compiled Rust extension plugins.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::{ExtensionContext, ExtensionError, IdeExtension};
///
/// struct MyExtension;
/// impl IdeExtension for MyExtension {
///     fn id(&self) -> &str { "my_ext" }
///     fn activate(&mut self, _: &mut dyn ExtensionContext) -> Result<(), ExtensionError> { Ok(()) }
/// }
///
/// let ext = MyExtension;
/// assert_eq!(ext.id(), "my_ext");
/// ```
pub trait IdeExtension: Send + Sync {
    /// Unique identifier matching the manifest ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{ExtensionContext, ExtensionError, IdeExtension};
    ///
    /// struct MyExtension;
    /// impl IdeExtension for MyExtension {
    ///     fn id(&self) -> &str { "my_ext" }
    ///     fn activate(&mut self, _: &mut dyn ExtensionContext) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let ext = MyExtension;
    /// assert_eq!(ext.id(), "my_ext");
    /// ```
    fn id(&self) -> &str;

    /// Invoked by the IDE when the extension is loaded and activated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crawrad_extension_api::{CommandHandler, ExtensionContext, ExtensionError, IdeExtension, MessageLevel};
    ///
    /// struct DummyCtx;
    /// impl ExtensionContext for DummyCtx {
    ///     fn register_command(&mut self, _: &str, _: CommandHandler) -> Result<(), ExtensionError> { Ok(()) }
    ///     fn show_message(&self, _: MessageLevel, _: &str) {}
    ///     fn active_workspace_path(&self) -> Option<String> { None }
    /// }
    ///
    /// struct MyExtension;
    /// impl IdeExtension for MyExtension {
    ///     fn id(&self) -> &str { "my_ext" }
    ///     fn activate(&mut self, _: &mut dyn ExtensionContext) -> Result<(), ExtensionError> { Ok(()) }
    /// }
    ///
    /// let mut ext = MyExtension;
    /// let mut ctx = DummyCtx;
    /// assert!(ext.activate(&mut ctx).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionError`] if initialization or command registration fails.
    fn activate(&mut self, ctx: &mut dyn ExtensionContext) -> Result<(), ExtensionError>;

    /// Invoked when the extension is unloaded or the IDE shuts down.
    fn deactivate(&mut self) -> Result<(), ExtensionError> {
        Ok(())
    }
}
