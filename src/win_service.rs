use crate::config::CliConfig;
use clap::Parser;
use color_eyre::eyre::{self, Context};
use log::error;
use std::{ffi::OsString, time::Duration};
use tokio::sync::Notify;
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};

/// Notified when the service control manager asks the service to stop.
pub(crate) static SHUTDOWN: Notify = Notify::const_new();

define_windows_service!(ffi_service_main, service_main);

pub(crate) fn run() -> eyre::Result<()> {
    // The name is ignored for services that run in their own process.
    service_dispatcher::start("spotifyd", ffi_service_main)
        .wrap_err("failed to connect to the service control manager")
}

fn service_main(_arguments: Vec<OsString>) {
    let status_handle = match service_control_handler::register("spotifyd", |control| match control
    {
        ServiceControl::Stop | ServiceControl::Shutdown => {
            SHUTDOWN.notify_one();
            ServiceControlHandlerResult::NoError
        }
        ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
        _ => ServiceControlHandlerResult::NotImplemented,
    }) {
        Ok(handle) => handle,
        Err(_) => return,
    };

    let set_status = |state, controls_accepted, exit_code| {
        let _ = status_handle.set_service_status(ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: state,
            controls_accepted,
            exit_code,
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        });
    };

    set_status(
        ServiceState::Running,
        ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
        ServiceExitCode::Win32(0),
    );

    // The service arguments come from the command line registered with the service.
    let mut cli_config = CliConfig::parse();
    cli_config.no_daemon = true;

    let exit_code = match crate::run_daemon(cli_config) {
        Ok(()) => ServiceExitCode::Win32(0),
        Err(err) => {
            error!("{err:?}");
            ServiceExitCode::ServiceSpecific(1)
        }
    };

    set_status(
        ServiceState::Stopped,
        ServiceControlAccept::empty(),
        exit_code,
    );
}
